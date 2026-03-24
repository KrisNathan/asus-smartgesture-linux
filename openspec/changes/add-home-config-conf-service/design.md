## Context

The daemon currently constructs `StaticConfService` in `src/main.rs` and `TouchpadService` calls `get_conf()` inside the event loop, so configuration is effectively reloaded during runtime instead of being captured once at startup. The current `Conf` model is already the right shape for gesture settings, but there is no file-backed implementation and no serialization dependency in `Cargo.toml`.

This change needs to preserve the existing least-privilege user-service model. Configuration must stay in the desktop user's home directory, and the daemon must not require root-owned state or broaden permissions to read settings. It also needs explicit failure behavior: missing config should fall back to current defaults, while a present-but-invalid config file should produce a clear error so users can fix it.

## Goals / Non-Goals

**Goals:**
- Load gesture settings from `$HOME/.config/asus-touchpad-gesture.toml` when that file exists.
- Preserve the static values in `StaticConfService` as the authoritative fallback when the user config file is absent.
- Keep configuration access behind the existing `ConfService` abstraction so touchpad logic stays unchanged.
- Support writing config back to the same TOML path via `save_conf()`.
- Surface actionable read and parse errors for invalid user config files.

**Non-Goals:**
- Changing gesture semantics beyond where settings are sourced.
- Introducing system-wide config files, root-managed config paths, or environment-variable-based overrides.
- Hot-reload coordination beyond the existing `get_conf()` call pattern.
- Redesigning the `TouchpadService` event loop or adding a broader settings UI.

## Decisions

Use a new `FileConfService` implementation alongside `StaticConfService`.
Rationale: the existing trait already isolates configuration access, and a new implementation keeps the change localized to `src/conf/*` plus service wiring in `src/main.rs`. Replacing `StaticConfService` internals directly would blur the distinction between built-in defaults and file-backed behavior. An alternative was to rename the existing service and make it dual-purpose, but that makes fallback behavior harder to reason about and test.

Resolve the config path as `$HOME/.config/asus-touchpad-gesture.toml` from the running user's home directory.
Rationale: the requested path is user-scoped and matches the daemon's least-privilege model. The implementation can use `std::env::var_os("HOME")` or an equivalent home-directory helper, then append `.config/asus-touchpad-gesture.toml`. An alternative was to use XDG base directory discovery, but the requested contract is explicit and should be implemented directly first.

Treat a missing config file as a fallback condition, not an error.
Rationale: this preserves current behavior for all existing installations and lets the daemon operate immediately after upgrade. The file-backed service should delegate to `StaticConfService::get_conf()` when `std::io::ErrorKind::NotFound` is encountered. An alternative was to auto-create the file during startup, but that adds implicit state changes and is unnecessary for the requested behavior.

Treat unreadable or invalid existing config files as explicit errors.
Rationale: the repository rules prefer explicit failures over silent fallback. If the file exists but cannot be opened, decoded, or parsed as TOML, the daemon should return an error identifying the config path and failure reason. An alternative was to ignore invalid files and keep running on defaults, but that would hide broken user configuration.

Use structured serialization for `Conf` with TOML.
Rationale: `serde` plus `toml` is the smallest conventional Rust solution for mapping the existing `Conf` fields to a user-editable file and back. This also keeps `save_conf()` straightforward and avoids ad hoc string parsing. An alternative was a hand-written parser, but that adds risk and maintenance burden for no benefit here.

## Risks / Trade-offs

- [Invalid config stops runtime behavior] → Return clear path-specific errors so users can correct the file quickly instead of silently running with unintended settings.
- [Missing or malformed `HOME` environment handling] → Fail with an actionable error when the home directory cannot be resolved, rather than guessing a path.
- [Repeated file reads inside the event loop add overhead] → Keep the initial implementation simple because `get_conf()` is already called per fetch cycle; if this becomes measurable later, caching can be proposed separately without changing the config contract.
- [Fallback defaults can diverge from file defaults over time] → Keep `StaticConfService` as the single fallback source so default values remain defined in one place.

## Migration Plan

Existing deployments require no config migration because missing-file fallback preserves the current built-in values. After upgrade, users can optionally create `~/.config/asus-touchpad-gesture.toml` to override defaults. Rollback is straightforward: reverting to the previous binary restores static-only behavior, and the user config file can remain in place as inert user state because the old binary simply ignores it.

## Open Questions

None. The requested config path and fallback behavior are specific enough to implement without further product decisions.
