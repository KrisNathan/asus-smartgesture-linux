## Context

The touchpad gesture daemon currently supports volume control via `wpctl` and brightness control via KDE D-Bus when the user swipes vertically on the left or right edges. The system uses a service-oriented architecture with dedicated modules for audio and brightness control.

The existing edge detection only monitors left and right edges (configurable via `left_edge_threshold_percent` and `right_edge_threshold_percent`). The gesture processor tracks finger count to ensure only single-finger gestures trigger actions.

MPRIS (Media Player Remote Interfacing Specification) is a standard D-Bus interface that most Linux media players implement. The `Seek` method accepts an offset in microseconds and adjusts playback position relative to the current position.

## Goals / Non-Goals

**Goals:**
- Add top-edge gesture detection for horizontal swipes
- Implement MPRIS D-Bus client to send seek commands to the active media player
- Maintain architectural consistency with existing audio and brightness services
- Support configurable seek step amount
- Preserve single-finger gesture isolation

**Non-Goals:**
- Selecting specific media players (always use the first available MPRIS player)
- Play/pause or other media controls beyond seek
- Handling multiple concurrent media players (use the first one found)
- Bottom-edge gestures (reserve for future use)

## Decisions

### Decision 1: MPRIS D-Bus Integration Approach
**Choice**: Use `dbus` crate with direct `dbus-send`-style interface calls
**Rationale**: The daemon already uses D-Bus for brightness control via KDE. Using the same `dbus` crate maintains dependency consistency. The MPRIS interface is simple enough that we don't need a specialized MPRIS library.
**Alternative Considered**: Use `mpris` crate - adds another dependency for minimal benefit since we only need the Seek method.

### Decision 2: Service Architecture
**Choice**: Create new `media` module with `MediaService` trait and `MprisMediaService` implementation
**Rationale**: Follows the established pattern used for `audio` (AudioService/WpctlAudioService) and `brightness` (BrightnessService/KdeDbuseBrightnessService). Makes testing easier and allows future alternative implementations.
**Alternative Considered**: Inline MPRIS calls in main gesture handler - would break architectural consistency and make testing harder.

### Decision 3: Edge Detection Strategy
**Choice**: Add `top_edge_threshold_percent` config field and check Y-coordinate proximity to 0
**Rationale**: Symmetric with existing left/right edge detection logic. Users can configure sensitivity the same way.
**Alternative Considered**: Fixed threshold - reduces flexibility for different touchpad sizes.

### Decision 4: Seek Amount Configuration
**Choice**: Add `seek_step_microseconds` config field (default: 10,000,000 = 10 seconds)
**Rationale**: MPRIS Seek method takes microseconds. Users familiar with the example command will understand the unit. Allows fine-grained control.
**Alternative Considered**: `seek_step_seconds` - would require conversion and lose precision for sub-second seeks.

### Decision 5: Media Player Discovery
**Choice**: Query D-Bus for names matching `org.mpris.MediaPlayer2.*` pattern and use the first one
**Rationale**: Simple and works for single-player scenarios (most common use case). If no players are running, log a warning and no-op.
**Alternative Considered**: Remember last-used player - adds state complexity for marginal benefit.

### Decision 6: Gesture Direction Mapping
**Choice**: Right swipe = positive offset (seek forward), left swipe = negative offset (seek backward)
**Rationale**: Natural mapping consistent with media player UI conventions (timeline moves right for forward playback).

## Risks / Trade-offs

**Risk**: User has multiple media players running → **Mitigation**: First-match behavior is deterministic based on D-Bus name order. Document this limitation. Future enhancement could add player selection.

**Risk**: No MPRIS players running when gesture is performed → **Mitigation**: Detect this case and log a debug message. Fail silently to avoid disrupting the user's workflow.

**Risk**: MPRIS D-Bus call fails or times out → **Mitigation**: Set reasonable timeout (e.g., 500ms) and log errors. Ensure failure doesn't crash the daemon.

**Trade-off**: Adding D-Bus dependency for MPRIS → Already depends on `dbus` crate for brightness, so no new dependency cost.

**Trade-off**: Top edge less accessible than side edges on some touchpads → User can adjust `top_edge_threshold_percent` to increase the detection zone. This is configurable UX, not a technical limitation.

## Migration Plan

No migration required - this is a purely additive change. Existing configurations will continue to work with default values for new fields (`top_edge_threshold_percent` defaults to 0.1, `seek_step_microseconds` defaults to 10,000,000).

Users who want to enable this feature can add the new config fields to `~/.config/asus-touchpad-gesture.toml`.

## Open Questions

None - design is ready for implementation.
