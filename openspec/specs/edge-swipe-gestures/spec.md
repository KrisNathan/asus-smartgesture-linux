## Purpose
Define edge swipe gesture detection for controlling system settings (volume and brightness) via single-finger swipes along touchpad edges, with proper finger count isolation to prevent interference with desktop environment gestures.
## Requirements
### Requirement: Edge Swipe Finger Count Isolation
The system MUST ONLY trigger edge swipe gestures (for volume or brightness) when exactly one finger is touching the touchpad.

#### Scenario: Single finger edge swipe
- **WHEN** user swipes with exactly one finger along the configured edge
- **THEN** system adjusts the volume or brightness proportionally

#### Scenario: Multi-finger swipe attempting edge
- **WHEN** user performs a multi-finger DE gesture (e.g., 3-finger swipe) where one finger is on the configured edge
- **THEN** system completely ignores the input and does not adjust volume or brightness

#### Scenario: Finger added during active edge swipe
- **WHEN** user is performing an active single-finger edge swipe and places a second finger on the pad
- **THEN** system immediately halts the adjustment and ignores subsequent movements until exactly one finger remains

### Requirement: Top Edge Detection Zone
The system SHALL support top edge detection in addition to existing left and right edge detection.

#### Scenario: Top edge threshold configuration
- **WHEN** user configures `top_edge_threshold_percent` in the configuration file
- **THEN** system uses this value to determine the top edge detection zone as a fraction of total touchpad height

#### Scenario: Top edge detection active
- **WHEN** user's finger Y-coordinate is within the top edge threshold
- **THEN** system recognizes this as a top edge gesture context

#### Scenario: Top edge distinct from side edges
- **WHEN** user's finger is within the top edge zone but not within left or right edge zones
- **THEN** system treats this as a top edge gesture, not a side edge gesture

