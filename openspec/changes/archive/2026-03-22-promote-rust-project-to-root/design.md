## Context

The repository is in an in-between state: the root contains a runnable Python prototype, Python packaging metadata, and a README that describes the prototype as the primary product, while the deployable Rust implementation already exists under `linux-touchpad-gesture/`. The Rust subproject includes the production-oriented install and uninstall scripts, a user-service deployment model, and the least-privilege udev-based device access model that the repository intends to ship.

This change is primarily a migration of source-of-truth, layout, and operator entrypoints. It touches code layout, deployment scripts, asset paths, and contributor-facing documentation. Because the daemon interacts with input devices and system services, the migration must preserve the existing non-root execution model, keep install and uninstall behavior matched, and avoid leaving stale references to removed files.

## Goals / Non-Goals

**Goals:**
- Make the repository root the production Rust crate so contributors can build, run, and validate the deployable application from the top level.
- Remove the root-level Python prototype files that currently look like supported product entrypoints.
- Replace the root install flow with the Rust production install and uninstall pair without weakening the current security model.
- Update README and related documentation so the Rust project, not the prototype, is the documented operator path.
- Keep the resulting migration easy to validate with root-level `cargo fmt` and `cargo check`.

**Non-Goals:**
- Change gesture behavior, device detection logic, audio or brightness integration, or other runtime daemon features.
- Rework the systemd hardening profile beyond path and naming updates required by the move.
- Introduce a compatibility layer that preserves the Python prototype as a supported runtime option.
- Redesign configuration handling beyond documenting the Rust implementation's current behavior.

## Decisions

### Move the Rust project into the repository root

The Rust crate, `src/` tree, and Rust-specific helper scripts will be promoted into the root rather than wrapped by a workspace or left inside `linux-touchpad-gesture/`.

Rationale:
- The user explicitly wants the Rust project to become the root.
- A real move removes ambiguity about where production work belongs.
- Wrapper Cargo manifests or forwarding scripts would preserve the same conceptual split that is already causing documentation drift.

Alternatives considered:
- Keep the Rust crate in `linux-touchpad-gesture/` and only rewrite the README. Rejected because it leaves the physical layout inconsistent with the documented source of truth.
- Add a root Cargo workspace that points at `linux-touchpad-gesture/`. Rejected because it still keeps production code one level down and complicates scripts for little gain.

### Replace, rather than preserve, the Python prototype path

The Python daemon, `pyproject.toml`, `uv.lock`, and the prototype-oriented root install flow will be removed instead of being retained as an alternate implementation.

Rationale:
- The repository rules treat the Python code as disposable migration reference only.
- Leaving executable Python entrypoints in place would continue to signal that the prototype is a supported product path.
- Git history remains the correct place to recover the prototype if needed for reference.

Alternatives considered:
- Move the prototype to a `prototype/` directory. Rejected because the request is to remove the prototype implementations, and keeping the code in-tree would still invite accidental maintenance.

### Preserve the current Rust deployment contract while relocating it

The root `install.sh` and `uninstall.sh` will become the matched production scripts, and they will continue to build the Rust binary, install the user service, manage the persistent udev rule, and keep the daemon running as the desktop user. Asset paths inside those scripts will be rewritten to root-relative locations, and the uninstall flow will continue to remove only what the installer created.

Rationale:
- The existing Rust install pair already matches the repository safety model better than the prototype setup script.
- The least-privilege model and teardown guarantees are more important than path compatibility.
- Reusing the production logic minimizes behavioral churn during the repository migration.

Alternatives considered:
- Rename service and asset conventions during the same change. Rejected for now because it increases migration scope and is not required to establish the Rust root layout.

### Make root documentation authoritative for the Rust workflow

The top-level README and related guidance will be rewritten around root-level Rust commands such as `cargo run`, `cargo fmt`, `cargo check`, `./install.sh`, and `./uninstall.sh`. Documentation that currently describes Python setup, `uv`, virtual environments, or a generated `config.json` path will be removed or replaced with Rust-accurate guidance.

Rationale:
- The main contributor and operator failure mode today is following obsolete root documentation.
- Aligning commands with the new root layout reduces onboarding and deployment mistakes.

Alternatives considered:
- Keep a mixed README that documents both Python and Rust. Rejected because the repository direction is to discard the prototype.

## Risks / Trade-offs

- [Path-sensitive scripts break after the move] -> Rewrite all root-relative paths in install, uninstall, test, and documentation together, then validate the standard root commands.
- [Installer and uninstaller drift during migration] -> Treat them as a matched pair in the same change and review every created file, service, and rule path together.
- [Users with local habits around `linux-touchpad-gesture/`] -> Document the new root commands clearly and accept that this is a deliberate breaking repository-layout change.
- [Losing easy access to the prototype for comparison] -> Rely on git history instead of shipping duplicate runnable implementations.

## Migration Plan

1. Move the Rust crate files and supporting scripts from `linux-touchpad-gesture/` into the repository root.
2. Merge or replace root assets so `install.sh`, `uninstall.sh`, the udev rule, README, and helper scripts all refer to the root Rust layout.
3. Remove the Python prototype implementation and Python dependency metadata from the repository root.
4. Run root-level validation for formatting and compilation, then verify the documented commands and asset references are consistent.

Rollback is a normal source revert of the migration commit because this change is a repository layout refactor rather than a live data migration.

## Open Questions

None at proposal time. The current plan keeps the existing Rust service naming and focuses the change on layout, removal, and documentation alignment.
