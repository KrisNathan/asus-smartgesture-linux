## 1. Input Event Parsing

- [x] 1.1 Update `src/touchpad_service.rs` state tracking to include active finger count.
- [x] 1.2 Add handling for `BTN_TOOL_FINGER`, `BTN_TOOL_DOUBLETAP`, `TRIPLETAP`, `QUADTAP`, and `QUINTTAP` events in the `evdev` loop to update the finger count state.

## 2. Multi-finger Filtering

- [x] 2.1 Refactor the gesture trigger conditions to check if active fingers == 1.
- [x] 2.2 Ignore any position updates (`ABS_MT_POSITION_X`/`Y` or `ABS_X`/`Y`) for gesture handling if active fingers > 1.
- [x] 2.3 Reset active gesture states (e.g., edge-scrolling in-progress) if the finger count increases beyond 1 mid-gesture.

## 3. Testing and Validation

- [x] 3.1 Compile with `cargo check` and `cargo fmt`.
- [x] 3.2 Verify single-finger edge swipe correctly changes volume/brightness.
- [x] 3.3 Verify that a 3 or 4 finger DE gesture attempting to trigger the edge swipe is completely ignored.
- [x] 3.4 Verify that adding a second finger during an active edge swipe immediately halts the volume/brightness adjustment.
