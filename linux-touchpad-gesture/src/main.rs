mod audio;
use audio::{AudioService, WpctlAudioService};

mod conf;
use conf::{ConfService, StaticConfService};

mod touchpad_service;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audio_service = Box::new(WpctlAudioService::new());
    let conf_service = Box::new(StaticConfService::new());

    let mut touchpad_service = touchpad_service::TouchpadService::new(conf_service, audio_service);

    touchpad_service.init_debug();

    // Ok(())

    loop {
        touchpad_service.fetch_events()?;
    }

    // let mut devices = get_touchpad_devices();
    // if devices.is_empty() {
    //     println!("No touchpad devices found.");
    //     return Ok(());
    // }

    // let mut device = devices.remove(0);
    // println!("Using touchpad: {}", device.name().unwrap_or("Unknown"));

    // let bounds = get_touchpad_bounds(&device)?;

    // print!(
    //     "Touchpad bounds: X[{}, {}], Y[{}, {}]\n",
    //     bounds.min_x, bounds.max_x, bounds.min_y, bounds.max_y
    // );

    // evdev::enumerate().for_each(|(path, device)| {
    //     println!("{}: {}", path.display(), device.name().unwrap_or("Unknown"));
    // });

    // let f = File::open("/dev/input/event4")?;
    // let fd = OwnedFd::from(f);
    // let mut device = Device::from_fd(fd)?;

    // let has_touchpad = check_touchpad(&device);
    // if has_touchpad {
    //     println!("Touchpad detected: {}", device.name().unwrap_or("Unknown"));
    // } else {
    //     println!("No touchpad detected.");
    //     return Ok(());
    // }

    // loop {
    //     for event in device.fetch_events()? {
    //         match event.destructure() {
    //             EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_X, x) => {
    //                 println!("X position: {}", x);
    //             }
    //             EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_Y, y) => {
    //                 println!("Y position: {}", y);
    //             }
    //             EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_MT_TRACKING_ID, id) => {
    //                 if id == -1 {
    //                     println!("Touch ended");
    //                 } else {
    //                     println!("Touch started: {}", id);
    //                 }
    //             }
    //             _ => {}
    //         }
    //     }
    // }

    // let device = Device::open("/dev/input/event2")?;
    // check if the device has an ENTER key
    // if device.supported_keys().map_or(false, |keys| keys.contains(KeyCode::KEY_ENTER)) {
    //     println!("are you prepared to ENTER the world of evdev?");
    // } else {
    //     println!(":(");
    // }
}
