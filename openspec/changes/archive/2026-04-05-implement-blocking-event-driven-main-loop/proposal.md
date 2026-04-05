## Why

The current main loop in `src/main.rs` uses a tight polling loop with `fetch_events()` that returns immediately even when no touchpad events are available. This causes 100% CPU usage when idle, wasting battery and system resources. We need an event-driven approach that blocks efficiently until touchpad input is actually available.

## What Changes

- Refactor `TouchpadService` to expose the underlying evdev device's file descriptor
- Replace the polling main loop with `poll()`-based blocking I/O that waits for events
- Add signal handling for graceful shutdown (SIGTERM/SIGINT)
- **BREAKING**: Change `fetch_events()` behavior - it will now expect to be called when data is known to be available (after `poll()` returns readable)
- Add `nix` crate dependency for `poll()` syscall support

## Capabilities

### New Capabilities
- `event-driven-main-loop`: Blocking I/O with poll-based event notification for 0% CPU when idle

### Modified Capabilities
- (No existing spec requirements are changing - this is purely an implementation optimization)

## Impact

- **Performance**: CPU usage drops from ~100% to 0% when touchpad is idle
- **Battery Life**: Significant improvement on laptops due to reduced wakeups
- **Responsiveness**: Touch events are still handled immediately when they arrive
- **Dependencies**: New dependency on `nix` crate for Unix poll syscall
- **Code Structure**: Minor changes to `TouchpadService` to expose device fd, major changes to `main()` loop
- **Shutdown**: Proper graceful shutdown on SIGTERM/SIGINT signals
