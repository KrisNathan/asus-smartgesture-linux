## 1. Brightness service clippy fixes

- [x] 1.1 Replace redundant field names in `src/brightness/kde_dbus_brightness_service.rs`: change `conn: conn` to `conn` and `proxy: proxy` to `proxy`

## 2. Config module clippy fixes

- [x] 2.1 Rename `src/conf/conf.rs` to `src/conf/config.rs` to resolve module inception warning
- [x] 2.2 Update `src/conf/mod.rs`: change `mod conf` to `mod config` and `pub use conf::Conf` to `pub use config::Conf`

## 3. Touchpad service clippy fixes

- [x] 3.1 Replace `.map_or(false, |axes| { ... })` with `.is_some_and(|axes| { ... })` in `src/touchpad_service.rs:27`
- [x] 3.2 Change `describe_touchpad_access_failure` parameter from `&PathBuf` to `&Path` in `src/touchpad_service.rs:37`
- [x] 3.3 Replace `print!(.."\n")` with `println!(..)` in `src/touchpad_service.rs:210-213`

## 4. Verification

- [x] 4.1 Run `cargo clippy` and confirm zero actionable warnings remain
- [x] 4.2 Run `cargo check` to verify compilation
