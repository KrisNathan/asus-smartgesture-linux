## 1. Add Dependencies

- [x] 1.1 Add `nix` crate to `Cargo.toml` with features for `poll` support
- [x] 1.2 Add `ctrlc` crate to `Cargo.toml` for signal handling
- [x] 1.3 Run `cargo check` to verify dependencies resolve correctly

## 2. Expose Device File Descriptor

- [x] 2.1 Add `use std::os::unix::io::{AsFd, BorrowedFd};` to `src/touchpad_service.rs`
- [x] 2.2 Implement `AsFd` trait for `TouchpadService` struct that returns `self.device.as_fd()`
- [x] 2.3 Run `cargo check` to verify the implementation compiles

## 3. Refactor Main Loop

- [x] 3.1 Add imports to `src/main.rs`: `nix::poll::{poll, PollFd, PollFlags}`, `nix::unistd::{close, pipe, read}`, `std::os::fd::{AsFd, AsRawFd}`, `std::sync::atomic::{AtomicBool, Ordering}`, `std::sync::Arc`
- [x] 3.2 Create self-pipe using `nix::unistd::pipe()` for signal handling
- [x] 3.3 Set up signal handler using `ctrlc::set_handler()` that writes a byte to the pipe on signal
- [x] 3.4 Use `poll()` with infinite timeout (`None::<u16>`) on BOTH touchpad FD and pipe read FD
- [x] 3.5 Handle `poll()` return values: check which FD is ready (pipe for signal, touchpad for events)
- [x] 3.6 Read and discard byte from pipe when signal received, then check `running` flag and exit
- [x] 3.7 Handle `fetch_events()` errors with backoff (log and sleep 250ms)
- [x] 3.8 Add graceful shutdown message before returning from main

## 4. Validation

- [x] 4.1 Run `cargo fmt` to ensure code formatting
- [x] 4.2 Run `cargo check` to verify no compilation errors
- [x] 4.3 Run `cargo build --release` to verify release build succeeds
- [x] 4.4 Run `./install.sh` to install the updated daemon
- [x] 4.5 Verify with `top` or `htop` that CPU usage is 0% when touchpad is idle (true blocking)
- [x] 4.6 Test touch gestures work correctly (volume on left edge, brightness on right edge, media on top edge)
- [x] 4.7 Test graceful shutdown with Ctrl+C (SIGINT) - should exit instantly (no 100ms delay)
- [x] 4.8 Run `./uninstall.sh` to verify clean removal still works
