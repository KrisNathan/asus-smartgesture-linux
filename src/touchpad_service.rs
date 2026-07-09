use evdev::{AbsoluteAxisCode, Device, EventSummary, KeyCode, SynchronizationCode};
use std::collections::HashMap;
use std::os::unix::io::{AsFd, BorrowedFd};
use std::time::{Duration, Instant};

use crate::debug_log;
use crate::{
    audio::AudioService,
    brightness::BrightnessService,
    conf::{Conf, ConfService},
    logging::debug_enabled,
    media::MediaService,
};

const MIN_SERVICE_CALL_INTERVAL: Duration = Duration::from_millis(50);

enum TouchpadActionMode {
    Volume,
    Brightness,
    Media,
}

enum PendingAction {
    Volume(f64),
    Brightness(f64),
    Media(i64),
}

#[derive(Clone, Copy)]
struct TouchpadBounds {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    height: i32,
}

impl TouchpadBounds {
    fn width(&self) -> i32 {
        self.max_x - self.min_x
    }
}

fn check_touchpad(device: &Device) -> bool {
    device.supported_absolute_axes().is_some_and(|axes| {
        let has_x = axes.contains(AbsoluteAxisCode::ABS_X)
            || axes.contains(AbsoluteAxisCode::ABS_MT_POSITION_X);
        let has_y = axes.contains(AbsoluteAxisCode::ABS_Y)
            || axes.contains(AbsoluteAxisCode::ABS_MT_POSITION_Y);

        has_x && has_y
    })
}

