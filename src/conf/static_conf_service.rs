use super::ConfService;

pub struct StaticConfService;

static LEFT_EDGE_THRESHOLD_PERCENT: f64 = 0.1;
static RIGHT_EDGE_THRESHOLD_PERCENT: f64 = 0.9;
static TOP_EDGE_THRESHOLD_PERCENT: f64 = 0.1;
static SENSITIVITY: f64 = 0.5;
static INVERT_Y: bool = false;
static VOLUME_STEP: f64 = 0.05;
static BRIGHTNESS_STEP: f64 = 0.05;
static SEEK_STEP_MICROSECONDS: i64 = 10_000_000;

impl ConfService for StaticConfService {
    fn new() -> Self {
        StaticConfService
    }

    fn get_conf(&self) -> Result<super::Conf, std::io::Error> {
        Ok(super::Conf {
            left_edge_threshold_percent: LEFT_EDGE_THRESHOLD_PERCENT,
            right_edge_threshold_percent: RIGHT_EDGE_THRESHOLD_PERCENT,
            top_edge_threshold_percent: TOP_EDGE_THRESHOLD_PERCENT,
            sensitivity: SENSITIVITY,
            invert_y: INVERT_Y,
            volume_step: VOLUME_STEP,
            brightness_step: BRIGHTNESS_STEP,
            seek_step_microseconds: SEEK_STEP_MICROSECONDS,
        })
    }

    fn save_conf(&self, _conf: &super::Conf) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn get_left_edge_threshold_percent(&self) -> f64 {
        LEFT_EDGE_THRESHOLD_PERCENT
    }
    fn get_right_edge_threshold_percent(&self) -> f64 {
        RIGHT_EDGE_THRESHOLD_PERCENT
    }
    fn get_top_edge_threshold_percent(&self) -> f64 {
        TOP_EDGE_THRESHOLD_PERCENT
    }
    fn get_sensitivity(&self) -> f64 {
        SENSITIVITY
    }
    fn get_invert_y(&self) -> bool {
        INVERT_Y
    }
    fn get_volume_step(&self) -> f64 {
        VOLUME_STEP
    }
    fn get_brightness_step(&self) -> f64 {
        BRIGHTNESS_STEP
    }
    fn get_seek_step_microseconds(&self) -> i64 {
        SEEK_STEP_MICROSECONDS
    }
}
