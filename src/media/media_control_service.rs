use crate::conf::MediaControlMode;
use crate::media::{ArrowKeyMediaService, MediaService, MprisMediaService};

pub enum MediaControlService {
    MprisSeek(MprisMediaService),
    ArrowKeys(ArrowKeyMediaService),
}

impl MediaControlService {
    pub fn new(mode: MediaControlMode) -> Result<Self, Box<dyn std::error::Error>> {
        match mode {
            MediaControlMode::MprisSeek => Ok(Self::MprisSeek(MprisMediaService::new()?)),
            MediaControlMode::ArrowKeys => Ok(Self::ArrowKeys(ArrowKeyMediaService::new()?)),
        }
    }
}

impl MediaService for MediaControlService {
    fn seek(&self, offset_microseconds: i64) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::MprisSeek(service) => service.seek(offset_microseconds),
            Self::ArrowKeys(service) => service.seek(offset_microseconds),
        }
    }
}
