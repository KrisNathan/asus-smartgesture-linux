use std::cell::RefCell;

use evdev::{AttributeSet, EventType, InputEvent, KeyCode, uinput::VirtualDevice};

use crate::debug_log;
use crate::media::MediaService;

pub struct ArrowKeyMediaService {
    device: RefCell<VirtualDevice>,
}

impl ArrowKeyMediaService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let keys = AttributeSet::from_iter([KeyCode::KEY_LEFT, KeyCode::KEY_RIGHT]);
        let device = VirtualDevice::builder()?
            .name("ASUS Touchpad Media Arrow Keys")
            .with_keys(&keys)?
            .build()?;

        Ok(Self {
            device: RefCell::new(device),
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
        // Sign-only: one tap per call. Magnitude is ignored; touchpad fires
        // once per media_step so swipe distance still scales tap count.
        if offset_microseconds == 0 {
            return Ok(());
        }

        let key = if offset_microseconds > 0 {
            KeyCode::KEY_RIGHT
        } else {
            KeyCode::KEY_LEFT
        };

        self.tap_key(key)
    }
}
