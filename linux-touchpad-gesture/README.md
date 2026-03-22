# linux-touchpad-gesture

Rust implementation of the touchpad gesture daemon for KDE Plasma.

## Temporary Touchpad Access

For MVP testing, run the daemon as your normal desktop user and grant temporary read access to the touchpad event device with [test.sh](/home/kris/Documents/projects/asus-touchpad-gesture-linux/linux-touchpad-gesture/test.sh).

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
./target/debug/linux-touchpad-gesture
```

### Revoke the temporary access

```bash
./test.sh revoke
```

## Notes

- The ACL change is temporary and easy to undo with `./test.sh revoke`.
- If the touchpad device is recreated, you may need to run `./test.sh grant` again.
- `test.sh` auto-detects the first input device whose name contains `touchpad`.

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
