
pub mod jvm;
pub mod treewalk;
pub mod wasm;

/// An enum representing targetable byte-code formats, including the Java Virtual Machine
/// and WebAssembly.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(bad_style)]
pub enum Target {
    /// Java Virtual Machine Class file format.
    JVM,
    /// Web Assembly format.
    WASM,
    /// The BrainFuck esoteric language.
    BrainFuck,
}