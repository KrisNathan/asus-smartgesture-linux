## 1. Promote the Rust project to the repository root

- [x] 1.1 Move the Rust crate files and supporting source tree from `linux-touchpad-gesture/` into the repository root so the root `Cargo.toml` and root `src/` define the production application.
- [x] 1.2 Move any Rust-specific helper assets that remain part of the supported workflow, then remove the obsolete Python prototype files and the now-redundant nested project layout.

## 2. Rewire deployment and helper scripts

- [x] 2.1 Replace the root `install.sh` with the production Rust install flow and update all asset paths so it builds and installs from the repository root.
- [x] 2.2 Add or update the matching root `uninstall.sh` and any retained helper scripts so they remove only the assets created by installation and no longer reference `linux-touchpad-gesture/`.

## 3. Align documentation with the Rust root layout

- [x] 3.1 Rewrite the top-level `README.md` so it documents the Rust project as the only supported implementation, including root-level build, run, install, and uninstall commands.
- [x] 3.2 Remove or update any remaining repository guidance that mentions Python prototype setup, `uv`, virtual environments, or the nested Rust project path.

## 4. Validate the migrated root project

- [x] 4.1 Run `cargo fmt` and `cargo check` from the repository root and fix any path or compilation issues introduced by the move.
- [x] 4.2 Verify that install, uninstall, helper-script text, and documentation all reference the same root-level service, udev rule, and asset locations with no stale prototype references.
