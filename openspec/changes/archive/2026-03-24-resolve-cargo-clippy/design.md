## Context

The project is a Rust-based touchpad gesture daemon for Linux. Running `cargo clippy` surfaces 5 actionable warnings (excluding dead_code warnings for unused fields/methods which are intentionally out of scope). These warnings are all linter-level code quality issues with no behavioral impact.

## Goals / Non-Goals

**Goals:**
- Resolve all actionable `cargo clippy` warnings to achieve a clean linter run
- Improve code readability by using Rust idioms
- Ensure no behavioral changes are introduced

**Non-Goals:**
- Fixing `dead_code` warnings for unused fields or methods (these may be needed for future features or API completeness)
- Adding new functionality or changing public APIs
- Refactoring beyond what clippy suggests

## Decisions

### 1. Rename `conf/conf` module to `conf/config`

**Rationale**: Clippy's `module_inception` lint flags when a module has the same name as its parent. Renaming `src/conf/conf.rs` → `src/conf/config.rs` (and updating the `mod conf` → `mod config` declaration) eliminates the warning while keeping the public re-export (`pub use config::Conf`) unchanged.

### 2. Use shorthand field initialization in struct literal

**Rationale**: When a field name matches the binding name (`conn: conn`), Rust allows shorthand (`conn`). This is the idiomatic form and eliminates `redundant_field_names` warnings.

### 3. Replace `map_or(false, ...)` with `is_some_and(...)`

**Rationale**: `is_some_and` (stabilized in Rust 1.70) is more readable and directly expresses the intent: "check if the Option contains a value satisfying a predicate." This eliminates the `unnecessary_map_or` warning.

### 4. Change `&PathBuf` to `&Path` in function parameter

**Rationale**: `&Path` is the idiomatic borrowed form for path parameters. `&PathBuf` forces callers to pass a reference to the heap-allocated type when a slice-like `&Path` would suffice. This is the `ptr_arg` lint recommendation.

### 5. Replace `print!(.."\n")` with `println!(..)`

**Rationale**: Using `print!` with a trailing newline is redundant when `println!` exists. This is the `print_with_newline` lint recommendation.

## Risks / Trade-offs

- [Module rename] → Risk of breaking internal `use` paths. Mitigated by updating `mod` declaration and verifying `pub use` still resolves correctly.
- [All changes] → Risk of behavioral regression. Mitigated by verifying each change is purely syntactic/idiomatic with no semantic difference (e.g., `is_some_and` has identical semantics to `map_or(false, ...)` for `Some`/`None`).
