## ADDED Requirements

### Requirement: User gesture config file is loaded from the desktop user's home directory
The system SHALL read gesture configuration overrides from `$HOME/.config/asus-touchpad-gesture.toml` for the current desktop user.

#### Scenario: Config file path resolves for the current user
- **WHEN** the daemon needs gesture configuration and the current user's home directory is available
- **THEN** it looks for the config file at `$HOME/.config/asus-touchpad-gesture.toml`

### Requirement: Missing user config falls back to built-in defaults
The system SHALL use the built-in static configuration values when `$HOME/.config/asus-touchpad-gesture.toml` does not exist.

#### Scenario: Config file is absent
- **WHEN** the daemon attempts to read `$HOME/.config/asus-touchpad-gesture.toml` and the file is not found
- **THEN** it loads the same gesture values currently defined by the static configuration service

### Requirement: Present but invalid user config is rejected explicitly
The system MUST surface an actionable error when `$HOME/.config/asus-touchpad-gesture.toml` exists but cannot be read or parsed as valid configuration.

#### Scenario: Config file contains invalid TOML
- **WHEN** the config file exists but its contents cannot be decoded into gesture settings
- **THEN** the daemon returns an error that identifies the config file path and parse failure

#### Scenario: Config file cannot be read
- **WHEN** the config file exists but file access fails for a reason other than missing file
- **THEN** the daemon returns an error that identifies the config file path and read failure

### Requirement: User gesture config can be persisted to the same path
The system SHALL save gesture configuration updates to `$HOME/.config/asus-touchpad-gesture.toml`.

#### Scenario: Config is saved successfully
- **WHEN** the daemon or another component calls the configuration service to save gesture settings
- **THEN** it writes TOML-formatted settings to `$HOME/.config/asus-touchpad-gesture.toml`
