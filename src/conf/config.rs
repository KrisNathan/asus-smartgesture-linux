use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaControlMode {
    #[default]
    MprisSeek,
    ArrowKeys,
}

fn default_media_step() -> f64 {
    0.05
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conf {
    pub left_edge_threshold_percent: f64,
    pub right_edge_threshold_percent: f64,
    pub top_edge_threshold_percent: f64,
    pub sensitivity: f64,
    pub invert_y: bool,
    pub volume_step: f64,
    pub brightness_step: f64,
    /// Pad-fraction of horizontal travel that triggers one media seek step.
    #[serde(default = "default_media_step")]
    pub media_step: f64,
    pub seek_step_microseconds: i64,
    #[serde(default)]
    pub media_control_mode: MediaControlMode,
}
