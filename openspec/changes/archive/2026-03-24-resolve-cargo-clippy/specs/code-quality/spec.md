## ADDED Requirements

### Requirement: Clean clippy output

The project SHALL compile with `cargo clippy` producing zero warnings (excluding dead_code warnings for intentionally unused fields and methods).

#### Scenario: No clippy warnings on build

- **WHEN** `cargo clippy` is run against the project
- **THEN** zero warnings are emitted for redundant_field_names, module_inception, unnecessary_map_or, ptr_arg, and print_with_newline lints

### Requirement: Idiomatic Rust patterns

The codebase SHALL follow standard Rust idioms as enforced by clippy.

#### Scenario: Shorthand field initialization

- **WHEN** a struct field name matches the local binding name
- **THEN** the shorthand form is used (e.g., `conn` instead of `conn: conn`)

#### Scenario: Correct slice type for path parameters

- **WHEN** a function accepts a path parameter for reading only
- **THEN** the parameter type is `&Path` rather than `&PathBuf`

#### Scenario: Use println instead of print with newline

- **WHEN** outputting a line with a trailing newline
- **THEN** `println!` is used instead of `print!(.."\n")`
