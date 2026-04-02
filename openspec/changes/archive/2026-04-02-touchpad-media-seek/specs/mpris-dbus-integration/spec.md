## ADDED Requirements

### Requirement: MPRIS Media Player Discovery
The system SHALL discover MPRIS-compatible media players via D-Bus name enumeration.

#### Scenario: Active media player is discovered
- **WHEN** an MPRIS-compatible media player is running
- **THEN** system discovers the player by querying D-Bus for names matching `org.mpris.MediaPlayer2.*` pattern

#### Scenario: Multiple media players running
- **WHEN** multiple MPRIS-compatible media players are running
- **THEN** system uses the first discovered player based on D-Bus name enumeration order

#### Scenario: No media players running
- **WHEN** no MPRIS-compatible media players are running
- **THEN** system logs a debug message and does not perform any seek operation

### Requirement: MPRIS Seek Command Execution
The system SHALL send seek commands to the discovered media player using the MPRIS D-Bus interface.

#### Scenario: Seek forward command sent
- **WHEN** system needs to seek forward by N microseconds
- **THEN** system calls `org.mpris.MediaPlayer2.Player.Seek` method with positive offset N on the discovered player's D-Bus object path

#### Scenario: Seek backward command sent
- **WHEN** system needs to seek backward by N microseconds
- **THEN** system calls `org.mpris.MediaPlayer2.Player.Seek` method with negative offset -N on the discovered player's D-Bus object path

#### Scenario: MPRIS call fails
- **WHEN** the MPRIS D-Bus call fails or times out
- **THEN** system logs an error message and continues operation without crashing

### Requirement: MPRIS D-Bus Timeout
The system SHALL use a reasonable timeout for MPRIS D-Bus calls to prevent blocking the gesture processing loop.

#### Scenario: D-Bus call timeout
- **WHEN** an MPRIS D-Bus call takes longer than 500 milliseconds
- **THEN** system times out the call, logs an error, and continues processing subsequent gestures

### Requirement: MPRIS Interface Compliance
The system SHALL use the standard MPRIS D-Bus interface specification for media control.

#### Scenario: Seek method signature
- **WHEN** system calls the Seek method
- **THEN** system uses the method signature `org.mpris.MediaPlayer2.Player.Seek(x: int64)` where x is the offset in microseconds
