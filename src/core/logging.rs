pub trait Logging {
    fn enable_logging(&mut self);
    fn disable_logging(&mut self);
    fn logging_enabled(&self) -> bool;
}
