## ADDED Requirements

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
