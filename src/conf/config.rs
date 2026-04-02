use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conf {
    pub left_edge_threshold_percent: f64,
    pub right_edge_threshold_percent: f64,
    pub top_edge_threshold_percent: f64,
    pub sensitivity: f64,
    pub invert_y: bool,
    pub volume_step: f64,
    pub brightness_step: f64,
    pub seek_step_microseconds: i64,
}
