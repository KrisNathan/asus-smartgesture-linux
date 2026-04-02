## Context

The current audio service implementation uses `wpctl` (WirePlumber control utility) to adjust system volume. This approach requires spawning external processes and, when running as a different user, involves complex environment variable setup with `sudo`. 

The Rust ecosystem provides mature D-Bus libraries (e.g., `zbus`) that enable direct communication with audio servers. Both PulseAudio and PipeWire expose D-Bus interfaces for volume control, making a native D-Bus implementation possible.

**Current state:**
- `WpctlAudioService` shells out to `wpctl set-volume`
- When `SUDO_USER` is set, constructs sudo command with explicit D-Bus environment variables
- Trait-based design (`AudioService`) provides abstraction but only one implementation exists

**Constraints:**
- Must maintain backward compatibility with existing `wpctl` implementation
- Cannot break existing configurations or user workflows
- Must run as normal user (no root privilege escalation)
- Must work on KDE Plasma with Wayland and PipeWire (primary target environment)

## Goals / Non-Goals

**Goals:**
- Provide a native D-Bus implementation of `AudioService` that eliminates external process spawning
- Support both PulseAudio and PipeWire audio servers
- Allow user-configurable selection between `wpctl` and D-Bus backends
- Reduce latency and improve reliability of volume adjustments
- Simplify the codebase by removing sudo/environment variable complexity from the D-Bus path

**Non-Goals:**
- Replacing or removing the `wpctl` implementation (remains as default/fallback)
- Supporting audio servers other than PulseAudio/PipeWire
- Implementing advanced audio features beyond volume control (e.g., mute, sink switching)
- Providing a GUI for backend selection (configuration file only)

## Decisions

### Decision 1: Use `zbus` for D-Bus communication
**Rationale:** `zbus` is a pure Rust, async-first D-Bus library with excellent ergonomics and active maintenance. It provides both high-level and low-level APIs, allowing us to start simple and optimize if needed.

**Alternatives considered:**
- `dbus-rs`: Older, C bindings-based library. More mature but less ergonomic and harder to use in async contexts.
- `rustbus`: Pure Rust but lower-level. Would require more boilerplate for our use case.

**Choice:** `zbus` for its balance of ergonomics, safety, and feature completeness.

### Decision 2: Target PulseAudio D-Bus interface primarily
**Rationale:** PipeWire provides PulseAudio compatibility via `pipewire-pulse`, exposing the same D-Bus interface. This means a single implementation can work for both audio servers.

**Alternatives considered:**
- Separate implementations for PulseAudio and PipeWire: More complex, would require runtime detection and branching.
- PipeWire-only via native API: Would not support PulseAudio systems.

**Choice:** Target PulseAudio D-Bus API, which works transparently with PipeWire's compatibility layer.

### Decision 3: Keep `wpctl` as the default backend
**Rationale:** Minimize risk of breaking changes. Users opt into the new D-Bus backend explicitly via configuration. This provides a safe rollback path and allows gradual migration.

**Alternatives considered:**
- Make D-Bus the default: Higher risk; could break working setups if D-Bus has compatibility issues.
- Auto-detect and choose best backend: Complex logic, harder to debug, less predictable.

**Choice:** Explicit configuration with `wpctl` as default preserves stability.

### Decision 4: Configuration via `audio_backend` field
**Rationale:** Add a simple enum field to the existing configuration structure. Values: `wpctl` (default) or `dbus`. This follows the existing configuration pattern and requires minimal changes.

**Implementation:**
```rust
// In config.rs
pub enum AudioBackend {
    Wpctl,
    Dbus,
}

// Default to wpctl for backward compatibility
impl Default for AudioBackend {
    fn default() -> Self {
        AudioBackend::Wpctl
    }
}
```

### Decision 5: Error handling via `std::io::Error`
**Rationale:** The `AudioService` trait already uses `Result<(), std::io::Error>`. To maintain trait compatibility, D-Bus errors must be converted to `std::io::Error`.

**Alternatives considered:**
- Change trait to use a custom error type: Breaking change, affects all implementations.
- Use `anyhow::Error`: More flexible but still requires trait change.

**Choice:** Convert D-Bus errors to `std::io::Error` using `ErrorKind::Other` with descriptive messages.

## Risks / Trade-offs

**[Risk] D-Bus interface changes between PulseAudio versions**
→ **Mitigation:** Test on multiple PulseAudio/PipeWire versions. Document minimum supported versions in README. Keep `wpctl` as fallback.

**[Risk] D-Bus session bus not available or accessible**
→ **Mitigation:** Return clear error on connection failure. Installation instructions verify D-Bus setup. Configuration allows falling back to `wpctl`.

**[Risk] Volume delta calculation differences between backends**
→ **Mitigation:** Ensure both backends interpret delta values identically. Add integration tests comparing wpctl and D-Bus outputs for same inputs.

**[Risk] Async D-Bus calls may introduce latency or blocking issues**
→ **Mitigation:** Use `zbus`'s blocking API or carefully handle async in the sync trait method. Measure latency in testing to ensure it's acceptable for real-time gestures.

**[Trade-off] Additional dependency increases binary size**
→ **Acceptance:** `zbus` adds ~200KB to binary size but provides significant reliability and maintainability benefits.

**[Trade-off] Configuration complexity increases**
→ **Acceptance:** One additional config field is minimal added complexity. Default behavior unchanged for existing users.

## Migration Plan

**Phase 1: Implementation**
1. Add `zbus` to `Cargo.toml`
2. Create `src/audio/dbus_audio_service.rs` implementing `AudioService`
3. Add `AudioBackend` enum to configuration
4. Update service initialization to select backend based on config

**Phase 2: Testing**
1. Unit tests for D-Bus audio service
2. Integration tests comparing wpctl and D-Bus behavior
3. Manual testing on KDE Plasma + Wayland + PipeWire
4. Test error paths (no D-Bus, no default sink, etc.)

**Phase 3: Documentation**
1. Update README with D-Bus backend option
2. Document configuration field in example config
3. Add troubleshooting section for D-Bus issues
4. Update `install.sh` notes if needed (D-Bus should already be available)

**Phase 4: Rollout**
1. Release as experimental feature with clear documentation
2. Gather user feedback
3. Consider making D-Bus default in future release if stable

**Rollback strategy:**
- Users can always switch back to `wpctl` via configuration
- No breaking changes to existing installations
- Both implementations maintained in parallel

## Open Questions

1. **Should we support explicit sink selection in the future?**
   - Currently targets default sink only
   - Future enhancement could allow per-gesture sink configuration
   - Not needed for MVP

2. **Should we expose mute/unmute functionality?**
   - Current `AudioService` trait only supports volume adjustment
   - D-Bus supports mute, but would require trait extension
   - Defer to future enhancement if users request it

3. **Should we implement retry logic for transient D-Bus failures?**
   - Volume gestures happen frequently; retries could mask issues
   - Failing fast provides clearer feedback
   - Decision: No automatic retries in MVP; log errors clearly
