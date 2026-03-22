## Why

Currently, the left and right edge swipe gestures for volume and brightness can trigger accidentally when the user performs multi-finger desktop environment gestures (like 3 or 4 finger swipes for workspace switching or overview). This change ensures that edge swipe gestures only activate when exactly one finger is touching the touchpad, preventing interference with native DE features.

## What Changes

- Gestures will only activate if `libinput` or `evdev` reports exactly 1 active finger on the touchpad.
- If an edge swipe is currently in progress but another finger is detected, the gesture will immediately halt or ignore further events until single-finger contact resumes.
- The volume and brightness adjustment logic will be gated behind a multi-finger check.

## Capabilities

### New Capabilities

- `edge-swipe-gestures`: Touchpad edge scrolling requirements, encompassing volume and brightness adjustment constraints and the new single-finger isolation requirement.

### Modified Capabilities

None.

## Impact

- Input handling logic in the main event loop will be affected to track the total number of fingers touching the pad.
- `evdev` multi-touch slot states (e.g., `ABS_MT_TRACKING_ID`) or tool button codes (`BTN_TOOL_DOUBLETAP`, `TRIPLETAP`, etc.) will need to be monitored to correctly calculate active finger count.
