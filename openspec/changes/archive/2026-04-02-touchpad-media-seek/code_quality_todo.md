# Code Quality TODO

Issues identified during code review of the touchpad-media-seek implementation.

## High Priority

### 1. Missing D-Bus Timeout Implementation
**Task Reference:** 2.6 "Add 500ms timeout to D-Bus calls in `MprisMediaService`"

**Location:** `src/media/mpris_media_service.rs:31-37, 58-64`

**Issue:** D-Bus calls lack timeout configuration and could block indefinitely, freezing the gesture processing loop.

**Current Code:**
```rust
let dbus_proxy = self.conn.call_method(
    Some("org.freedesktop.DBus"),
    "/org/freedesktop/DBus",
    Some("org.freedesktop.DBus"),
    "ListNames",
    &(),  // ❌ No timeout configured
)?;
```

**Fix:** Use zbus timeout API or wrap calls with timeout mechanism.

---

### 2. Inefficient D-Bus Player Discovery
**Location:** `src/media/mpris_media_service.rs:31-44`

**Issue:** Queries **all** D-Bus names on **every seek operation**, causing unnecessary overhead.

**Performance Impact:**
- `ListNames` returns hundreds of D-Bus names
- Called multiple times per swipe gesture
- Unnecessary allocations and string comparisons

**Current Approach:**
```rust
// Every seek() call does this:
let names: Vec<String> = dbus_proxy.body().deserialize()?;
let player_name = names.iter().find(|name| name.starts_with("org.mpris.MediaPlayer2."));
```

**Better Approach:**
- Cache discovered player name
- Use D-Bus introspection to query only MPRIS names
- Consider using zbus proxy pattern (like brightness service)

---

### 3. Hardcoded Magic Number for Seek Threshold
**Location:** `src/touchpad_service.rs:420`

**Issue:** Hardcoded `0.1` threshold is inconsistent with configurable `volume_step` and `brightness_step`.

**Current Code:**
```rust
if self.accumulated_delta_media.abs() >= 0.1 {
    // Trigger seek after 10% horizontal movement
```

**Problems:**
- Not configurable by user
- Inconsistent with volume/brightness behavior (which use config steps)
- Magic number without clear justification

**Fix:** Add `media_seek_threshold` to configuration or use existing `sensitivity` field consistently.

---

## Medium Priority

### 4. Error Handling for Seek Failures
**Location:** `src/media/mpris_media_service.rs:71-74`

**Issue:** D-Bus call failures propagate errors up, potentially disrupting gesture processing.

**Current Code:**
```rust
Err(e) => {
    eprintln!("MPRIS seek failed for {}: {}", player_name, e);
    Err(Box::new(e))  // Propagates error
}
```

**Consideration:** Media control is a non-critical feature. Should we:
- Log error and return `Ok(())` (graceful degradation)
- Or keep current behavior (error propagation with recovery in main loop)

**Current main.rs recovery:**
```rust
if let Err(error) = touchpad_service.fetch_events() {
    eprintln!("touchpad event loop error: {error}");
    thread::sleep(Duration::from_millis(250));  // Recovers
}
```

**Verdict:** Current behavior is acceptable but logging and continuing would be more graceful.

---

### 5. Add Explanatory Comments
**Location:** `src/media/mpris_media_service.rs:14-24`

**Issue:** SUDO_USER defensive logic lacks explanation.

**Current Code:**
```rust
let conn = if let Ok(sudo_user) = env::var("SUDO_USER") {
    let uid_output = Command::new("id").args(["-u", &sudo_user]).output()?;
    // ... spawn process, parse output
```

**Why This Exists:**
- Repository rule: "The daemon must run as the normal desktop user, not as root"
- SUDO_USER is only set when running under sudo
- This is defensive code for the edge case where someone runs with sudo despite instructions
- Attempts to connect to the real user's session bus

**Fix:** Add comment explaining this defensive edge case handling.

---

## Low Priority

### 6. Consider Using zbus Proxy Pattern
**Location:** `src/media/mpris_media_service.rs`

**Observation:** Current implementation uses low-level `call_method` API, while brightness service uses cleaner zbus proxy pattern.

**Current Pattern:**
```rust
let result = self.conn.call_method(
    Some(player_name.as_str()),
    object_path,
    Some(interface),
    "Seek",
    &(offset_microseconds),
);
```

**Alternative Pattern (like KDEQDBusBrightnessService):**
```rust
#[proxy(
    interface = "org.mpris.MediaPlayer2.Player"
)]
trait MprisPlayer {
    fn seek(&self, offset: i64) -> zbus::Result<()>;
}
```

**Trade-off:**
- Proxy pattern is cleaner and more type-safe
- But current approach works for dynamic player discovery
- May not be worth refactoring given dynamic nature of player selection

---

## Positive Aspects (No Action Needed)

✓ Proper service architecture following AudioService/BrightnessService pattern
✓ No panics or unsafe code
✓ Single-finger isolation correctly enforced
✓ Edge priority correctly prioritizes side edges over top edge
✓ Type safety with proper enum usage
✓ Graceful handling of missing MPRIS player
✓ Error propagation follows Result-based patterns
