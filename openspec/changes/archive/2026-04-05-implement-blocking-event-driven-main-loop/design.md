## Context

The current implementation uses a tight loop in `main()` that repeatedly calls `touchpad_service.fetch_events()`. The `fetch_events()` method on `evdev::Device` is non-blocking and returns immediately - if no events are available, it returns an empty iterator. This causes the CPU to spin at 100% when the touchpad is idle.

The `evdev` crate provides underlying device file descriptors that can be used with Unix `poll()` system calls. We need to:
1. Expose the device fd from `TouchpadService`
2. Use `poll()` to block efficiently until events are available
3. Maintain graceful shutdown capability via signal handling

**Current state of src/main.rs:**
```rust
loop {
    if let Err(error) = touchpad_service.fetch_events() {
        eprintln!("touchpad event loop error: {error}");
        thread::sleep(Duration::from_millis(250));
    }
}
```

**Current state of TouchpadService:**
- Holds ownership of an `evdev::Device`
- `fetch_events()` iterates over all available events
- No access to the underlying file descriptor

## Goals / Non-Goals

**Goals:**
- Achieve 0% CPU usage when touchpad is idle (no active touch)
- Maintain immediate response to touchpad events (<1ms latency)
- Add graceful shutdown on SIGTERM/SIGINT signals
- Keep the existing `fetch_events()` batch processing logic intact
- Support proper error recovery if device becomes unavailable

**Non-Goals:**
- No changes to gesture recognition logic or action handling
- No changes to service trait interfaces (AudioService, BrightnessService, etc.)
- No async/await or tokio runtime (overkill for this simple daemon)
- No multi-device support (out of scope for current single-touchpad setup)

## Decisions

### Use `nix::poll` instead of `std::os::unix::net`

**Decision**: Use the `nix` crate's `poll()` function rather than rolling our own unsafe FFI bindings.

**Rationale**:
- `nix` provides safe wrappers around Unix system calls
- We already need Unix-specific code for evdev, so portability is not a concern
- `nix::poll::poll()` provides clean Rust API over the underlying `poll(2)` syscall
- Well-maintained crate with good documentation

**Alternative considered**: Direct `libc` usage with unsafe blocks. Rejected due to unnecessary complexity when `nix` provides safe abstractions.

### Expose `AsFd` trait implementation

**Decision**: Implement `AsFd` for `TouchpadService` to expose the underlying device fd.

**Rationale**:
- Standard Rust I/O trait for file descriptor access
- Compatible with `nix::poll::PollFd::new()` which accepts `impl AsFd`
- Clean, idiomatic Rust approach

**Implementation**:
```rust
impl AsFd for TouchpadService<...> {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.device.as_fd()
    }
}
```

### Signal handling with `ctrlc` crate

**Decision**: Use the `ctrlc` crate for SIGINT/SIGTERM handling.

**Rationale**:
- Simple, battle-tested signal handling
- Provides cross-platform compatibility (though we only need Unix)
- Easy integration with a `std::sync::atomic::AtomicBool` flag
- Allows graceful shutdown without unsafe signal handler code

**Implementation pattern**:
```rust
let running = Arc::new(AtomicBool::new(true));
let r = running.clone();
ctrlc::set_handler(move || {
    r.store(false, Ordering::SeqCst);
})?;

while running.load(Ordering::SeqCst) {
    // poll with timeout to check running flag periodically
}
```

### Keep `fetch_events()` as-is, add `poll()` in main loop

**Decision**: Do not modify the internal event processing logic. Instead, wrap the call with `poll()`.

**Rationale**:
- `TouchpadService::fetch_events()` correctly batches and processes MT (multi-touch) events
- The state machine for tracking touches is complex and shouldn't be touched
- `fetch_events()` becomes effectively non-blocking when we know data is available
- Minimal code changes, reduces regression risk

**New main loop structure**:
```rust
loop {
    // Block until events available OR signal received (via self-pipe)
    let mut poll_fds = [
        PollFd::new(read_pipe.as_fd(), PollFlags::POLLIN),      // Self-pipe for signals
        PollFd::new(touchpad_service.as_fd(), PollFlags::POLLIN), // Touchpad events
    ];
    match poll(&mut poll_fds, None::<u16>) {  // Infinite timeout - true blocking
        Ok(0) => continue,  // Shouldn't happen with infinite timeout
        Ok(_) => {
            // Check if signal was received (pipe has data)
            if pipe_has_data(&poll_fds[0]) && !running.load(Ordering::SeqCst) {
                break;  // Graceful shutdown
            }
            // Check if touchpad has events
            if touchpad_has_data(&poll_fds[1]) {
                if let Err(e) = touchpad_service.fetch_events() {
                    handle_error(e);
                }
            }
        }
        Err(e) => handle_poll_error(e),
    }
}
```

