DISCLAIMER: Currently only a vibe-coded prototype. It works, but it's not production-ready.

# ASUS Touchpad Gesture Linux

A lightweight background daemon for Linux that adds advanced touchpad edge-scrolling gestures, inspired by ASUS features on Windows. This tool allows you to smoothly adjust your system volume and screen brightness simply by swiping up or down along the extreme edges of your laptop's touchpad.

Designed primarily for **Fedora 43** running **KDE Plasma 6 on Wayland**, it bypasses standard Wayland input isolation by reading directly from `evdev`, making it highly reliable.

## Features

- **Left-Edge Scroll:** Adjust system volume.
- **Right-Edge Scroll:** Adjust screen brightness.
- **Smoothness:** Adjustments are dynamically calculated based on the distance of your swipe.
- **Wayland Native:** Works flawlessly under Wayland by hooking directly into `libevdev`.
- **Low Footprint:** Written in Python using `asyncio` to ensure minimal CPU usage in the background.

## Prerequisites

Ensure you have the following system utilities installed (the install script will check for them):

- [`uv`](https://docs.astral.sh/uv/) (required for Python package management)
- `qdbus` (usually pre-installed with KDE Plasma, used for brightness control)
- `wireplumber` (specifically `wpctl`, for volume control)

## Usage

```
sudo ./.venv/bin/python -m asus_touchpad_gesture.py
```

## Installation

The installation does not require running the daemon as root, but it does require setting up a `udev` rule to grant the active local desktop user permission to read touchpad events.

1. Clone or download this repository.
2. Make the install script executable:
   ```bash
   chmod +x install.sh
   ```
3. Run the local setup script to create the Python virtual environment and configure the user systemd service:
   ```bash
   ./install.sh
   ```
4. As instructed by the script, run the following commands with `sudo` to apply the necessary `udev` rule:

   ```bash
   sudo cp 71-touchpad-gestures.rules /etc/udev/rules.d/
   sudo udevadm control --reload-rules && sudo udevadm trigger
   ```

   _(If brightness controls do not work out of the box, you may also need to run `sudo usermod -aG video $USER`)_

5. Start and enable the gesture daemon:
   ```bash
   systemctl --user start asus-touchpad-gesture.service
   systemctl --user enable asus-touchpad-gesture.service
   ```

The generated user service is hardened with a read-only system view, a private `/tmp`, no privilege escalation, and Unix-socket-only IPC. It intentionally does not use `PrivateDevices` because the daemon must still read the touchpad event node under `/dev/input`.

## Configuration

A configuration file is generated upon installation at `~/.config/asus-touchpad-gesture/config.json`.

```json
{
  "left_edge_threshold_percent": 0.1,
  "right_edge_threshold_percent": 0.9,
  "sensitivity_y": 0.05,
  "invert_y": false
}
```

- **`left_edge_threshold_percent`**: Width of the left edge trigger zone (0.10 = 10% of the touchpad width).
- **`right_edge_threshold_percent`**: Start of the right edge trigger zone (0.90 = the rightmost 10%).
- **`sensitivity_y`**: Multiplier for the swipe distance. Lower values require longer swipes for the same volume/brightness change.
- **`invert_y`**: Set to `true` if scrolling up decreases volume/brightness instead of increasing it.

After modifying the configuration, restart the service:

```bash
systemctl --user restart asus-touchpad-gesture.service
```

## Troubleshooting

- **No volume/brightness change:** Verify the `udev` rule is installed as `/etc/udev/rules.d/71-touchpad-gestures.rules` and reloaded. You can inspect the device ACLs with `getfacl /dev/input/eventX` and confirm your active desktop user has read access.
- **Check daemon logs:**
  ```bash
  journalctl --user -u asus-touchpad-gesture.service -f
  ```
- **"No touchpad device found" error:** Verify the `udev` rules are loaded correctly. You can also manually test your devices by running `evtest` (requires root) to verify which `/dev/input/eventX` corresponds to your touchpad.
