//! Trait and Enum that make up the Wright interpreter error system.
//! This is used all throughout the Wright project for error reporting and execution.

/// Trait for Wright's compiler errors.
/// Each error should have its own implementation of this.
pub trait WrightError {
    /// The constructor method.
    fn new(info: String, level: WrightErrorLevels) -> Self;
    /// Returns information about the error as a String.
    /// Usually a formatted output statement referencing the line,
    /// and sometimes location, where the error was raised.
    fn get_info(&self) -> String;
    /// Return the type of error, usually a short error name like
    /// "IOError" or "ParserError"
    fn get_type(&self) -> String;
    /// Returns the error level as an instance of `WrightErrorLevels`
    fn get_level(&self) -> WrightErrorLevels;
    /// Executes the error, terminating the program and returning a failed exit code (1).
    fn panic(&self) -> i32 {
        println!("
{:?}:{}:
    {}
        ", self.get_level(), self.get_type(), self.get_info());
        return 1;
    }
}

#[derive(Debug, Copy, Clone)]
/// Levels of Wright's errors.
/// Warning is for non-fatal interpreter warnings.
/// Fatal is for process terminating interpreter errors.
/// Panic is for fatal panics raised by the user program.
pub enum WrightErrorLevels {
    Warning,
    Fatal,
    Panic,
}