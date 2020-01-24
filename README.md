# The Wright language
### *A language that flies*
[![Build Status](https://travis-ci.org/Wright-Language-Developers/Wright-lang.svg?branch=master)](https://travis-ci.org/Wright-Language-Developers/Wright-lang)
[![Build status](https://ci.appveyor.com/api/projects/status/kh76mn7ly95kish3?svg=true)](https://ci.appveyor.com/project/WrightLanguage/wright-lang)
[![Documentation](https://docs.rs/wright/badge.svg)](https://docs.rs/wright)
[![Crates.io](https://img.shields.io/crates/v/wright.svg)](https://crates.io/crates/wright)
[![GitHub release](https://img.shields.io/github/release/Wright-Language-Developers/Wright-lang.svg)](https://github.com/Wright-Language-Developers/Wright-lang/releases)
[![GitHub (pre-)release](https://img.shields.io/github/release/Wright-Language-Developers/Wright-lang/all.svg)](https://github.com/Wright-Language-Developers/Wright-lang/releases)
[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/Wright-Language-Developers/Wright-lang.svg)](http://isitmaintained.com/project/Wright-Language-Developers/Wright-lang "Average time to resolve an issue")
[![Percentage of issues still open](http://isitmaintained.com/badge/open/Wright-Language-Developers/Wright-lang.svg)](http://isitmaintained.com/project/Wright-Langauge-Developers/Wright-lang "Percentage of issues still open")
![Status](https://img.shields.io/badge/status-actively--developed-green.svg)
[![Minimum Rust Version](https://img.shields.io/badge/minimum_rust_version-1.35.0-orange.svg)](https://www.rust-lang.org/)


|  | Downloads|
|:---:|:---:|
| Total |![Github All Releases](https://img.shields.io/github/downloads/Wright-Language-Developers/Wright-lang/total.svg) |
| Releases | ![Github Releases](https://img.shields.io/github/downloads/Wright-Language-Developers/Wright-lang/latest/total.svg) |
| Pre-Releases| ![Github Pre-Releases](https://img.shields.io/github/downloads-pre/Wright-Language-Developers/Wright-lang/latest/total.svg) |
| Crates.io | ![Crates.io](https://img.shields.io/crates/d/wright.svg) |
| Crates.io (Latest) | ![Crates.io](https://img.shields.io/crates/dv/wright.svg) |


Wright is a lightweight programming language heavily inspired by Rust. Wright aims to take advantage of some of 
Rust's key features such as safety and speed, and augment them a number of other features and additions to the
backend and type system.

##### The core goals of the language:
* Ease of use
* Robustness
* Reasonable speed
* Memory Safety
* Concurrency
* Portability

##### Comparison of Wright and Rust:
|               |Rust |Wright|
|:---           | --- | ---|
|Reference Types| x ||
|Lifetimes |x||
|Garbage Collector ||x|
|Classes | |x|
|Type-level Constants |not yet|x|
|Enums |tagged unions | dedicated sub-type|
|Structs |x|x|
|Traits |x|x
|Unions |enum or untagged | dedicated tagged type
|Floats | primitives | standard library |
|Targets| many through LLVM | none yet, hopefully JVM, LLVM, and a dedicated RISC VM 


### Installation:
There are several installation options.
1. Get the latest stable version from [the releases page](https://github.com/Wright-Language-Developers/Wright-lang/releases).
2. If you have rust, via `cargo install wright`.
3. Building from source, by cloning this repository, and running `cargo build --release` in the wright directory, and 
then adding wright/target/release to your system path.
