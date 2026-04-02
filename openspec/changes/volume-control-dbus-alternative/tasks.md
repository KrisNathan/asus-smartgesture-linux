## 1. Dependencies and Configuration

- [ ] 1.1 Add `zbus` dependency to Cargo.toml with appropriate version
- [ ] 1.2 Add `AudioBackend` enum to configuration structure in `src/conf/config.rs`
- [ ] 1.3 Implement `Default` trait for `AudioBackend` to default to `Wpctl`
- [ ] 1.4 Add deserialization support for `AudioBackend` enum from config file

## 2. D-Bus Audio Service Implementation

- [ ] 2.1 Create new module `src/audio/dbus_audio_service.rs`
- [ ] 2.2 Implement struct `DbusAudioService` with necessary fields (D-Bus connection)
- [ ] 2.3 Implement `AudioService::new()` to establish D-Bus session connection
- [ ] 2.4 Implement connection error handling with descriptive error messages
- [ ] 2.5 Implement default sink identification via D-Bus introspection
- [ ] 2.6 Implement `adjust_volume` method to call PulseAudio D-Bus volume methods
- [ ] 2.7 Add delta-to-absolute volume conversion logic matching wpctl behavior
- [ ] 2.8 Convert D-Bus errors to `std::io::Error` for trait compliance

## 3. Module Integration

- [ ] 3.1 Export `DbusAudioService` in `src/audio/mod.rs`
- [ ] 3.2 Update service initialization in main daemon to select backend based on config
- [ ] 3.3 Add conditional compilation or runtime selection logic for audio backend
- [ ] 3.4 Ensure both backends can coexist without conflicts

## 4. Error Handling and Edge Cases

- [ ] 4.1 Handle case when D-Bus session bus is unavailable
- [ ] 4.2 Handle case when no default audio sink is configured
- [ ] 4.3 Handle zero delta volume adjustment gracefully
- [ ] 4.4 Add appropriate error logging for D-Bus failures
- [ ] 4.5 Verify error messages are actionable for users

## 5. Testing

- [ ] 5.1 Add unit tests for delta-to-volume conversion logic
- [ ] 5.2 Add integration test creating D-Bus service instance (if D-Bus available)
- [ ] 5.3 Test error paths (connection failure, no default sink)
- [ ] 5.4 Manual testing: verify volume increases with positive delta
- [ ] 5.5 Manual testing: verify volume decreases with negative delta
- [ ] 5.6 Manual testing: compare wpctl and D-Bus backends produce equivalent results
- [ ] 5.7 Run `cargo fmt` and `cargo check` to verify code quality

## 6. Documentation

- [ ] 6.1 Update README.md with D-Bus backend option and configuration example
- [ ] 6.2 Document `audio_backend` configuration field with valid values
- [ ] 6.3 Add troubleshooting section for D-Bus connection issues
- [ ] 6.4 Update example configuration file with `audio_backend` field commented
- [ ] 6.5 Add inline code comments for complex D-Bus interactions
- [ ] 6.6 Document minimum PulseAudio/PipeWire versions if applicable

## 7. Validation

- [ ] 7.1 Run full test suite with `cargo test`
- [ ] 7.2 Build release binary and verify size increase is acceptable
- [ ] 7.3 Test on target environment (KDE Plasma + Wayland + PipeWire)
- [ ] 7.4 Verify backward compatibility: existing configs still work with wpctl default
- [ ] 7.5 Test configuration switching: can toggle between backends via config change
