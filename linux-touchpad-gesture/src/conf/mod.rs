pub struct Conf {
    pub left_edge_threshold_percent: f64,
    pub right_edge_threshold_percent: f64,
    pub sensitivity: f64,
    pub invert_y: bool,
}

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
}


mod static_conf_service;
pub use static_conf_service::StaticConfService;