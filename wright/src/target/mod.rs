
pub mod jvm;
pub mod treewalk;


/// An enum representing targetable byte-code formats, including the Java Virtual Machine
/// and WebAssembly.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(bad_style)]
pub enum Target {
    /// Java Virtual Machine Class file format.
    /// See [the JVM 9 Specification](https://docs.oracle.com/javase/specs/jvms/se9/html/index.html)
    JVM,
}
