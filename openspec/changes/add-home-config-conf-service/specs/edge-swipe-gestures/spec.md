## ADDED Requirements

### Requirement: Edge swipe gestures use the active gesture configuration source
The system SHALL apply edge swipe gesture thresholds, sensitivity, inversion, volume step size, and brightness step size from the active gesture configuration source for the current user.

#### Scenario: User config file provides gesture values
- **WHEN** `$HOME/.config/asus-touchpad-gesture.toml` exists and contains valid gesture settings
- **THEN** edge swipe gesture handling uses those configured values instead of the built-in defaults

#### Scenario: User config file is absent
- **WHEN** `$HOME/.config/asus-touchpad-gesture.toml` does not exist
- **THEN** edge swipe gesture handling uses the built-in static gesture values