fn get_touchpad_devices() -> Result<Device, Box<dyn std::error::Error>> {
    let mut devices = Vec::new();

    for (_, device) in evdev::enumerate() {
        if !check_touchpad(&device) {
            continue;
        }

        devices.push(device);
    }

    let best_device = devices
        .into_iter()
        .max_by_key(|d| {
            let name = d.name().unwrap_or("");
            let has_touchpad = name.to_lowercase().contains("touchpad");
            let has_buttons = d
                .supported_keys()
                .map_or(false, |k| k.contains(KeyCode::BTN_LEFT));
            (has_touchpad as i32, has_buttons as i32)
        })
        .ok_or("No touchpad devices found.")?;

    Ok(best_device)
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
            AbsoluteAxisCode::ABS_MT_POSITION_X if min_x.is_none() || max_x.is_none() => {
                min_x = Some(info.minimum());
                max_x = Some(info.maximum());
            }
            AbsoluteAxisCode::ABS_Y => {
                min_y = Some(info.minimum());
                max_y = Some(info.maximum());
            }
            AbsoluteAxisCode::ABS_MT_POSITION_Y if min_y.is_none() || max_y.is_none() => {
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

fn get_action_mode(
    bounds: &TouchpadBounds,
    conf: &Conf,
    x: f64,
    y: f64,
) -> Option<TouchpadActionMode> {
    let width = bounds.max_x - bounds.min_x;
    let height = bounds.height;

    let percent_x = if width > 0 {
        (x - bounds.min_x as f64) / width as f64
    } else {
        0.0
    };

    let percent_y = if height > 0 {
        (y - bounds.min_y as f64) / height as f64
    } else {
        0.0
    };

    // Check left/right edges first (preserve existing volume/brightness behavior)
    if percent_x <= conf.left_edge_threshold_percent {
        return Some(TouchpadActionMode::Volume);
    }

    if percent_x >= conf.right_edge_threshold_percent {
        return Some(TouchpadActionMode::Brightness);
    }

    // Only check top edge if not on left/right edges
    if percent_y <= conf.top_edge_threshold_percent {
        return Some(TouchpadActionMode::Media);
    }

    None
}

struct ActiveTouch {
    x: Option<i32>,
    y: Option<i32>,
    action: Option<TouchpadActionMode>,
    action_decided: bool,
    last_y: Option<i32>,
    last_x: Option<i32>,
}

pub struct TouchpadService<'a, CS, AS, BS, MS>
where
    CS: ConfService,
    AS: AudioService,
    BS: BrightnessService,
    MS: MediaService,
{
    conf: &'a CS,
    device: Device,
    audio_service: &'a AS,
    brightness_service: &'a BS,
    media_service: &'a MS,

    bounds: TouchpadBounds,

    current_slot: i32,
    active_touches: HashMap<i32, ActiveTouch>,
    active_fingers: i32,

    accumulated_delta_volume: f64,
    accumulated_delta_brightness: f64,
    accumulated_delta_media: f64,

    last_volume_call: Option<Instant>,
    last_brightness_call: Option<Instant>,
    last_media_call: Option<Instant>,

    /// After SYN_DROPPED, ignore events until the next SYN_REPORT (may span batches).
    skip_until_syn_report: bool,
}

impl<'a, CS, AS, BS, MS> TouchpadService<'a, CS, AS, BS, MS>
where
    CS: ConfService,
    AS: AudioService,
    BS: BrightnessService,
    MS: MediaService,
{
    fn check_rate_limit(last_call: &mut Option<Instant>) -> bool {
        let now = Instant::now();
        if let Some(last) = *last_call
            && now.duration_since(last) < MIN_SERVICE_CALL_INTERVAL
        {
            return false;
        }
        *last_call = Some(now);
        true
    }

    pub fn new(
        conf: &'a CS,
        audio_service: &'a AS,
        brightness_service: &'a BS,
        media_service: &'a MS,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let device = get_touchpad_devices()?;
        let bounds = get_touchpad_bounds(&device)?;

        Ok(TouchpadService {
            conf,
            device,
            audio_service,
            brightness_service,
            media_service,
            bounds,
            current_slot: 0,
            active_touches: HashMap::new(),
            active_fingers: 0,
            accumulated_delta_volume: 0.0,
            accumulated_delta_brightness: 0.0,
            accumulated_delta_media: 0.0,
            last_volume_call: None,
            last_brightness_call: None,
            last_media_call: None,
            skip_until_syn_report: false,
        })
    }

    pub fn init_debug(&self) {
        if !debug_enabled() {
            return;
        }

        if let Err(error) = get_touchpad_devices() {
            println!("{error}");
            return;
        }

        let device = &self.device;
        println!("Using touchpad: {}", device.name().unwrap_or("Unknown"));

        println!(
            "Touchpad bounds: X[{}, {}], Y[{}, {}]",
            self.bounds.min_x, self.bounds.max_x, self.bounds.min_y, self.bounds.max_y
        );
    }

    fn hard_resync(&mut self) {
        self.active_touches.clear();
        self.active_fingers = 0;
        self.current_slot = 0;
        self.accumulated_delta_volume = 0.0;
        self.accumulated_delta_brightness = 0.0;
        self.accumulated_delta_media = 0.0;
        self.skip_until_syn_report = false;
    }

    pub fn reopen_device(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Clear gesture state even if reopen fails, so a dead fd cannot keep driving actions.
        self.hard_resync();
        let device = get_touchpad_devices()?;
        let bounds = get_touchpad_bounds(&device)?;
        self.device = device;
        self.bounds = bounds;
        Ok(())
    }

    fn apply_pending(&mut self, pending: Vec<PendingAction>) {
        for action in pending {
            match action {
                PendingAction::Volume(delta) => {
                    if let Err(error) = self.audio_service.adjust_volume(&delta) {
                        eprintln!("volume adjust failed: {error}");
                        self.accumulated_delta_volume += delta;
                    }
                }
                PendingAction::Brightness(delta) => {
                    if let Err(error) = self.brightness_service.adjust_brightness(&delta) {
                        eprintln!("brightness adjust failed: {error}");
                        self.accumulated_delta_brightness += delta;
                    }
                }
                PendingAction::Media(seek_offset) => {
                    if let Err(error) = self.media_service.seek(seek_offset) {
                        eprintln!("media seek failed: {error}");
                        self.accumulated_delta_media += seek_offset as f64;
                    }
                }
            }
        }
    }

    pub fn fetch_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let bounds = self.bounds;
        let conf = self.conf.get_conf()?.clone();
        let mut pending = Vec::new();

        for event in self.device.fetch_events()? {
            match event.destructure() {
                EventSummary::Synchronization(_, SynchronizationCode::SYN_DROPPED, _) => {
                    // Inline clear: cannot call hard_resync while device is borrowed by the iterator.
                    self.active_touches.clear();
                    self.active_fingers = 0;
                    self.current_slot = 0;
                    self.accumulated_delta_volume = 0.0;
                    self.accumulated_delta_brightness = 0.0;
                    self.accumulated_delta_media = 0.0;
                    pending.clear();
                    // Ignore the rest of this torn frame until the next SYN_REPORT.
                    self.skip_until_syn_report = true;
                }
                EventSummary::Synchronization(_, SynchronizationCode::SYN_REPORT, _)
                    if self.skip_until_syn_report =>
                {
                    self.skip_until_syn_report = false;
                }
                _ if self.skip_until_syn_report => {}
                EventSummary::Key(_, key, value) => {
                    let mut new_fingers = self.active_fingers;
                    match key {
                        KeyCode::BTN_TOOL_FINGER => {
                            if value == 1 {
                                new_fingers = 1;
                            } else if value == 0 && self.active_fingers == 1 {
                                new_fingers = 0;
                            }
                        }
                        KeyCode::BTN_TOOL_DOUBLETAP => {
                            if value == 1 {
                                new_fingers = 2;
                            } else if value == 0 && self.active_fingers == 2 {
                                new_fingers = 0;
                            }
                        }
                        KeyCode::BTN_TOOL_TRIPLETAP => {
                            if value == 1 {
                                new_fingers = 3;
                            } else if value == 0 && self.active_fingers == 3 {
                                new_fingers = 0;
                            }
                        }
                        KeyCode::BTN_TOOL_QUADTAP => {
                            if value == 1 {
                                new_fingers = 4;
                            } else if value == 0 && self.active_fingers == 4 {
                                new_fingers = 0;
                            }
                        }
                        KeyCode::BTN_TOOL_QUINTTAP => {
                            if value == 1 {
                                new_fingers = 5;
                            } else if value == 0 && self.active_fingers == 5 {
                                new_fingers = 0;
                            }
                        }
                        _ => {}
                    }
                    if new_fingers > 1 && self.active_fingers <= 1 {
                        for touch in self.active_touches.values_mut() {
                            touch.last_y = None;
                            touch.last_x = None;
                        }
                    }
                    self.active_fingers = new_fingers;
                }
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
                                action_decided: false,
                                last_y: None,
                                last_x: None,
                            },
                        );
                    }
                }
                EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_MT_POSITION_X, x) => {
                    debug_log!("ABS_MT_POSITION_X {x}");

                    if self.active_fingers > 1 {
                        continue;
                    }

                    if let Some(touch) = self.active_touches.get_mut(&self.current_slot) {
                        touch.x = Some(x);

                        // Only decide action if we have both X and Y coordinates
                        if !touch.action_decided {
                            if let Some(y) = touch.y {
                                touch.action = get_action_mode(&bounds, &conf, x as f64, y as f64);
                                touch.action_decided = true;
                            }
                        }
                    }
                }
                EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_MT_POSITION_Y, y) => {
                    debug_log!("ABS_MT_POSITION_Y {y}");

                    if self.active_fingers > 1 {
                        continue;
                    }

                    if let Some(touch) = self.active_touches.get_mut(&self.current_slot) {
                        touch.y = Some(y);

                        // Only decide action if we have both X and Y coordinates
                        if !touch.action_decided {
                            if let Some(x) = touch.x {
                                touch.action = get_action_mode(&bounds, &conf, x as f64, y as f64);
                                touch.action_decided = true;
                            }
                        }

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

                                        if Self::check_rate_limit(&mut self.last_volume_call) {
                                            pending.push(PendingAction::Volume(rounded_delta));
                                            self.accumulated_delta_volume -= rounded_delta;
                                        }
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

                                        if Self::check_rate_limit(&mut self.last_brightness_call) {
                                            pending.push(PendingAction::Brightness(rounded_delta));
                                            self.accumulated_delta_brightness -= rounded_delta;
                                        }
                                    }
                                }
                                touch.last_y = Some(y);
                            }
                            Some(TouchpadActionMode::Media) => {
                                // Media seek uses horizontal movement (X axis)
                                if let (Some(current_x), Some(last_x)) = (touch.x, touch.last_x) {
                                    let dx = current_x - last_x;
                                    let fractional_dx = dx as f64 / bounds.width() as f64;
                                    let adjusted_dx = fractional_dx * conf.sensitivity;

                                    // Convert pad travel into seek microseconds so the
                                    // accumulator and pending value match the IPC units,
                                    // same pattern as volume/brightness.
                                    if conf.media_step > 0.0 {
                                        self.accumulated_delta_media += (adjusted_dx
                                            / conf.media_step)
                                            * conf.seek_step_microseconds as f64;
                                    }

                                    let seek_step = conf.seek_step_microseconds as f64;
                                    if seek_step > 0.0
                                        && self.accumulated_delta_media.abs() >= seek_step
                                    {
                                        let media_steps =
                                            (self.accumulated_delta_media / seek_step) as i64;
                                        let seek_offset = media_steps * conf.seek_step_microseconds;

                                        if Self::check_rate_limit(&mut self.last_media_call) {
                                            pending.push(PendingAction::Media(seek_offset));
                                            self.accumulated_delta_media -= seek_offset as f64;
                                        }
                                    }
                                }
                                if let Some(current_x) = touch.x {
                                    touch.last_x = Some(current_x);
                                }
                            }
                            None => {}
                        }
                    }
                }
                _ => {}
            }
        }

        self.apply_pending(pending);
        Ok(())
    }
}

impl<'a, CS, AS, BS, MS> AsFd for TouchpadService<'a, CS, AS, BS, MS>
where
    CS: ConfService,
    AS: AudioService,
    BS: BrightnessService,
    MS: MediaService,
{
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.device.as_fd()
    }
}
