## ADDED Requirements

### Requirement: D-Bus connection establishment
The system SHALL establish a connection to the session D-Bus and maintain it for audio service communication with PulseAudio or PipeWire.

#### Scenario: Successful connection on initialization
- **WHEN** the D-Bus audio service is initialized
- **THEN** a valid D-Bus connection to the session bus is established

#### Scenario: Connection failure handling
- **WHEN** the D-Bus connection cannot be established
- **THEN** the service returns an appropriate error indicating connection failure

### Requirement: Volume adjustment via D-Bus
The system SHALL adjust the default audio sink volume by the specified delta percentage using D-Bus method calls to the audio server.

#### Scenario: Volume increase
- **WHEN** a positive delta value is provided (e.g., +0.05 for 5% increase)
- **THEN** the default sink volume is increased by that percentage via D-Bus

#### Scenario: Volume decrease
- **WHEN** a negative delta value is provided (e.g., -0.05 for 5% decrease)
- **THEN** the default sink volume is decreased by that percentage via D-Bus

#### Scenario: Zero delta handling
- **WHEN** a delta of zero is provided
- **THEN** no volume change is made and the call succeeds without error

### Requirement: Default sink identification
The system SHALL identify and target the current default audio sink for volume operations without requiring explicit sink specification.

#### Scenario: Single default sink
- **WHEN** there is one default audio sink configured
- **THEN** volume adjustments are applied to that sink

#### Scenario: No default sink available
- **WHEN** no default audio sink is configured
- **THEN** the service returns an error indicating no default sink found

### Requirement: AudioService trait compliance
The system SHALL implement the `AudioService` trait with identical behavior to existing implementations, ensuring drop-in compatibility.

#### Scenario: Trait method compliance
- **WHEN** the D-Bus audio service implements `AudioService`
- **THEN** it provides `new()` and `adjust_volume(&self, delta: &f64)` methods with matching signatures

#### Scenario: Error handling consistency
- **WHEN** an error occurs during volume adjustment
- **THEN** the service returns `Result<(), std::io::Error>` consistent with the trait definition

### Requirement: Audio server compatibility
The system SHALL work with both PulseAudio and PipeWire audio servers, detecting and adapting to whichever is active on the system.

#### Scenario: PulseAudio detection
- **WHEN** PulseAudio is the active audio server
- **THEN** D-Bus calls target the PulseAudio D-Bus interface

#### Scenario: PipeWire detection
- **WHEN** PipeWire is the active audio server
- **THEN** D-Bus calls target the PipeWire D-Bus interface (via PulseAudio compatibility layer if needed)

### Requirement: No external command dependencies
The system SHALL perform all volume operations through direct D-Bus communication without spawning external processes or executing shell commands.

#### Scenario: Volume adjustment without process spawning
- **WHEN** adjusting volume via the D-Bus audio service
- **THEN** no external commands like `wpctl`, `pactl`, or shell processes are executed
