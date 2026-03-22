use super::AudioService;
use std::env;
use std::process::Command;

pub struct WpctlAudioService;

impl AudioService for WpctlAudioService {
    fn new() -> Self {
        WpctlAudioService
    }
    fn adjust_volume(&self, delta: &f64) -> Result<(), std::io::Error> {
        let percent = delta.abs() * 100.0;
        let sign: char = if delta > &0.0 { '+' } else { '-' };
        let volume_arg = format!("{}%{}", percent, sign);

        if let Ok(sudo_user) = env::var("SUDO_USER") {
            let uid_output = Command::new("id").args(["-u", &sudo_user]).output()?;
            let uid = String::from_utf8_lossy(&uid_output.stdout)
                .trim()
                .to_owned();

            Command::new("sudo")
                .args([
                    "-u",
                    &sudo_user,
                    "env",
                    &format!("DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/{uid}/bus"),
                    &format!("XDG_RUNTIME_DIR=/run/user/{uid}"),
                    "wpctl",
                    "set-volume",
                    "@DEFAULT_SINK@",
                    &volume_arg,
                ])
                .spawn()?;
        } else {
            Command::new("wpctl")
                .args(["set-volume", "@DEFAULT_SINK@", &volume_arg])
                .spawn()?;
        }

        Ok(())
    }
}
