use crate::conf::Conf;

pub trait ConfService {
    fn new() -> Self
    where
        Self: Sized;
    fn get_conf(&self) -> Result<&Conf, std::io::Error>;

    // future: update config
    #[allow(dead_code)]
    fn save_conf(&self) -> Result<(), Box<dyn std::error::Error>>;
}
