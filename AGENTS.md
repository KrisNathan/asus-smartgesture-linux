# Repository Rules

- Prod Rust app `asus-smartgesture-linux`.
- Deploy: root `install.sh` + `uninstall.sh`.
- Source of truth: Rust; scripts own deploy.


## Repo context

- Rust daemon for ASUS-style touchpad edges.
- KDE Plasma 6 Wayland; Fedora 43 target.
- Left edge volume; right edge brightness.
- Swipe response: proportional, smooth, low CPU.
- Read device-level (`libinput`/`evdev`), not X11.
- Use touchpad size for edge detect.
- Volume: D-Bus or `wpctl`. Brightness: PowerDevil D-Bus or `brightnessctl`.
- Handle multi-device, sleep/wake, missing perms.
- User-level setup only: `udev` + `systemd --user`, no root.

## Safety

- User-level only; never root.
- Least privilege: `udev` `uaccess`, temp ACLs, hardened user services.
- Fail loud on `touchpad`, `wpctl`, `qdbus`, `systemd`, `udev` setup.
- No `unwrap`/`expect`/panic in daemon unless truly fatal.
- Avoid `unsafe`; keep it narrow if unavoidable.
- Install/uninstall/test helpers idempotent.
- Do not touch state installer did not create.

## Rust

- Small typed `Result` code.
- Cohesive modules; hide hardware/IPC/shell behind services.
- Cover input-device, multi-touch, missing-dep, permission edges.
- External commands: explicit args, no shell tricks.
- Keep KDE Plasma/Wayland/`wpctl`/`qdbus`/`systemd --user`/udev compat.

## Deploy

- `install.sh` + `uninstall.sh` are a pair.
- If install create/copy/enable/reload, update uninstall same change.
- Teardown safe to rerun.
- Migration cleanup only when deliberate and documented.
- Keep README synced.

## Change

- Smallest change that works.
- Keep hardening unless clear reason.
- If deploy/perm/teardown changes, update code, scripts, docs together.

## Validation

- Run `cargo fmt` + `cargo check` for Rust unless blocked.
- Add/update tests for subtle or stateful behavior.
- If validation not run, say why.

## Git

- Commit messages must follow `git-semantic-commit-message`.
