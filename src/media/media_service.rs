pub trait MediaService {
    fn new() -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
    fn seek(&self, offset_microseconds: i64) -> Result<(), Box<dyn std::error::Error>>;
}
