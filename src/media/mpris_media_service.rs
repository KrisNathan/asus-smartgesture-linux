use std::{cell::RefCell, env, process::Command};

use zbus::blocking::{Connection, connection::Builder};

use crate::debug_log;
use crate::media::MediaService;

pub struct MprisMediaService {
    conn: Connection,
    /// Cached MPRIS player name to avoid repeated ListNames calls.
    /// Cleared when Seek fails (player disappeared/restarted).
    cached_player: RefCell<Option<String>>,
}

impl MediaService for MprisMediaService {
    fn seek(&self, offset_microseconds: i64) -> Result<(), Box<dyn std::error::Error>> {
        // Try cached player first to avoid ListNames overhead
        if let Some(ref player_name) = *self.cached_player.borrow() {
            if self.try_seek(player_name, offset_microseconds).is_ok() {
                return Ok(());
            }
            // Cached player failed (disappeared/restarted), clear cache
            debug_log!(
                "Cached MPRIS player {} unavailable, re-scanning",
                player_name
            );
            *self.cached_player.borrow_mut() = None;
        }

        // Find an MPRIS media player via ListNames
        let player_name = self.find_mpris_player()?;

        match player_name {
            Some(name) => {
                *self.cached_player.borrow_mut() = Some(name.clone());
                self.try_seek(&name, offset_microseconds)
            }
            None => {
                debug_log!("No MPRIS media player found, skipping seek");
                Ok(())
            }
        }
    }
}

impl MprisMediaService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
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

        Ok(MprisMediaService {
            conn,
            cached_player: RefCell::new(None),
        })
    }

    /// Performs the actual Seek D-Bus call.
    fn try_seek(
        &self,
        player_name: &str,
        offset_microseconds: i64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let object_path = "/org/mpris/MediaPlayer2";
        let interface = "org.mpris.MediaPlayer2.Player";

        let result = self.conn.call_method(
            Some(player_name),
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

    /// Lists MPRIS players on D-Bus. Returns None if none found.
    fn find_mpris_player(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let dbus_proxy = self.conn.call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "ListNames",
            &(),
        )?;

        let names: Vec<String> = dbus_proxy.body().deserialize()?;

        let player_name = names
            .iter()
            .find(|name| name.starts_with("org.mpris.MediaPlayer2."))
            .cloned();

        Ok(player_name)
    }
}
