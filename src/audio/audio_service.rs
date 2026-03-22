pub trait AudioService {
    fn new() -> Self
    where
        Self: Sized;
    fn adjust_volume(&self, delta: &f64) -> Result<(), std::io::Error>;
}
