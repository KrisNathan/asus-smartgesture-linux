# Repository Agent Rules

## Mission

- Treat this repository as a transition from a disposable Python prototype to the real Rust application.
- The production app to ship and maintain lives in `linux-touchpad-gesture/`.
- The deployable install and uninstall flow is `linux-touchpad-gesture/install.sh` and `linux-touchpad-gesture/uninstall.sh`.
- The top-level Python implementation is a prototype and migration reference only. Assume it will be discarded. Do not add new product direction there unless a task explicitly says to touch the prototype.

## Source Of Truth

- Prefer the Rust implementation when behavior, docs, or scripts diverge.
- Keep repo documentation explicit about the current state:
  - Python at the repository root is prototype-only.
  - Rust in `linux-touchpad-gesture/` is the deployable implementation.
- Treat the top-level `install.sh` as prototype setup, not the production deployment path.
- Avoid copying new logic from Rust back into Python unless the task is explicitly about prototype parity during migration.

## Correctness And Safety

- Preserve least-privilege operation. The daemon must run as the normal desktop user, not as root.
- Do not weaken the security model to gain convenience. Prefer targeted udev `uaccess`, temporary ACLs, and hardened user services over broad permissions or privileged execution.
- Favor explicit errors over silent fallback. If touchpad access, `wpctl`, `qdbus`, `systemd`, or `udev` setup fails, surface a clear actionable error.
- Avoid `unwrap`, `expect`, or panic-driven control flow in long-running daemon paths unless failure is truly unrecoverable and documented.
- Do not introduce `unsafe` Rust unless it is unavoidable, tightly scoped, and justified in comments and review notes.
- Preserve idempotence for install, uninstall, and test helper scripts.
- Do not remove or overwrite user or system state that the installer did not create.

## Rust Engineering Rules

- Keep behavior changes inside `linux-touchpad-gesture/` unless the task explicitly requires prototype updates.
- Prefer small, typed abstractions and `Result`-based error handling over ad hoc stringly control flow.
- Keep modules cohesive. Push hardware, IPC, and shell interactions behind service boundaries when practical.
- Validate edge cases around input devices, multi-touch state, missing dependencies, and permission failures.
- When spawning external commands, keep arguments explicit and avoid shell-dependent behavior.
- Maintain compatibility with the current service model: KDE Plasma, Wayland, `wpctl`, `qdbus`, `systemd --user`, and udev-based device access.

## Installer And Uninstaller Contract

- Treat `linux-touchpad-gesture/install.sh` and `linux-touchpad-gesture/uninstall.sh` as a matched pair.
- `uninstall.sh` must perform a clean teardown of everything `install.sh` adds or enables, and nothing unrelated.
- If `install.sh` changes what it creates, copies, enables, or reloads, update `uninstall.sh` in the same change so teardown remains complete and correct.
- Teardown must be safe to run repeatedly.
- Migration cleanup is allowed only when it is deliberate and documented, such as removing a legacy rule path that the installer previously managed.
- Keep README instructions aligned with the actual installer and uninstaller behavior.

## Change Discipline

- Make the smallest change that fully solves the task.
- Do not mix prototype cleanups with Rust production work unless the task requires both.
- Preserve existing hardening in service files unless there is a clear reason to change it.
- If a change affects deployment, permissions, or teardown, update code, scripts, and docs together.

## Validation

- For Rust code changes, run at least `cargo fmt` and `cargo check` in `linux-touchpad-gesture/` unless the task prevents it.
- Prefer adding or updating tests when behavior is subtle, stateful, or easy to regress.
- If you cannot run validation, say so explicitly and explain the gap.
