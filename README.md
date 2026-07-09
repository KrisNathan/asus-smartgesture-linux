# asus-smartgesture-linux

Rust implementation of the touchpad gesture daemon for KDE Plasma.

## Temporary Touchpad Access

For MVP testing, run the daemon as your normal desktop user and grant temporary device access with [test.sh](./test.sh).

This avoids permanent system changes such as udev rules or group membership changes.

### Check the detected touchpad device

```bash
./test.sh status
```

### Grant temporary access

```bash
./test.sh grant
```

This uses `setfacl` to grant your user read access to the detected `/dev/input/event*` device. If `/dev/uinput` exists, it also grants read/write access for arrow-key media mode.

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

- The ACL changes are temporary and easy to undo with `./test.sh revoke`.
- If the touchpad device is recreated, you may need to run `./test.sh grant` again.
- `test.sh` auto-detects the first input device whose name contains `touchpad`.

## Configuration

The daemon reads gesture configuration from `~/.config/asus-touchpad-gesture.toml`. If the `HOME` environment variable is not set, the config path falls back to the current directory (`./.config/asus-touchpad-gesture.toml`). If the config file does not exist, the daemon uses built-in default values.

After modifying the configuration, don't forget to restart the service: `systemctl --user restart asus-touchpad-gesture-rust.service`

Example `~/.config/asus-touchpad-gesture.toml`:

```toml
left_edge_threshold_percent = 0.1
right_edge_threshold_percent = 0.9
top_edge_threshold_percent = 0.1
sensitivity = 0.5
invert_y = false
volume_step = 0.05
brightness_step = 0.05
media_step = 0.05
seek_step_microseconds = 10000000
media_control_mode = "mpris_seek"
```

### Fields

- `left_edge_threshold_percent`: Fraction of touchpad width for left edge gesture activation (0.0 to 1.0)
- `right_edge_threshold_percent`: Fraction of touchpad width for right edge gesture activation (0.0 to 1.0)
- `top_edge_threshold_percent`: Fraction of touchpad height for top edge gesture activation (0.0 to 1.0)
- `sensitivity`: Gesture detection sensitivity (0.0 to 1.0)
- `invert_y`: Invert vertical gesture direction
- `volume_step`: Volume change per gesture step (0.0 to 1.0)
- `brightness_step`: Brightness change per gesture step (0.0 to 1.0)
- `media_step`: Fraction of touchpad width that triggers one media seek step (0.0 to 1.0; default 0.05). Controls swipe distance for both MPRIS and arrow-key modes
- `seek_step_microseconds`: MPRIS seek amount per `media_step` of travel, in microseconds (default: 10,000,000 = 10 seconds). Unused for tap count in arrow-key mode. `0` disables media seek gestures
- `media_control_mode`: Media gesture backend, either `mpris_seek` or `arrow_keys` (default: `mpris_seek`)

If the config file is missing, built-in defaults are used. If the file exists but contains invalid TOML or cannot be parsed, the daemon returns an error indicating the config path and failure reason.

## Media Seek Gestures

The daemon supports media playback control via top-edge horizontal swipe gestures. By default it uses MPRIS seek calls when an MPRIS-compatible media player is running (e.g., Brave, Firefox, VLC, Spotify). Set `media_control_mode = "arrow_keys"` to emulate left/right arrow key taps instead. Swipe distance is paced by `media_step`: each step seeks by `seek_step_microseconds` (MPRIS) or taps once (arrow keys).

### Usage

- **Swipe right on top edge**: Seek forward once per `media_step` of travel (MPRIS: `seek_step_microseconds`; arrow keys: one right-arrow tap)
- **Swipe left on top edge**: Seek backward once per `media_step` of travel (MPRIS: `seek_step_microseconds`; arrow keys: one left-arrow tap)

### Requirements

- An MPRIS-compatible media player must be running
- Arrow-key mode requires read/write access to `/dev/uinput`
- Only single-finger gestures trigger media seek (multi-finger swipes are ignored)
- The gesture must start within the top edge zone (configurable via `top_edge_threshold_percent`)

### Example Configuration

```toml
# Top 10% of touchpad height triggers media seek
top_edge_threshold_percent = 0.1

# Each 10% of horizontal travel seeks 10 seconds
media_step = 0.1
seek_step_microseconds = 10000000

# Or emulate left/right arrow keys instead of MPRIS seek
media_control_mode = "arrow_keys"
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
