# asus-smartgesture-linux

Rust implementation of the touchpad gesture daemon for KDE Plasma.

## Temporary Touchpad Access

For MVP testing, run the daemon as your normal desktop user and grant temporary read access to the touchpad event device with [test.sh](./test.sh).

This avoids permanent system changes such as udev rules or group membership changes.

### Check the detected touchpad device

```bash
./test.sh status
```

### Grant temporary access

```bash
./test.sh grant
```

This uses `setfacl` to grant your user read access to the detected `/dev/input/event*` device.

### Run the daemon

Run the binary as your normal user, not with `sudo`:

```bash
cargo run
```

Or:

```bash
./target/debug/asus-smartgesture-linux
```

### Revoke the temporary access

```bash
./test.sh revoke
```

## Notes

- The ACL change is temporary and easy to undo with `./test.sh revoke`.
- If the touchpad device is recreated, you may need to run `./test.sh grant` again.
- `test.sh` auto-detects the first input device whose name contains `touchpad`.

## Configuration

The daemon reads gesture configuration from `~/.config/asus-touchpad-gesture.toml`. If this file does not exist, the daemon uses built-in default values.

Example `~/.config/asus-touchpad-gesture.toml`:

```toml
left_edge_threshold_percent = 0.1
right_edge_threshold_percent = 0.9
top_edge_threshold_percent = 0.1
sensitivity = 0.5
invert_y = false
volume_step = 0.05
brightness_step = 0.05
seek_step_microseconds = 10000000
```

### Fields

- `left_edge_threshold_percent`: Fraction of touchpad width for left edge gesture activation (0.0 to 1.0)
- `right_edge_threshold_percent`: Fraction of touchpad width for right edge gesture activation (0.0 to 1.0)
- `top_edge_threshold_percent`: Fraction of touchpad height for top edge gesture activation (0.0 to 1.0)
- `sensitivity`: Gesture detection sensitivity (0.0 to 1.0)
- `invert_y`: Invert vertical gesture direction
- `volume_step`: Volume change per gesture step (0.0 to 1.0)
- `brightness_step`: Brightness change per gesture step (0.0 to 1.0)
- `seek_step_microseconds`: Media seek step in microseconds (default: 10,000,000 = 10 seconds)

If the config file is missing, built-in defaults are used. If the file exists but contains invalid TOML or cannot be parsed, the daemon returns an error indicating the config path and failure reason.

## Media Seek Gestures

The daemon supports media playback control via top-edge horizontal swipe gestures when an MPRIS-compatible media player is running (e.g., Brave, Firefox, VLC, Spotify).

### Usage

- **Swipe right on top edge**: Seek forward by the configured step amount
- **Swipe left on top edge**: Seek backward by the configured step amount

### Requirements

- An MPRIS-compatible media player must be running
- Only single-finger gestures trigger media seek (multi-finger swipes are ignored)
- The gesture must start within the top edge zone (configurable via `top_edge_threshold_percent`)

### Example Configuration

```toml
# Top 10% of touchpad height triggers media seek
top_edge_threshold_percent = 0.1

# Seek 10 seconds per gesture (10,000,000 microseconds)
seek_step_microseconds = 10000000
```

## User Service

Install the Rust implementation as a `systemd --user` service with:

```bash
./install.sh
```

This does all of the following:

- builds the release binary
- installs `~/.config/systemd/user/asus-touchpad-gesture-rust.service`
- copies the persistent udev rule to `/etc/udev/rules.d/71-touchpad-gestures.rules`
- enables `udev` `uaccess` ACLs for the active local desktop user

Start the service with:

```bash
systemctl --user start asus-touchpad-gesture-rust.service
```

This path does not require adding your user to the `input` group.

The generated user service is hardened with a read-only system view, a private `/tmp`, no privilege escalation, and Unix-socket-only IPC. It intentionally does not use `PrivateDevices` because the daemon must still read the touchpad event node under `/dev/input`.

Follow logs with:

```bash
journalctl --user -u asus-touchpad-gesture-rust.service -f
```

Remove the user service with:

```bash
./uninstall.sh
```

This removes the user service and deletes the installed udev rule.
