use evdev::{AbsoluteAxisCode, Device, EventSummary, KeyCode};
use std::fs::File;
use std::os::fd::OwnedFd;
use std::process::Command;

static LEFT_EDGE_THRESHOLD_PERCENT: f64 = 0.1;
static RIGHT_EDGE_THRESHOLD_PERCENT: f64 = 0.1;
static SENSITIVITY: f64 = 0.2;
static INVERT_Y: bool = false;

fn check_touchpad(device: &Device) -> bool {
    device
        .supported_absolute_axes()
        .map_or(false, |axes| axes.contains(AbsoluteAxisCode::ABS_X))
}

fn get_touchpad_devices() -> Vec<Device> {
    evdev::enumerate()
        .filter_map(|(path, device)| {
            if check_touchpad(&device) {
                Some(Device::open(path).ok()?)
            } else {
                None
            }
        })
        .collect()
}

struct TouchpadBounds {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

fn get_touchpad_bounds(device: &Device) -> Result<TouchpadBounds, Box<dyn std::error::Error>> {
    let abs_info: Vec<_> = device.get_absinfo()?.collect();

    let mut min_x = None;
    let mut max_x = None;
    let mut min_y = None;
    let mut max_y = None;

    for (axis, info) in device.get_absinfo()? {
        match axis {
            AbsoluteAxisCode::ABS_X => {
                min_x = Some(info.minimum());
                max_x = Some(info.maximum());
            }
            AbsoluteAxisCode::ABS_Y => {
                min_y = Some(info.minimum());
                max_y = Some(info.maximum());
            }
            _ => {}
        }
    }

    match (min_x, max_x, min_y, max_y) {
        (Some(mx), Some(Mx), Some(my), Some(My)) => Ok(TouchpadBounds {
            min_x: mx,
            max_x: Mx,
            min_y: my,
            max_y: My,
        }),
        _ => Err("Touchpad doesn't support X/Y absolute axes".into()),
    }
}

trait AudioService {
    fn adjust_volume(&self, delta: &f64) -> Result<(), std::io::Error>;
}

struct WpctlAudioService;

impl AudioService for WpctlAudioService {
    fn adjust_volume(&self, delta: &f64) -> Result<(), std::io::Error> {
        let percent = delta.abs() * 100.0;
        let sign: char = if delta > &0.0 { '+' } else { '-' };

        // wpctl
        Command::new("wpctl")
            .args(&[
                "set-volume",
                "@DEFAULT_SINK@",
                &format!("{}%{}", percent, sign),
            ])
            .spawn()?;

        Ok(())
    }
}

enum TouchpadActionMode {
    Volume,
    Brightness,
    None,
}

struct TouchpadGestureHandler<'a> {
    device: &'a mut Device,
    audio_service: Box<dyn AudioService>,

    bounds: TouchpadBounds,
}

impl<'a> TouchpadGestureHandler<'a> {
    fn new(device: &'a mut Device, audio_service: Box<dyn AudioService>) -> Self {
        let bounds = get_touchpad_bounds(device).unwrap();
        TouchpadGestureHandler {
            device,
            audio_service,
            bounds: bounds,
        }
    }

    fn get_action_mode(self, x: f64) -> TouchpadActionMode {
        let width = self.bounds.max_x - self.bounds.min_x;
        let height = self.bounds.max_y - self.bounds.min_y;
        let percent_x = if width > 0 {
            (x - self.bounds.min_x as f64) / width as f64
        } else {
            0.0
        };

        if percent_x <= LEFT_EDGE_THRESHOLD_PERCENT {
            TouchpadActionMode::Volume
        } else if percent_x >= 1.0 - RIGHT_EDGE_THRESHOLD_PERCENT {
            TouchpadActionMode::Brightness
        } else {
            TouchpadActionMode::None
        }
    }

    fn event_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            for event in self.device.fetch_events()? {
                match event.destructure() {
                    EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_X, x) => {
                        println!("X position: {}", x);
                    }
                    EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_Y, y) => {
                        println!("Y position: {}", y);
                    }
                    EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_MT_TRACKING_ID, id) => {
                        if id == -1 {
                            println!("Touch ended");
                        } else {
                            println!("Touch started: {}", id);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut devices = get_touchpad_devices();
    if devices.is_empty() {
        println!("No touchpad devices found.");
        return Ok(());
    }

    let mut device = devices.remove(0);
    println!("Using touchpad: {}", device.name().unwrap_or("Unknown"));

    let bounds = get_touchpad_bounds(&device)?;

    print!(
        "Touchpad bounds: X[{}, {}], Y[{}, {}]\n",
        bounds.min_x, bounds.max_x, bounds.min_y, bounds.max_y
    );

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
    println!("Hello, world!");
    Ok(())
}
