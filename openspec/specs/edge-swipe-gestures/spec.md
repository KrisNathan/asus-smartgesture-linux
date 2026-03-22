## ADDED Requirements

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
