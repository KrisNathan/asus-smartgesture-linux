#!/usr/bin/env python3
import asyncio
import evdev
import json
import os
import subprocess
import sys
from pathlib import Path

# Setup logging manually or use printing for simple daemon
DEBUG = os.environ.get("DEBUG", "0") == "1"

def log(*args):
    if DEBUG:
        print(*args, file=sys.stderr)

# Configuration defaults
CONFIG_FILE = Path.home() / ".config" / "asus-touchpad-gesture" / "config.json"
DEFAULT_CONFIG = {
    "left_edge_threshold_percent": 0.10,
    "right_edge_threshold_percent": 0.90,
    "sensitivity_y": 0.20,
    "invert_y": False
}

def load_config():
    config = DEFAULT_CONFIG.copy()
    if CONFIG_FILE.exists():
        try:
            with open(CONFIG_FILE, 'r') as f:
                user_config = json.load(f)
                config.update(user_config)
            log(f"Loaded config from {CONFIG_FILE}")
        except Exception as e:
            print(f"Error loading config: {e}", file=sys.stderr)
    else:
        log("Using default configuration")
    return config

def get_touchpad_device():
    """Finds the first touchpad device."""
    devices = [evdev.InputDevice(path) for path in evdev.list_devices()]
    for device in devices:
        # Checking for absolute axes and touchpad capabilities
        capabilities = device.capabilities()
        if evdev.ecodes.EV_ABS in capabilities and evdev.ecodes.EV_KEY in capabilities:
            # Check for typical touchpad keys
            if evdev.ecodes.BTN_TOUCH in capabilities[evdev.ecodes.EV_KEY]:
                log(f"Found touchpad: {device.name} at {device.path}")
                return device
    raise Exception("No touchpad device found")

