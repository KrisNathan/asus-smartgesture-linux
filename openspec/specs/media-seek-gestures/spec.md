# media-seek-gestures Specification

## Purpose
TBD - created by archiving change touchpad-media-seek. Update Purpose after archive.
## Requirements
### Requirement: Top Edge Horizontal Swipe Detection
The system SHALL detect horizontal swipe gestures on the top edge of the touchpad and trigger media seek operations.

#### Scenario: Right swipe on top edge seeks forward
- **WHEN** user swipes right with one finger starting from the top edge
- **THEN** system sends a seek forward command to the active media player

#### Scenario: Left swipe on top edge seeks backward
- **WHEN** user swipes left with one finger starting from the top edge
- **THEN** system sends a seek backward command to the active media player

#### Scenario: Multi-finger swipe on top edge is ignored
- **WHEN** user performs a multi-finger swipe where one or more fingers are on the top edge
- **THEN** system completely ignores the input and does not send any media commands

#### Scenario: Finger added during active top edge swipe
- **WHEN** user is performing an active single-finger top edge swipe and places a second finger on the pad
- **THEN** system immediately halts the seek operation and ignores subsequent movements until exactly one finger remains

### Requirement: Configurable Top Edge Threshold
The system SHALL support configurable top edge detection zone via the `top_edge_threshold_percent` configuration field.

#### Scenario: Top edge threshold determines detection zone
- **WHEN** user sets `top_edge_threshold_percent` to 0.1
- **THEN** system triggers top edge gestures only when the finger Y-coordinate is within the top 10% of the touchpad height

#### Scenario: Missing top edge threshold uses default
- **WHEN** user's configuration file does not specify `top_edge_threshold_percent`
- **THEN** system uses a default value of 0.1 (top 10%)

### Requirement: Configurable Seek Step Amount
The system SHALL support configurable seek step amount via the `seek_step_microseconds` configuration field.

#### Scenario: Seek step controls forward/backward distance
- **WHEN** user sets `seek_step_microseconds` to 10000000 and swipes right on the top edge
- **THEN** system sends a seek command with +10,000,000 microseconds (10 seconds forward)

#### Scenario: Seek step controls backward distance
- **WHEN** user sets `seek_step_microseconds` to 5000000 and swipes left on the top edge
- **THEN** system sends a seek command with -5,000,000 microseconds (5 seconds backward)

#### Scenario: Missing seek step uses default
- **WHEN** user's configuration file does not specify `seek_step_microseconds`
- **THEN** system uses a default value of 10,000,000 microseconds (10 seconds)

### Requirement: Single Finger Gesture Isolation
The system MUST ONLY trigger top edge media seek gestures when exactly one finger is touching the touchpad.

#### Scenario: Single finger top edge swipe
- **WHEN** user swipes with exactly one finger along the top edge
- **THEN** system sends the media seek command proportionally

#### Scenario: Multi-finger swipe attempting top edge
- **WHEN** user performs a multi-finger DE gesture (e.g., 3-finger swipe) where one finger is on the top edge
- **THEN** system completely ignores the input and does not send any media commands

