## ADDED Requirements

### Requirement: Main loop blocks efficiently when idle
The system SHALL use `poll()`-based blocking I/O to wait for touchpad events instead of polling in a tight loop.

#### Scenario: Idle state
- **WHEN** no touchpad events are available
- **THEN** the daemon SHALL block on `poll()` and consume negligible CPU (<1%)

#### Scenario: Event arrives
- **WHEN** a touchpad event occurs
- **THEN** the daemon SHALL wake from `poll()` and process events immediately

### Requirement: Graceful shutdown on signals
The system SHALL handle SIGTERM and SIGINT signals to exit the main loop cleanly using the self-pipe pattern for instant response.

#### Scenario: SIGINT received
- **WHEN** the user presses Ctrl+C (SIGINT)
- **THEN** the signal handler SHALL write a byte to the self-pipe
- **AND** `poll()` SHALL return immediately (no timeout delay)
- **AND** the daemon SHALL exit the main loop and terminate gracefully within milliseconds

#### Scenario: SIGTERM received
- **WHEN** systemd sends SIGTERM during service stop
- **THEN** the signal handler SHALL write a byte to the self-pipe
- **AND** `poll()` SHALL return immediately (no timeout delay)
- **AND** the daemon SHALL exit the main loop and terminate gracefully within milliseconds

### Requirement: TouchpadService exposes device fd
The TouchpadService SHALL implement `AsFd` to expose the underlying evdev device file descriptor for use with `poll()`.

#### Scenario: PollFd creation
- **WHEN** the main loop needs to poll for events
- **THEN** it SHALL create a `PollFd` from `touchpad_service.as_fd()`

### Requirement: Event processing remains unchanged
The system SHALL continue to use the existing `fetch_events()` method for batch event processing after `poll()` indicates data is available.

#### Scenario: Post-poll event fetch
- **WHEN** `poll()` returns indicating data is available
- **THEN** the system SHALL call `fetch_events()` to process all pending events
- **AND** the gesture recognition and action handling SHALL behave identically to the previous implementation

### Requirement: Error handling with backoff
The system SHALL handle errors from `poll()` and the touchpad device with appropriate backoff strategies.

#### Scenario: Poll error
- **WHEN** `poll()` returns an error
- **THEN** the system SHALL log the error
- **AND** sleep for 250ms before retrying

#### Scenario: Device error
- **WHEN** `fetch_events()` returns an error (e.g., device disconnected)
- **THEN** the system SHALL log the error
- **AND** sleep for 250ms before retrying
