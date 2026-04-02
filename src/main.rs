mod audio;
use audio::{AudioService, WpctlAudioService};

mod brightness;
use brightness::{BrightnessService, KDEQDBusBrightnessService};

mod media;
use media::{MediaService, MprisMediaService};

mod conf;
use conf::FileConfService;

mod logging;
mod touchpad_service;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf_service = FileConfService::new();
    let audio_service = WpctlAudioService::new();
    let brightness_service = KDEQDBusBrightnessService::new()?;
    let media_service = MprisMediaService::new()?;

    let mut touchpad_service = touchpad_service::TouchpadService::new(
        &conf_service,
        &audio_service,
        &brightness_service,
        &media_service,
    )?;

    touchpad_service.init_debug();

    loop {
        if let Err(error) = touchpad_service.fetch_events() {
            eprintln!("touchpad event loop error: {error}");
            thread::sleep(Duration::from_millis(250));
        }
    }
}