### Self-pipe pattern for true blocking

**Decision**: Use the self-pipe trick with `pipe()` and `poll()` on both touchpad FD and pipe read FD with infinite timeout (`None::<u16>`).

**Rationale**:
- Achieves true 0% CPU when idle - no periodic wake-ups at all
- Signal handler writes a byte to the pipe to wake up `poll()` instantly
- Shutdown response is immediate (no 100ms delay)
- This is the standard Unix pattern for interrupting blocking `poll()` from signal handlers
- Cleaner than timeout-based polling - the process truly blocks until something happens

**Implementation**:
```rust
// Create self-pipe for signal handling
let (read_pipe, write_pipe) = pipe().map_err(|e| format!("failed to create self-pipe: {e}"))?;

// Signal handler writes to pipe when signal received
let running = Arc::new(AtomicBool::new(true));
let r = running.clone();
ctrlc::set_handler(move || {
    r.store(false, Ordering::SeqCst);
    let buf = [1u8];
    let _ = nix::unistd::write(&write_pipe, &buf);
})?;

// Main loop polls on both touchpad and pipe with infinite timeout
loop {
    let mut poll_fds = [
        PollFd::new(read_pipe.as_fd(), PollFlags::POLLIN),
        PollFd::new(touchpad_service.as_fd(), PollFlags::POLLIN),
    ];

    match poll(&mut poll_fds, None::<u16>) {
        Ok(0) => continue,  // Shouldn't happen with infinite timeout
        Ok(_) => {
            // Check pipe first (index 0)
            if poll_fds[0].revents().map_or(false, |r| r.contains(PollFlags::POLLIN)) {
                // Read the byte and check if we should exit
                if !running.load(Ordering::SeqCst) {
                    break;
                }
            }
            // Check touchpad (index 1)
            if poll_fds[1].revents().map_or(false, |r| r.contains(PollFlags::POLLIN)) {
                touchpad_service.fetch_events()?;
            }
        }
        Err(e) => handle_error(e),
    }
}
```

**Trade-off**: Slightly more complex than timeout-based polling, but achieves superior efficiency and responsiveness.

## Risks / Trade-offs

### [Risk] Signal handler writes to closed pipe
**Mitigation**: The signal handler uses `let _ = write(...)` to ignore errors. The main loop cleans up pipes after exiting the loop. In rare race conditions where the pipe is closed during signal handling, the signal may not immediately wake poll(), but the process will still exit on the next event or when manually killed.

### [Risk] Device removal during operation
**Mitigation**: `poll()` will return with an error (POLLERR/POLLHUP) if device is unplugged. We already have error handling that backs off and retries.

### [Risk] Platform portability (Unix-only)
**Mitigation**: The entire project is already Linux-specific due to evdev dependency. This change doesn't reduce portability further.

### [Risk] Accumulated events handling
**Mitigation**: When `poll()` returns, `fetch_events()` drains all available events in a single iteration. This is the same behavior as before, just triggered by `poll()` instead of spinning.

### [Risk] Signal handler race condition
**Mitigation**: Using `AtomicBool` with `SeqCst` ordering ensures signal handler and main thread have consistent view of the flag. The self-pipe pattern ensures `poll()` returns immediately when a signal is received, avoiding any delay.

## Migration Plan

This is a pure implementation optimization with no user-facing changes.

**Deployment steps**:
1. Update `Cargo.toml` to add `nix` and `ctrlc` dependencies
2. Implement `AsFd` for `TouchpadService`
3. Refactor `main()` to use `poll()` loop
4. Run `cargo check` and `cargo fmt` to verify
5. Test with `install.sh` and verify with `top` that CPU usage is 0% when idle

**Rollback**: Simple - revert the commit. No state changes or data migrations involved.

## Open Questions

- Should we add a small non-blocking check (1ms timeout) on startup to verify the device is working before entering the main loop?
- Should we consider using `epoll` instead of `poll` for potential performance benefits with many file descriptors (though we only have 2)?
