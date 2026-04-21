use super::ConfService;

static LEFT_EDGE_THRESHOLD_PERCENT: f64 = 0.1;
static RIGHT_EDGE_THRESHOLD_PERCENT: f64 = 0.9;
static TOP_EDGE_THRESHOLD_PERCENT: f64 = 0.1;
static SENSITIVITY: f64 = 0.5;
static INVERT_Y: bool = false;
static VOLUME_STEP: f64 = 0.05;
static BRIGHTNESS_STEP: f64 = 0.05;
static SEEK_STEP_MICROSECONDS: i64 = 10_000_000;

pub struct StaticConfService {
    config: super::Conf,
}

impl ConfService for StaticConfService {
    fn new() -> Self {
        StaticConfService {
            config: super::Conf {
                left_edge_threshold_percent: LEFT_EDGE_THRESHOLD_PERCENT,
                right_edge_threshold_percent: RIGHT_EDGE_THRESHOLD_PERCENT,
                top_edge_threshold_percent: TOP_EDGE_THRESHOLD_PERCENT,
                sensitivity: SENSITIVITY,
                invert_y: INVERT_Y,
                volume_step: VOLUME_STEP,
                brightness_step: BRIGHTNESS_STEP,
                seek_step_microseconds: SEEK_STEP_MICROSECONDS,
            },
        }
    }

    fn get_conf(&self) -> Result<&super::Conf, std::io::Error> {
        Ok(&self.config)
    }

    fn save_conf(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