class TouchpadGestureContext:
    def __init__(self, device, config):
        self.device = device
        self.config = config
        
        # Pre-compute command prefix for dropping privileges dynamically
        self.user_env_prefix = []
        sudo_user = os.environ.get("SUDO_USER")
        if os.geteuid() == 0 and sudo_user:
            uid = subprocess.check_output(["id", "-u", sudo_user]).decode().strip()
            self.user_env_prefix = [
                "sudo", "-u", sudo_user, 
                f"DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/{uid}/bus",
                f"XDG_RUNTIME_DIR=/run/user/{uid}"
            ]
            
        # Pre-compute max brightness since it never changes
        self.max_brightness = 100.0
        try:
            cmd_max = ["qdbus", "org.kde.Solid.PowerManagement", "/org/kde/Solid/PowerManagement/Actions/BrightnessControl", "org.kde.Solid.PowerManagement.Actions.BrightnessControl.brightnessMax"]
            max_raw = subprocess.check_output(self.user_env_prefix + cmd_max, stderr=subprocess.DEVNULL).decode().strip()
            if max_raw:
                self.max_brightness = float(max_raw)
        except Exception as e:
            log(f"Warning: Could not fetch max brightness via DBus. Error: {e}")
        
        # Touchpad dimensions
        abs_caps = dict(device.capabilities().get(evdev.ecodes.EV_ABS, []))
        
        absinfo_x = abs_caps.get(evdev.ecodes.ABS_MT_POSITION_X)
        if not absinfo_x:
            absinfo_x = abs_caps.get(evdev.ecodes.ABS_X)
            
        absinfo_y = abs_caps.get(evdev.ecodes.ABS_MT_POSITION_Y)
        if not absinfo_y:
            absinfo_y = abs_caps.get(evdev.ecodes.ABS_Y)
            
        if not absinfo_x or not absinfo_y:
            raise Exception("Touchpad does not report absolute X/Y positions")
        
        self.min_x = absinfo_x.min
        self.max_x = absinfo_x.max
        self.min_y = absinfo_y.min
        self.max_y = absinfo_y.max
        
        self.width = self.max_x - self.min_x
        self.height = self.max_y - self.min_y
        
        # State tracking
        self.active_touches = {} # slot -> {"x": val, "y": val, "start_x": val, "start_y": val, "action": mode}
        self.current_slot = 0
        
        # Action accumulation
        self.accumulated_delta_vol = 0.0
        self.accumulated_delta_bri = 0.0

    def get_action_mode(self, x):
        """Determine if X is on the left or right edge."""
        percent_x = (x - self.min_x) / self.width if self.width > 0 else 0
        
        if percent_x <= self.config["left_edge_threshold_percent"]:
            return "volume"
        elif percent_x >= self.config["right_edge_threshold_percent"]:
            return "brightness"
        return "none"

    def adjust_volume(self, delta):
        # delta usually maps to physical distance.
        # Pipewire/wireplumber wpctl format: wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%+ or 5%-
        percent = int(abs(delta) * 100)
        if percent == 0:
            return
            
        cmd = ["wpctl", "set-volume", "@DEFAULT_SINK@"]
        if delta < 0: # Swiping down decreases volume (default)
            cmd.append(f"{percent}%-")
        else:
            cmd.append(f"{percent}%+")
            
        try:
            subprocess.run(self.user_env_prefix + cmd, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            log(f"Volume adjusted: {'+' if delta > 0 else '-'}{percent}%")
        except subprocess.CalledProcessError as e:
            log(f"Failed to adjust volume: {e}")
        except FileNotFoundError:
            log(f"Failed to adjust volume: 'wpctl' command not found. Is wireplumber installed?")

    def adjust_brightness(self, delta):
        # We need to compute a step, but KDE's brightness is absolute.
        # However, qdbus allows adjusting by looking up current and setting new.
        # Alternatively, KDE has commands to step brightness up/down, but `setBrightness` is standard.
        # Since reading current brightness might be slow in this loop, we can use qdbus to step it.
        # Actually, KDE provides step up/down dbus methods:
        # qdbus org.kde.Solid.PowerManagement /org/kde/Solid/PowerManagement/Actions/BrightnessControl org.kde.Solid.PowerManagement.Actions.BrightnessControl.brightnessSteps
        # A simpler way without reading is just using standard dbus send if available, 
        # or we invoke qdbus/dbus-send with the delta.
        
        # Let's map percent to a discrete step logic since we want to be smooth.
        # Because we accumulate delta until it triggers, we want a small step.
        percent = int(abs(delta) * 100)
        if percent == 0:
            return
            
        # For KDE Plasma via dbus-send (which is standard even without qdbus installed sometimes)
        # However, qdbus is guaranteed on KDE.
        # Let's use dbus-send which is ubiquitous, to either step or set.
        # But we need the current value. It's faster to just use brightnessctl and let KDE notice?
        # KDE 6 on Wayland doesn't always listen to udev backlight changes instantly unless via DBus.
        # Let's use `busctl` or `qdbus` to get the current max and current, then set it.
        # However, KDE has `org.kde.Solid.PowerManagement.Actions.BrightnessControl.setBrightness`
        
        # Let's use a simpler bash one-liner that gets current, adds/subs, and sets.
        # Since we just want it to register with the OSD.
        cmd_get = ["qdbus", "org.kde.Solid.PowerManagement", "/org/kde/Solid/PowerManagement/Actions/BrightnessControl", "org.kde.Solid.PowerManagement.Actions.BrightnessControl.brightness"]
        
        try:
            current_raw = subprocess.check_output(self.user_env_prefix + cmd_get, stderr=subprocess.DEVNULL).decode().strip()
            
            if not current_raw:
                return
                
            current = float(current_raw)
            maximum = self.max_brightness
            
            # calculate the new value based on the delta percentage
            step_val = max(1.0, maximum * (percent / 100.0))
            if delta < 0:
                new_val = max(0.0, current - step_val)
            else:
                new_val = min(maximum, current + step_val)
                
            cmd_set = ["qdbus", "org.kde.Solid.PowerManagement", "/org/kde/Solid/PowerManagement/Actions/BrightnessControl", "org.kde.Solid.PowerManagement.Actions.BrightnessControl.setBrightness", str(int(new_val))]
            
            subprocess.run(self.user_env_prefix + cmd_set, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            log(f"Brightness adjusted via DBus to {int(new_val)}")
            
        except subprocess.CalledProcessError as e:
            log(f"Failed to adjust brightness DBus: {e}")
        except FileNotFoundError:
            log(f"Failed to adjust brightness: 'qdbus' command not found. Are you on KDE?")

    async def run(self):
        log(f"Listening for events on {self.device.path}...")
        
        async for event in self.device.async_read_loop():
            if event.type == evdev.ecodes.EV_ABS:
                if event.code == evdev.ecodes.ABS_MT_SLOT:
                    self.current_slot = event.value
                
                elif event.code == evdev.ecodes.ABS_MT_TRACKING_ID:
                    if event.value != -1: # New touch
                        self.active_touches[self.current_slot] = {"x": None, "y": None, "action": None, "last_y_reported": None}
                    else: # Touch ended
                        self.active_touches.pop(self.current_slot, None)
                            
                elif event.code == evdev.ecodes.ABS_MT_POSITION_X:
                    if self.current_slot in self.active_touches:
                        touch = self.active_touches[self.current_slot]
                        touch["x"] = event.value
                        
                        # Determine action mode on first x reading if not set
                        if touch["action"] is None:
                            touch["action"] = self.get_action_mode(event.value)
                            
                        # If a single finger moves away from the edge, we might want to cancel, 
                        # but usually once an edge scroll starts, it locks in. We'll lock it in.
                            
                elif event.code == evdev.ecodes.ABS_MT_POSITION_Y:
                    if self.current_slot in self.active_touches:
                        touch = self.active_touches[self.current_slot]
                        y = event.value
                        touch["y"] = y
                        
                        if touch["action"] in ("volume", "brightness"):
                            if touch["last_y_reported"] is None:
                                touch["last_y_reported"] = y
                            else:
                                dy = touch["last_y_reported"] - y # Positive dy means swiping UP
                                
                                # Convert pixels to a percentage of screen/touchpad height
                                fractional_dy = dy / self.height
                                
                                # Apply sensitivity
                                move = fractional_dy * self.config["sensitivity_y"]
                                
                                if self.config.get("invert_y", False):
                                    move = -move
                                
                                if touch["action"] == "volume":
                                    self.accumulated_delta_vol += move
                                    if abs(self.accumulated_delta_vol) >= 0.05: # 5% step
                                        self.adjust_volume(self.accumulated_delta_vol)
                                        touch["last_y_reported"] = y
                                        self.accumulated_delta_vol = 0
                                        
                                elif touch["action"] == "brightness":
                                    self.accumulated_delta_bri += move
                                    if abs(self.accumulated_delta_bri) >= 0.05: # 5% step
                                        self.adjust_brightness(self.accumulated_delta_bri)
                                        touch["last_y_reported"] = y
                                        self.accumulated_delta_bri = 0

            elif event.type == evdev.ecodes.EV_SYN:
                # State synchronization, usually we process the accumulated slot data here
                # Our simple logic processes on the fly, which is mostly fine for this.
                pass

async def main():
    config = load_config()
    device = None
    try:
        device = get_touchpad_device()
        # Grab the device so we have exclusive access to it if needed? 
        # Usually not recommended for touchpad as we still want normal pointer movement.
        # However, libinput might consume it. Since we are just reading, it's fine.
        # But if we want to block pointer movement during edge scroll, we'd have to grab, 
        # which blocks all input. Wayland KWin handles grabbing on its end, so getting raw 
        # evdev access alongside it is tricky but usually works for reading.
        
        context = TouchpadGestureContext(device, config)
        await context.run()
    except Exception as e:
        print(f"Daemon failed: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    asyncio.run(main())
