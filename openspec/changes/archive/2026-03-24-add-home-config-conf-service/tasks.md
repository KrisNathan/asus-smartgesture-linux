## 1. Configuration model and dependencies

- [x] 1.1 Add the Rust dependencies needed to serialize and deserialize `Conf` as TOML.
- [x] 1.2 Update the `Conf` type as needed so it can be read from and written to a TOML config file without changing the existing gesture fields.

## 2. File-backed config service

- [x] 2.1 Implement a new `FileConfService` in `src/conf/` that resolves `$HOME/.config/asus-touchpad-gesture.toml` for the current user.
- [x] 2.2 Make `FileConfService::get_conf()` fall back to `StaticConfService` when the config file is missing and return explicit errors for unreadable or invalid existing files.
- [x] 2.3 Implement `FileConfService::save_conf()` to persist gesture settings as TOML to the same user config path.

## 3. Runtime wiring and validation

- [x] 3.1 Export the new config service from `src/conf/mod.rs` and switch `src/main.rs` to instantiate it instead of `StaticConfService`.
- [x] 3.2 Add or update tests covering valid file reads, missing-file fallback, and invalid-file error behavior for the config service.
- [x] 3.3 Run `cargo fmt` and `cargo check` from the repository root and address any issues caused by the config service change.

## 4. User-facing documentation

- [x] 4.1 Update the README with the location and behavior of `~/.config/asus-touchpad-gesture.toml`, including the built-in-default fallback when the file does not exist.
