use std::cell::RefCell;

use evdev::{AttributeSet, EventType, InputEvent, KeyCode, uinput::VirtualDevice};

use crate::debug_log;
use crate::media::MediaService;

pub struct ArrowKeyMediaService {
    device: RefCell<VirtualDevice>,
    seek_step_microseconds: i64,
}

impl ArrowKeyMediaService {
    pub fn new(seek_step_microseconds: i64) -> Result<Self, Box<dyn std::error::Error>> {
        let keys = AttributeSet::from_iter([KeyCode::KEY_LEFT, KeyCode::KEY_RIGHT]);
        let device = VirtualDevice::builder()?
            .name("ASUS Touchpad Media Arrow Keys")
            .with_keys(&keys)?
            .build()?;

        Ok(Self {
            device: RefCell::new(device),
            seek_step_microseconds,
        })
    }

    fn tap_key(&self, key: KeyCode) -> Result<(), Box<dyn std::error::Error>> {
        let mut device = self.device.try_borrow_mut()?;
        let code = key.code();

        device.emit(&[InputEvent::new(EventType::KEY.0, code, 1)])?;
        device.emit(&[InputEvent::new(EventType::KEY.0, code, 0)])?;

        debug_log!("Arrow key media command sent: {:?}", key);
        Ok(())
    }
}

impl MediaService for ArrowKeyMediaService {
    fn seek(&self, offset_microseconds: i64) -> Result<(), Box<dyn std::error::Error>> {
        let key = if offset_microseconds >= 0 {
            KeyCode::KEY_RIGHT
        } else {
            KeyCode::KEY_LEFT
        };

        let taps = if self.seek_step_microseconds == 0 {
            1
        } else {
            (offset_microseconds.unsigned_abs() / self.seek_step_microseconds.unsigned_abs()).max(1)
        };

        for _ in 0..taps {
            self.tap_key(key)?;
        }
        Ok(())
    }
}
