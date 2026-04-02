## Why

Users need quick media playback control while working at their computers. Currently, the touchpad gesture daemon supports volume and brightness adjustments via edge swipes, but lacks media playback controls. Adding seek forward/backward gestures using the top edge enables hands-on-keyboard control of media without switching to the mouse or keyboard shortcuts.

## What Changes

- Add horizontal swipe gesture detection on the top edge of the touchpad
- Implement MPRIS D-Bus integration for media player control
- Add seek forward capability when swiping right on the top edge
- Add seek backward capability when swiping left on the top edge
- Configure seek amount and top edge threshold in the user configuration file
- Maintain single-finger gesture isolation (consistent with existing edge swipe behavior)

## Capabilities

### New Capabilities
- `media-seek-gestures`: Top-edge horizontal swipe gestures that send seek commands to the active media player via MPRIS D-Bus interface
- `mpris-dbus-integration`: D-Bus client integration for communicating with MPRIS-compatible media players

### Modified Capabilities
- `edge-swipe-gestures`: Extend to support top edge detection in addition to existing left/right edge detection

## Impact

- **Code**: New gesture handler for top-edge detection, new MPRIS D-Bus client module
- **Configuration**: Add `top_edge_threshold_percent` and `seek_step_microseconds` to config file
- **Dependencies**: May require additional D-Bus crate dependency for MPRIS communication
- **Systems**: Requires MPRIS-compatible media player running (e.g., Brave, Firefox, VLC, Spotify)
