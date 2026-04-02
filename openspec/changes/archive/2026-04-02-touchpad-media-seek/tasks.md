## 1. Configuration

- [x] 1.1 Add `top_edge_threshold_percent` field to `Conf` struct in `src/conf/config.rs` with default value 0.1
- [x] 1.2 Add `seek_step_microseconds` field to `Conf` struct in `src/conf/config.rs` with default value 10,000,000
- [x] 1.3 Update README.md configuration section to document new fields `top_edge_threshold_percent` and `seek_step_microseconds`

## 2. Media Service Module

- [x] 2.1 Create `src/media/mod.rs` with module structure and exports
- [x] 2.2 Create `MediaService` trait in `src/media/mod.rs` with `seek(&self, offset_microseconds: i64) -> Result<(), Box<dyn Error>>` method
- [x] 2.3 Create `src/media/mpris_media_service.rs` implementing MPRIS D-Bus client
- [x] 2.4 Implement media player discovery in `MprisMediaService` by querying D-Bus for `org.mpris.MediaPlayer2.*` pattern
- [x] 2.5 Implement `seek()` method in `MprisMediaService` to call `org.mpris.MediaPlayer2.Player.Seek` with int64 offset
- [x] 2.6 Add 500ms timeout to D-Bus calls in `MprisMediaService`
- [x] 2.7 Add error handling for no players running (log debug message and return Ok)
- [x] 2.8 Add error handling for D-Bus call failures (log error and return Result with error)

## 3. Edge Detection

- [x] 3.1 Add top edge detection logic to gesture processor in `src/touchpad_service.rs` or gesture handler
- [x] 3.2 Implement Y-coordinate check against `top_edge_threshold_percent` (finger Y <= threshold * touchpad height)
- [x] 3.3 Ensure top edge detection respects single-finger isolation (only trigger when finger_count == 1)
- [x] 3.4 Add logic to distinguish top edge from left/right edges (priority or mutual exclusion)

## 4. Gesture Handler Integration

- [x] 4.1 Add `MediaService` instance to main daemon in `src/main.rs`
- [x] 4.2 Wire up top-edge horizontal swipe detection to media seek handler
- [x] 4.3 Implement right swipe → positive offset (seek forward) logic
- [x] 4.4 Implement left swipe → negative offset (seek backward) logic
- [x] 4.5 Calculate seek offset based on swipe distance and `seek_step_microseconds` config
- [x] 4.6 Add finger count check to halt seek operation when second finger is added

## 5. Testing

- [x] 5.1 Manual test: Right swipe on top edge seeks forward in a media player
- [x] 5.2 Manual test: Left swipe on top edge seeks backward in a media player
- [x] 5.3 Manual test: Multi-finger swipe on top edge is ignored
- [x] 5.4 Manual test: Adding second finger during top edge swipe halts seek operation
- [x] 5.5 Manual test: No MPRIS players running logs debug message without crashing
- [x] 5.6 Manual test: Configuration with custom `top_edge_threshold_percent` changes detection zone
- [x] 5.7 Manual test: Configuration with custom `seek_step_microseconds` changes seek amount
- [x] 5.8 Run `cargo fmt` to format code
- [x] 5.9 Run `cargo check` to verify compilation
- [x] 5.10 Run `cargo clippy` to check for warnings

## 6. Documentation

- [x] 6.1 Update README.md with media seek gesture usage examples
- [x] 6.2 Add example configuration showing top edge media seek settings
- [x] 6.3 Document MPRIS dependency requirement (media player must support MPRIS)
