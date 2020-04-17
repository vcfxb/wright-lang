# The Wright language
### *A language that flies*
#### Badges
| Service | Badge |
|:---:|:---:|
| Travic CI | [![Build Status](https://travis-ci.org/Wright-Language-Developers/Wright-lang.svg?branch=master)](https://travis-ci.org/Wright-Language-Developers/Wright-lang) |
| GitHub Action CI | ![Rust](https://github.com/Wright-Language-Developers/Wright-lang/workflows/Rust/badge.svg?branch=master) |
| Appveyor CI | [![Build status](https://ci.appveyor.com/api/projects/status/kh76mn7ly95kish3?svg=true)](https://ci.appveyor.com/project/WrightLanguage/wright-lang) |
| Code Coverage | [![Coverage Status](https://coveralls.io/repos/github/Wright-Language-Developers/Wright-lang/badge.svg?branch=master)](https://coveralls.io/github/Wright-Language-Developers/Wright-lang?branch=master) |
| Docs.rs | [![Documentation](https://docs.rs/wright/badge.svg)](https://docs.rs/wright) |
| Crates.io | [![Crates.io](https://img.shields.io/crates/v/wright.svg)](https://crates.io/crates/wright) |
| GitHub release | [![GitHub release](https://img.shields.io/github/release/Wright-Language-Developers/Wright-lang.svg)](https://github.com/Wright-Language-Developers/Wright-lang/releases) |
| GitHub (pre-)release | [![GitHub (pre-)release](https://img.shields.io/github/release/Wright-Language-Developers/Wright-lang/all.svg)](https://github.com/Wright-Language-Developers/Wright-lang/releases) |
| Average Issue resolution | [![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/Wright-Language-Developers/Wright-lang.svg)](https://isitmaintained.com/project/Wright-Language-Developers/Wright-lang "Average time to resolve an issue") |
| Issues still open | [![Percentage of issues still open](http://isitmaintained.com/badge/open/Wright-Language-Developers/Wright-lang.svg)](https://isitmaintained.com/project/Wright-Language-Developers/Wright-lang "Percentage of issues still open") |
| Development Status | ![Status](https://img.shields.io/badge/status-actively--developed-green.svg) |

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
