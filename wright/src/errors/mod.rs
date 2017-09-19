pub trait WrightError {
    fn new(info: String, level: WrightErrorLevels) -> Self;
    fn get_info(&self) -> String;
    fn get_type(&self) -> String;
    fn get_level(&self) -> WrightErrorLevels;
    fn panic(&self) -> i32 {
        println!("
{:?}:{}:
    {}
        ", self.get_level(), self.get_type(), self.get_info());
        return 1;
    }
}

#[derive(Debug, Copy, Clone)]
pub enum WrightErrorLevels {
    Warning,
    Fatal,
}