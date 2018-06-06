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
[![Minimum Rust Version](https://img.shields.io/badge/minimum_rust_version-1.24.0-orange.svg)](https://www.rust-lang.org/)


|  | Downloads|
|:---:|:---:|
| Total |![Github All Releases](https://img.shields.io/github/downloads/Wright-Language-Developers/Wright-lang/total.svg) |
| Releases | ![Github Releases](https://img.shields.io/github/downloads/Wright-Language-Developers/Wright-lang/latest/total.svg) |
| Pre-Releases| ![Github Pre-Releases](https://img.shields.io/github/downloads-pre/Wright-Language-Developers/Wright-lang/latest/total.svg) |
| Crates.io | ![Crates.io](https://img.shields.io/crates/d/wright.svg) |
| Crates.io (Latest) | ![Crates.io](https://img.shields.io/crates/dv/wright.svg) |


Wright is a lightweight programming language that is intended for use in similar situations to other JVM languages like
Java and Kotlin. What makes wright different is its unique approach to the object oriented model, following an enforcing
concepts from more functional languages. From rust, for example, wright uses a trait and polymorphism system, along with
tagged unions. Wright also takes from Java'sinheritance model, but slightly tweaks it, prevent inheritance based issues 
and confusion.

##### Wright is inspired by
* Rust
    * Traits
    * Tagged Unions
    * Type aliases
    * Structs
    * Functional Programming, Lambda expressions.
    * Mutability vs Immutability
    * Type inference
    * Memory safety
    * Concurrency
* Java
    * Inheritance*
    * Bytecode format
    * Mutability vs Immutability
    * Type inference (In Java 10)
* Kotlin
    * Inheritance*
    * Bytecode format
    * Mutability vs Immutability
    * Type Inference
* Python
    * Type Inference
    * Ease of use
* JavaScript
    * Object Oriented Model

##### The core goals of the language:
* Ease of use
* Robustness
* Reasonable speed
* Memory Safety
* Concurrency
* Portability
* Interoperability with other JVM languages including Java and Kotlin. 

##### It functions on a system of
* Strong typing
* Static typing
* Parametric polymorphism
* Object-Oriented programming
* Functional programming
* Multi-paradigm programming

### Installation:
There are several installation options.
1. Get the latest stable version from [the releases page](https://github.com/Wright-Language-Developers/Wright-lang/releases).
2. If you have rust, via `cargo install wright`.
3. Building from source, by cloning this repository, and running `cargo build --release` in the wright directory, and 
then adding wright/target/release to your system path.
