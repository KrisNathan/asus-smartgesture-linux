## Why

The repository currently presents the Python prototype at the root even though the deployable implementation already lives in `linux-touchpad-gesture/`. That split creates misleading documentation, duplicate install paths, and a high risk that future work will target the wrong codebase.

## What Changes

- **BREAKING** Make the repository root the Rust application root by moving the production crate, service scripts, and supporting assets out of `linux-touchpad-gesture/`.
- Remove the root-level Python prototype implementation and Python packaging/setup files that currently look like the main product entrypoints.
- Replace the root install and uninstall flows so they match the existing production Rust deployment contract.
- Update repository documentation so `README.md` and related instructions describe the Rust application at the root, its install flow, and the removal of the prototype path.
- Align references to runtime assets such as the udev rule, systemd user service, config defaults, and validation commands with the new root layout.

## Capabilities

### New Capabilities
- `rust-root-project`: The repository root is the deployable Rust project, including source layout, install entrypoints, and operator-facing documentation.

### Modified Capabilities
- None.

## Impact

- Affected code: root repository layout, Cargo manifest and Rust source tree, install/uninstall scripts, service and asset path references, README and migration-facing documentation.
- Removed components: prototype Python daemon, Python dependency metadata, and prototype setup flow at the repository root.
- Affected workflows: local development, release builds, install/uninstall, and contributor guidance for where production changes belong.
