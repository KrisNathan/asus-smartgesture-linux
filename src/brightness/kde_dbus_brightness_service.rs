use std::{cell::Cell, env, process::Command};

use zbus::{
    blocking::{Connection, connection::Builder},
    proxy,
};

use crate::brightness::BrightnessService;

pub struct KDEQDBusBrightnessService<'a> {
    conn: Connection,
    proxy: KDEPowerManagementProxyBlocking<'a>,
    brightness_max: Cell<Option<i32>>,
}

#[proxy(
    default_service = "org.kde.Solid.PowerManagement",
    default_path = "/org/kde/Solid/PowerManagement/Actions/BrightnessControl",
    interface = "org.kde.Solid.PowerManagement.Actions.BrightnessControl"
)]
trait KDEPowerManagement {
    #[zbus(name = "brightness")]
    fn brightness(&self) -> zbus::Result<i32>;

    #[zbus(name = "brightnessMax")]
    fn brightness_max(&self) -> zbus::Result<i32>;

    #[zbus(name = "setBrightness")]
    fn set_brightness(&self, brightness: i32) -> zbus::Result<()>;
}

impl BrightnessService for KDEQDBusBrightnessService<'_> {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let conn = if let Ok(sudo_user) = env::var("SUDO_USER") {
            let uid_output = Command::new("id").args(["-u", &sudo_user]).output()?;
            let uid = String::from_utf8_lossy(&uid_output.stdout)
                .trim()
                .to_owned();
            let address = format!("unix:path=/run/user/{uid}/bus");

            Builder::address(address.as_str())?.build()?
        } else {
            Connection::session()?
        };

        let proxy = KDEPowerManagementProxyBlocking::new(&conn)?;

        Ok(KDEQDBusBrightnessService {
            conn,
            proxy,
            brightness_max: Cell::new(None),
        })
    }

    fn adjust_brightness(&self, delta: &f64) -> Result<(), Box<dyn std::error::Error>> {
        let max_brightness = match self.brightness_max.get() {
            Some(max) => max,
            None => {
                let max = self.proxy.brightness_max()?;
                self.brightness_max.set(Some(max));
                max
            }
        };

        let current_brightness = self.proxy.brightness()?;

        let delta_rounded = (delta * max_brightness as f64).round() as i32;

        let new_brightness = (current_brightness + delta_rounded).clamp(0, max_brightness);
        self.proxy.set_brightness(new_brightness)?;

        Ok(())
    }
}
