### Wright has not yet reached version 1.0.0 yet, and is currently in an incomplete/experimental state.

# <img src="https://github.com/vcfxb/wright-lang/blob/main/pages/static/assets/white_logo.png?raw=true" height=50 /> The Wright Programming Language
## *A language that flies*

*Wright is an all-purpose programming language inspired by Rust, Ada, and Typescript. 
Pulling from all three of these excellent languages, Wright intends to offer a combination of speed, ergonomics, and precision.*

### Badges
*Wright is automatically checked and tested using the latest available github runners for Ubuntu, MacOS, and Windows*
| Service | Badge |
|:---:|:---:|
| Cargo Check Status | ![Cargo Check status](https://github.com/vcfxb/wright-lang/actions/workflows/cargo-check.yml/badge.svg?branch=main) |
| Cargo Test Status | ![Cargo Test status](https://github.com/vcfxb/wright-lang/actions/workflows/cargo-test.yml/badge.svg?branch=main) |
| Cargo Clippy Status | ![Cargo Clippy status](https://github.com/vcfxb/wright-lang/actions/workflows/cargo-clippy.yml/badge.svg?branch=main) |
| Code Coverage (Coveralls) | [![Coverage Status](https://coveralls.io/repos/github/vcfxb/wright-lang/badge.svg?branch=main)](https://coveralls.io/github/vcfxb/wright-lang?branch=main) |
| Code Coverage (Codecov.io) | [![codecov](https://codecov.io/github/vcfxb/wright-lang/branch/main/graph/badge.svg?token=HO07JEYMIH)](https://codecov.io/github/vcfxb/wright-lang/commits?branch=main) |
| Docs.rs | [![Documentation](https://docs.rs/wright/badge.svg)](https://docs.rs/wright) |
| Crates.io | [![Crates.io](https://img.shields.io/crates/v/wright.svg)](https://crates.io/crates/wright) |
| GitHub release | [![GitHub release](https://img.shields.io/github/release/vcfxb/wright-lang.svg)](https://github.com/vcfxb/wright-lang/releases) |
| GitHub (pre-)release | [![GitHub (pre-)release](https://img.shields.io/github/release/vcfxb/wright-lang/all.svg)](https://github.com/vcfxb/wright-lang/releases) |
| Development Status | ![Status](https://img.shields.io/badge/status-actively--developed-green.svg) |

<!-- On `kill_cache=1` above: https://github.com/lemurheavy/coveralls-public/issues/1065#issuecomment-435494495 -->

|  | Downloads|
|:---:|:---:|
| Total |![Github All Releases](https://img.shields.io/github/downloads/vcfxb/wright-lang/total.svg) |
| Releases | ![Github Releases](https://img.shields.io/github/downloads/vcfxb/wright-lang/latest/total.svg) |
| Pre-Releases| ![Github Pre-Releases](https://img.shields.io/github/downloads-pre/vcfxb/wright-lang/latest/total.svg) |
| Crates.io | [![Crates.io](https://img.shields.io/crates/d/wright.svg)](https://crates.io/crates/wright) |
| Crates.io (Latest) | [![Crates.io](https://img.shields.io/crates/dv/wright.svg)](https://crates.io/crates/wright/0.9.2) |

### Syntax Samples
```
// Hello World! 
use wright::io::println;

func main() {
    println("Hello World!");
}
```

```
// FizzBuzz 1 through 100
use wright::io::println;

type FizzBuzzInteger = integer constrain |i| { i <= 100 && i >= 0 };

func fizzbuzz(i: FizzBuzzInteger) {
    if i % 15 == 0 { println("FizzBuzz"); }
    else if i % 5 == 0 { println("Buzz"); }
    else if i % 3 == 0 { println("Fizz"); }
    else { println(i); }
}

func main() {
    // Compiler error here if we use a range iterator that contains a value violating the constraints of 
    // `FizzBuzzInteger`. 
    (1..=100).for_each(fizzbuzz);
}
```

### The core goals of the language:
* __Developer experience__ -- Every error message, syntax choice, and standard library function should be friendly and well
    documented.
* __Robustness__ -- Wright's type system should be expressive enough to appropriately capture the domain, representation, 
    and functionality of every symbol the programmer interacts with. 
* __Speed__ -- Wright leverages the newest major version of LLVM (at the time of writing, LLVM 18), to compile code 
    directly to assembly, avoiding the overhead of an interpreter, garbage collector, and other associated tools 
    by default. 
* __Memory Safety__ -- Wright pulls significant inspiration from Rust's lifetime system, with some modifications. 

### Installation:
There are several installation options.
- Get the latest stable version from [the releases page](https://github.com/vcfxb/wright-lang/releases).
- If you have rust, via `cargo install wright`.
- Building from source, by cloning this repository, and running `cargo build --release` in the wright directory, and 
    then adding `wright/target/release` to your system path. You will need LLVM 18 installed and appropriately 
    configured to compile Wright. See the [llvm-sys crate docs](https://crates.io/crates/llvm-sys) for tips on how to do 
    this.
