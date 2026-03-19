mod audio;
use audio::{AudioService, WpctlAudioService};

mod conf;
use conf::{ConfService, StaticConfService};

mod logging;
mod touchpad_service;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audio_service = Box::new(WpctlAudioService::new());
    let conf_service = Box::new(StaticConfService::new());

    let mut touchpad_service =
        touchpad_service::TouchpadService::new(conf_service, audio_service)?;

    touchpad_service.init_debug();

    loop {
        if let Err(error) = touchpad_service.fetch_events() {
            eprintln!("touchpad event loop error: {error}");
            thread::sleep(Duration::from_millis(250));
        }
    }
}
