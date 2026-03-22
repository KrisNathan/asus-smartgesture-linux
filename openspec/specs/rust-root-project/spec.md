## ADDED Requirements

### Requirement: Repository root hosts the production Rust crate
The repository root SHALL contain the deployable Rust application crate so that standard Rust development and validation commands run from the top level without requiring a nested project path.

#### Scenario: Contributor runs Rust tooling from the repository root
- **WHEN** a contributor runs Rust project commands from the repository root
- **THEN** the root `Cargo.toml` and root `src/` tree define the production touchpad gesture application

#### Scenario: Production code no longer depends on a nested Rust project path
- **WHEN** repository scripts or documentation refer to the deployable application
- **THEN** they use root-relative Rust project paths instead of `linux-touchpad-gesture/`

### Requirement: Prototype Python entrypoints are removed from the supported product layout
The repository SHALL not ship the Python prototype as a supported runtime or setup path once the Rust project becomes the root product layout.

#### Scenario: Repository contents no longer advertise a Python product path
- **WHEN** a contributor inspects the repository root for the supported implementation
- **THEN** they do not find root-level Python daemon entrypoints or Python packaging/setup files that present the prototype as the product

#### Scenario: Supported setup commands are Rust-based
- **WHEN** documentation or scripts describe how to build or run the project
- **THEN** they reference Rust and Cargo workflows rather than `uv`, virtualenv, or direct Python daemon execution

### Requirement: Root documentation describes the Rust workflow
Top-level documentation SHALL describe the Rust application at the repository root and MUST provide accurate build, run, install, and uninstall guidance for that layout.

#### Scenario: README documents root-level development commands
- **WHEN** a user follows the top-level README
- **THEN** the documented commands target the root Rust project for build and execution

#### Scenario: README documents the production deployment path
- **WHEN** a user follows installation or removal instructions from the top-level README
- **THEN** those instructions reference the production root `install.sh` and `uninstall.sh` flow and its associated service and udev assets

### Requirement: Root install and uninstall scripts preserve the Rust deployment contract
The production install and uninstall scripts at the repository root SHALL remain a matched pair that manages only the Rust deployment assets while preserving the daemon's least-privilege user-service model.

#### Scenario: Install script provisions the root Rust deployment
- **WHEN** a user runs the root production install script
- **THEN** it builds the Rust binary from the repository root, installs the user service, and installs the managed udev rule using root-relative asset paths

#### Scenario: Uninstall script tears down only installed Rust assets
- **WHEN** a user runs the root production uninstall script
- **THEN** it removes the service and udev rule created by the install script without deleting unrelated user or system state
