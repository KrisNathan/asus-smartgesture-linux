use crate::conf::Conf;

pub trait ConfService {
    fn new() -> Self
    where
        Self: Sized;
    fn get_conf(&self) -> Result<&Conf, std::io::Error>;
    fn save_conf(&self, conf: &Conf) -> Result<(), Box<dyn std::error::Error>>;
}
