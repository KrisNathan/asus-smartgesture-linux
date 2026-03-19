use crate::conf::Conf;

pub trait ConfService {
    fn new() -> Self
    where
        Self: Sized;
    fn get_conf(&self) -> Result<Conf, std::io::Error>;
    fn save_conf(&self, conf: &Conf) -> Result<(), std::io::Error>;

    fn get_left_edge_threshold_percent(&self) -> f64;
    fn get_right_edge_threshold_percent(&self) -> f64;
    fn get_sensitivity(&self) -> f64;
    fn get_invert_y(&self) -> bool;
    fn get_volume_step(&self) -> f64;
    fn get_brightness_step(&self) -> f64;
}
