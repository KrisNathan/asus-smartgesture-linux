use evdev::{AbsoluteAxisCode, Device, EventSummary};
use std::collections::HashMap;

use crate::{
    audio::AudioService,
    brightness::BrightnessService,
    conf::{Conf, ConfService},
    logging::debug_enabled,
};
use crate::{brightness, debug_log};

enum TouchpadActionMode {
    Volume,
    Brightness,
}

struct TouchpadBounds {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    height: i32,
}

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

fn get_touchpad_bounds(device: &Device) -> Result<TouchpadBounds, Box<dyn std::error::Error>> {
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
        (Some(min_x), Some(max_x), Some(min_y), Some(max_y)) => Ok(TouchpadBounds {
            min_x,
            max_x,
            min_y,
            max_y,
            height: max_y - min_y,
        }),
        _ => Err("Touchpad doesn't support X/Y absolute axes".into()),
    }
}

fn get_action_mode(bounds: &TouchpadBounds, conf: &Conf, x: f64) -> Option<TouchpadActionMode> {
    let width = bounds.max_x - bounds.min_x;
    let _height = bounds.max_y - bounds.min_y; // currently unused, but may be useful for future features

    let percent_x = if width > 0 {
        (x - bounds.min_x as f64) / width as f64
    } else {
        0.0
    };

    if percent_x <= conf.left_edge_threshold_percent {
        Some(TouchpadActionMode::Volume)
    } else if percent_x >= conf.right_edge_threshold_percent {
        Some(TouchpadActionMode::Brightness)
    } else {
        None
    }
}

struct ActiveTouch {
    x: Option<i32>,
    y: Option<i32>,
    action: Option<TouchpadActionMode>,
    last_y: Option<i32>,
}

pub struct TouchpadService {
    conf: Box<dyn ConfService>,
    device: Device,
    audio_service: Box<dyn AudioService>,
    brightness_service: Box<dyn BrightnessService>,

    bounds: TouchpadBounds,

    current_slot: i32,
    active_touches: HashMap<i32, ActiveTouch>,

    accumulated_delta_volume: f64,
    accumulated_delta_brightness: f64,
}

impl TouchpadService {
    pub fn new(
        conf: Box<dyn ConfService>,
        audio_service: Box<dyn AudioService>,
        brightness_service: Box<dyn BrightnessService>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let devices = get_touchpad_devices();
        if devices.is_empty() {
            return Err("No touchpad devices found.".into());
        }
        let device = devices
            .into_iter()
            .next()
            .ok_or("No touchpad devices found.")?;
        let bounds = get_touchpad_bounds(&device)?;

        Ok(TouchpadService {
            conf,
            device,
            audio_service,
            brightness_service,
            bounds,
            current_slot: 0,
            active_touches: HashMap::new(),
            accumulated_delta_volume: 0.0,
            accumulated_delta_brightness: 0.0,
        })
    }

    pub fn init_debug(&self) {
        if !debug_enabled() {
            return;
        }

        let devices = get_touchpad_devices();
        if devices.is_empty() {
            println!("No touchpad devices found.");
            return;
        }

        let device = &self.device;
        println!("Using touchpad: {}", device.name().unwrap_or("Unknown"));

        print!(
            "Touchpad bounds: X[{}, {}], Y[{}, {}]\n",
            self.bounds.min_x, self.bounds.max_x, self.bounds.min_y, self.bounds.max_y
        );
    }

    pub fn fetch_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let bounds = &self.bounds;
        let conf = self.conf.get_conf()?;

        for event in self.device.fetch_events()? {
            match event.destructure() {
                EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_MT_SLOT, value) => {
                    // slot management is necessary to track multiple touches independently
                    debug_log!("ABS_MT_SLOT {value}");
                    self.current_slot = value;
                }
                EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_MT_TRACKING_ID, id) => {
                    // tracking_id of -1 indicates a touch has ended, otherwise it's a new touch
                    if id == -1 {
                        debug_log!("Touch ended");
                        self.active_touches.remove(&self.current_slot);
                    } else {
                        debug_log!("Touch started: {id}");
                        self.active_touches.insert(
                            self.current_slot,
                            ActiveTouch {
                                x: None,
                                y: None,
                                action: None,
                                last_y: None,
                            },
                        );
                    }
                }
                EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_MT_POSITION_X, x) => {
                    debug_log!("ABS_MT_POSITION_X {x}");

                    if let Some(touch) = self.active_touches.get_mut(&self.current_slot) {
                        touch.x = Some(x);

                        if touch.action.is_none() {
                            touch.action = get_action_mode(bounds, &conf, x as f64);
                        }
                    }
                }
                EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_MT_POSITION_Y, y) => {
                    debug_log!("ABS_MT_POSITION_Y {y}");

                    if let Some(touch) = self.active_touches.get_mut(&self.current_slot) {
                        touch.y = Some(y);

                        match touch.action {
                            Some(TouchpadActionMode::Volume) => {
                                if let Some(last_y) = touch.last_y {
                                    let dy = last_y - y;
                                    let fractional_dy = dy as f64 / bounds.height as f64;
                                    let mut adjusted_dy = fractional_dy * conf.sensitivity;

                                    if conf.invert_y {
                                        adjusted_dy = -adjusted_dy;
                                    }

                                    self.accumulated_delta_volume += adjusted_dy;
                                    if self.accumulated_delta_volume.abs() >= conf.volume_step {
                                        let volume_steps = (self.accumulated_delta_volume
                                            / conf.volume_step)
                                            as i32;
                                        let rounded_delta = volume_steps as f64 * conf.volume_step;

                                        self.audio_service.adjust_volume(&rounded_delta)?;

                                        self.accumulated_delta_volume -= rounded_delta;
                                    }
                                }
                                touch.last_y = Some(y);
                            }
                            Some(TouchpadActionMode::Brightness) => {
                                if let Some(last_y) = touch.last_y {
                                    let dy = last_y - y;
                                    let fractional_dy = dy as f64 / bounds.height as f64;
                                    let mut adjusted_dy = fractional_dy * conf.sensitivity;

                                    if conf.invert_y {
                                        adjusted_dy = -adjusted_dy;
                                    }

                                    self.accumulated_delta_brightness += adjusted_dy;
                                    if self.accumulated_delta_brightness.abs()
                                        >= conf.brightness_step
                                    {
                                        let brightness_steps = (self.accumulated_delta_brightness
                                            / conf.brightness_step)
                                            as i32;
                                        let rounded_delta =
                                            brightness_steps as f64 * conf.brightness_step;

                                        // TODO: brightness service
                                        self.brightness_service
                                            .adjust_brightness(&rounded_delta)?;

                                        self.accumulated_delta_brightness -= rounded_delta;
                                    }
                                }
                                touch.last_y = Some(y);
                            }
                            None => {}
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
