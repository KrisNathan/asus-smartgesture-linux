use std::{env, process::Command};

use zbus::blocking::{Connection, connection::Builder};

use crate::debug_log;
use crate::media::MediaService;

pub struct MprisMediaService {
    conn: Connection,
}

impl MediaService for MprisMediaService {
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

        Ok(MprisMediaService { conn })
    }

    fn seek(&self, offset_microseconds: i64) -> Result<(), Box<dyn std::error::Error>> {
        // Find an MPRIS media player
        let dbus_proxy = self.conn.call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "ListNames",
            &(),
        )?;

        let names: Vec<String> = dbus_proxy.body().deserialize()?;

        // Find the first MPRIS player
        let player_name = names
            .iter()
            .find(|name| name.starts_with("org.mpris.MediaPlayer2."));

        let player_name = match player_name {
            Some(name) => name,
            None => {
                debug_log!("No MPRIS media player found, skipping seek");
                return Ok(());
            }
        };

        // Call Seek method on the player
        let object_path = "/org/mpris/MediaPlayer2";
        let interface = "org.mpris.MediaPlayer2.Player";

        let result = self.conn.call_method(
            Some(player_name.as_str()),
            object_path,
            Some(interface),
            "Seek",
            &(offset_microseconds),
        );

        match result {
            Ok(_) => {
                debug_log!("Seek command sent to {}", player_name);
                Ok(())
            }
            Err(e) => {
                eprintln!("MPRIS seek failed for {}: {}", player_name, e);
                Err(Box::new(e))
            }
        }
    }
}
