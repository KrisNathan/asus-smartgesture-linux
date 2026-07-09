mod audio;
use audio::{AudioService, WpctlAudioService};

mod brightness;
use brightness::{BrightnessService, KDEQDBusBrightnessService};

mod media;
use media::MediaControlService;

mod conf;
use conf::{ConfService, FileConfService};

mod logging;
mod touchpad_service;
use nix::poll::{PollFd, PollFlags, poll};
use nix::unistd::{close, pipe, read};
use std::os::fd::{AsFd, AsRawFd};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conf_service = FileConfService::new();
    conf_service.load_file()?;
    let audio_service = WpctlAudioService::new();
    let brightness_service = KDEQDBusBrightnessService::new()?;
    let conf = conf_service.get_conf()?;
    let media_service =
        MediaControlService::new(conf.media_control_mode, conf.seek_step_microseconds)?;

    let mut touchpad_service = touchpad_service::TouchpadService::new(
        &conf_service,
        &audio_service,
        &brightness_service,
        &media_service,
    )?;

    touchpad_service.init_debug();

    // Create self-pipe for signal handling
    let (read_pipe, write_pipe) = pipe().map_err(|e| format!("failed to create self-pipe: {e}"))?;

    // Set up signal handler for graceful shutdown
    // Write a byte to the pipe when signal is received
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        // Write a byte to wake up poll()
        let buf = [1u8];
        let _ = nix::unistd::write(&write_pipe, &buf);
    })?;

    println!("Daemon started. Press Ctrl+C to exit.");

    let result: Result<(), Box<dyn std::error::Error>> = 'main_loop: loop {
        // Create poll_fds fresh each iteration to avoid borrow issues
        let mut poll_fds = [
            PollFd::new(read_pipe.as_fd(), PollFlags::POLLIN),
            PollFd::new(touchpad_service.as_fd(), PollFlags::POLLIN),
        ];

        match poll(&mut poll_fds, None::<u16>) {
            Ok(0) => {
                // Should not happen with infinite timeout, but handle gracefully
                continue;
            }
            Ok(_) => {
                // Check which FD has events
                // Check pipe first (index 0)
                if let Some(revents) = poll_fds[0].revents() {
                    if revents.contains(PollFlags::POLLIN) {
                        // Read and discard the byte from pipe
                        let mut buf = [0u8; 1];
                        match read(read_pipe.as_raw_fd(), &mut buf) {
                            Ok(_) => {
                                // Check if we should exit
                                if !running.load(Ordering::SeqCst) {
                                    break 'main_loop Ok(());
                                }
                            }
                            Err(e) => {
                                eprintln!("failed to read from self-pipe: {e}");
                                // Continue running, but check running flag
                                if !running.load(Ordering::SeqCst) {
                                    break 'main_loop Ok(());
                                }
                            }
                        }
                    }
                }

                // Check touchpad (index 1)
                if let Some(revents) = poll_fds[1].revents() {
                    let device_gone = revents
                        .intersects(PollFlags::POLLERR | PollFlags::POLLHUP | PollFlags::POLLNVAL);

                    if device_gone {
                        eprintln!("touchpad device error/hangup; reopening");
                        if let Err(error) = touchpad_service.reopen_device() {
                            eprintln!("touchpad reopen failed: {error}");
                            thread::sleep(Duration::from_millis(250));
                        }
                    } else if revents.contains(PollFlags::POLLIN) {
                        if let Err(error) = touchpad_service.fetch_events() {
                            eprintln!("touchpad event loop error: {error}");
                            if let Err(reopen_error) = touchpad_service.reopen_device() {
                                eprintln!("touchpad reopen failed: {reopen_error}");
                                thread::sleep(Duration::from_millis(250));
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("poll error: {e}");
                thread::sleep(Duration::from_millis(250));
            }
        }
    };

    // Clean up the pipe
    if let Err(e) = close(read_pipe.as_raw_fd()) {
        eprintln!("failed to close pipe read end: {e}");
    }

    println!("Shutting down gracefully...");
    result
}
