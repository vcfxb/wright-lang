# The Wright language
### *A language that flies*
#### Badges
| Service | Badge |
|:---:|:---:|
| Cargo Check Status | ![Cargo Check status](https://github.com/Alfriadox/wright-lang/actions/workflows/cargo-check.yml/badge.svg?branch=master) |
| Cargo Test Status | ![Cargo Test status](https://github.com/Alfriadox/wright-lang/actions/workflows/cargo-test.yml/badge.svg?branch=master) |
| Cargo Clippy Status | ![Cargo Clippy status](https://github.com/Alfriadox/wright-lang/actions/workflows/cargo-clippy.yml/badge.svg?branch=master) |
<!-- On kill_cache=1: https://github.com/lemurheavy/coveralls-public/issues/1065#issuecomment-435494495 -->
| Code Coverage | [![Coverage Status](https://coveralls.io/repos/github/Alfriadox/wright-lang/badge.svg?branch=master&kill_cache=1)](https://coveralls.io/github/Alfriadox/wright-lang?branch=master) |
| Docs.rs | [![Documentation](https://docs.rs/wright/badge.svg)](https://docs.rs/wright) |
| Crates.io | [![Crates.io](https://img.shields.io/crates/v/wright.svg)](https://crates.io/crates/wright) |
| GitHub release | [![GitHub release](https://img.shields.io/github/release/Alfriadox/wright-lang.svg)](https://github.com/Alfriadox/wright-lang/releases) |
| GitHub (pre-)release | [![GitHub (pre-)release](https://img.shields.io/github/release/Alfriadox/wright-lang/all.svg)](https://github.com/Alfriadox/wright-lang/releases) |
| Development Status | ![Status](https://img.shields.io/badge/status-actively--developed-green.svg) |

|  | Downloads|
|:---:|:---:|
| Total |![Github All Releases](https://img.shields.io/github/downloads/Alfriadox/wright-lang/total.svg) |
| Releases | ![Github Releases](https://img.shields.io/github/downloads/Alfriadox/wright-lang/latest/total.svg) |
| Pre-Releases| ![Github Pre-Releases](https://img.shields.io/github/downloads-pre/Alfriadox/wright-lang/latest/total.svg) |
| Crates.io | [![Crates.io](https://img.shields.io/crates/d/wright.svg)](https://crates.io/crates/wright) |
| Crates.io (Latest) | [![Crates.io](https://img.shields.io/crates/dv/wright.svg)](https://crates.io/crates/wright/0.8.0) |


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

### Installation:
There are several installation options.
1. Get the latest stable version from [the releases page](https://github.com/Alfriadox/wright-lang/releases).
2. If you have rust, via `cargo install wright`.
3. Building from source, by cloning this repository, and running `cargo build --release` in the wright directory, and 
then adding wright/target/release to your system path.
