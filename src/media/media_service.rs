pub trait MediaService {
    fn seek(&self, offset_microseconds: i64) -> Result<(), Box<dyn std::error::Error>>;
}
