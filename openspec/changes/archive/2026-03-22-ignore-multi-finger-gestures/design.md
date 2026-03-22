## Context

The edge-scrolling gesture for volume and brightness is currently indiscriminately tracking finger position on the touchpad. When a user performs multi-finger DE-level gestures (like 3 or 4 fingers swipe), the touchpad hardware (or the kernel driver) still reports the `ABS_MT_POSITION_X/Y` events for each finger (slot). If one of these fingers falls within the edges defined by the daemon, the volume/brightness changes simultaneously with the DE gesture. We need to ignore these inputs when multiple fingers are touching the touchpad.

## Goals / Non-Goals

**Goals:**
- Reliably detect how many fingers are currently touching the touchpad.
- Suspend the volume and brightness gesture execution when the number of fingers is > 1.
- Immediately halt any active gesture state when an additional finger touches the pad.

**Non-Goals:**
- Creating custom multi-finger gestures.
- Configuring the threshold for finger amount (it will strictly wait for exactly 1 finger for the edge swipe gestures).

## Decisions

- **Use Evdev BTN_TOOL_FINGER events to count active fingers**: 
  - *Rationale*: Evdev provides `BTN_TOOL_FINGER`, `BTN_TOOL_DOUBLETAP`, `BTN_TOOL_TRIPLETAP`, `BTN_TOOL_QUADTAP`, and `BTN_TOOL_QUINTTAP` to indicate the number of active fingers. This is the most robust and standard way to count fingers without manually tracking `ABS_MT_TRACKING_ID` state across multiple slots.
  - *Alternatives considered*: Manually tracking `ABS_MT_TRACKING_ID` > 0 for all `ABS_MT_SLOT`s. This is more tedious and prone to state desync if an event is dropped or misread. `BTN_TOOL_*` is synthesize by the kernel multi-touch driver and is much more reliable.

## Risks / Trade-offs

- [Risk] Some hardware may not emit `BTN_TOOL_DOUBLETAP` etc. and instead only uses multi-touch slots.
  → Mitigation: Most modern touchpads complying with Windows Precision / kernel MT B protocol emit the proper `BTN_TOOL_*` flags. We will rely on these first, but if tracking proves difficult, we can supplement it by tracking slot states.
