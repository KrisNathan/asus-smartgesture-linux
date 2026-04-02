## Why

The current implementation uses `wpctl` for volume control, which requires shelling out to an external command and complex environment setup when running as a different user. A native D-Bus implementation would provide better reliability, lower latency, and cleaner integration with the desktop environment.

## What Changes

- Add a new D-Bus-based implementation of the `AudioService` trait that directly communicates with PulseAudio/PipeWire via D-Bus
- Preserve the existing `WpctlAudioService` as a fallback option
- Allow users to select their preferred audio service implementation via configuration
- Eliminate the need for `sudo` and environment variable manipulation when adjusting volume

## Capabilities

### New Capabilities
- `dbus-audio-service`: Native D-Bus implementation for volume control that communicates directly with the audio server (PulseAudio/PipeWire) without spawning external processes

### Modified Capabilities
<!-- No existing capabilities require specification changes -->

## Impact

- **Code**: New module `src/audio/dbus_audio_service.rs` implementing the `AudioService` trait
- **Dependencies**: Add `zbus` or similar D-Bus library to `Cargo.toml`
- **Configuration**: Extend configuration to allow selecting audio service backend
- **Compatibility**: Maintains backward compatibility by keeping `wpctl` implementation as default/fallback
- **Runtime**: Removes dependency on `wpctl` binary when D-Bus backend is used
