## Why

Running `cargo clippy` on the project surfaces several linter warnings that should be resolved to maintain code quality and follow Rust idioms. These are straightforward fixes that improve readability and prevent future noise in the build output.

## What Changes

- Replace redundant field names in struct initialization (`conn: conn` → `conn`) in `kde_dbus_brightness_service.rs`
- Rename `conf` module to avoid module inception (`conf/conf` → `conf/config`)
- Replace `.map_or(false, ...)` with `.is_some_and(...)` in `touchpad_service.rs`
- Change `&PathBuf` parameter to `&Path` in `touchpad_service.rs` for idiomatic API design
- Replace `print!(.."\n")` with `println!(..)` in `touchpad_service.rs`

## Capabilities

### New Capabilities

None. This is a code quality / linter fix with no new functionality.

### Modified Capabilities

None. No spec-level behavior changes.

## Impact

- Source files: `src/brightness/kde_dbus_brightness_service.rs`, `src/conf/mod.rs`, `src/touchpad_service.rs`
- No API changes, no dependency changes, no behavioral changes
- All changes are purely cosmetic / idiomatic improvements
