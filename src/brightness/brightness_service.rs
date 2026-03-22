pub trait BrightnessService {
    fn new() -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
    fn adjust_brightness(&self, delta: &f64) -> Result<(), Box<dyn std::error::Error>>;
}
