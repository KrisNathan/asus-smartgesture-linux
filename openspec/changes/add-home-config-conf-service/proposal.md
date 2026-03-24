## Why

The daemon currently hardcodes gesture tuning values in `src/conf/static_conf_service.rs`, which forces users to rebuild the application to change sensitivity, edge thresholds, or step sizes. A user-scoped config file is needed so desktop users can tune gesture behavior without changing the deployed binary, while preserving the current defaults when no config file has been created.

## What Changes

- Add a file-backed configuration service that reads gesture settings from `$HOME/.config/asus-touchpad-gesture.toml`.
- Keep the existing static configuration values as the fallback source when the user config file does not exist.
- Parse the TOML config into the existing `Conf` model and return clear errors when the file exists but cannot be read or decoded.
- Route the daemon to use the file-backed service for configuration reads and writes instead of the static-only implementation.

## Capabilities

### New Capabilities
- `user-config-file`: User-scoped gesture configuration file loading and persistence for the daemon.

### Modified Capabilities
- `edge-swipe-gestures`: Gesture thresholds, sensitivity, inversion, and step sizes can be sourced from a user config file, with the current static values preserved as the missing-file fallback.

## Impact

- Affected code: `src/conf/*` and any daemon startup or service wiring that instantiates `StaticConfService`.
- Dependencies: TOML deserialization support and home-directory path resolution if not already available.
- User-visible behavior: Users can tune gesture settings through `~/.config/asus-touchpad-gesture.toml` without recompiling, and existing deployments keep current defaults until a config file is created.
