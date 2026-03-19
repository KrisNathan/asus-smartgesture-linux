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
- copies the persistent udev rule to `/etc/udev/rules.d/99-touchpad-gestures.rules`
- adds your user to the `input` group

Start the service with:

```bash
systemctl --user start asus-touchpad-gesture-rust.service
```

After `./install.sh`, log out and log back in before starting the service so the `input` group change takes effect.

Follow logs with:

```bash
journalctl --user -u asus-touchpad-gesture-rust.service -f
```

Remove the user service with:

```bash
./uninstall.sh
```

This removes the user service, deletes the installed udev rule, and removes your user from the `input` group.

After `./uninstall.sh`, log out and log back in for the group removal to fully take effect.
