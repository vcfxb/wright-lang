# The Wright language
### *A language that flies*
[![Build Status](https://travis-ci.org/Wright-Language-Developers/Wright-lang.svg?branch=master)](https://travis-ci.org/Wright-Language-Developers/Wright-lang)
[![Build status](https://ci.appveyor.com/api/projects/status/kh76mn7ly95kish3?svg=true)](https://ci.appveyor.com/project/WrightLanguage/wright-lang)
[![Documentation](https://docs.rs/wright/badge.svg)](https://docs.rs/wright)
[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/Wright-Language-Developers/Wright-lang.svg)](http://isitmaintained.com/project/Wright-Language-Developers/Wright-lang "Average time to resolve an issue")
[![Percentage of issues still open](http://isitmaintained.com/badge/open/Wright-Language-Developers/Wright-lang.svg)](http://isitmaintained.com/project/Wright-Langauge-Developers/Wright-lang "Percentage of issues still open")

Wright is a lightweight programming language that is intended for use in similar situations to other JVM languages like
Java and Kotlin. What makes wright different is its unique approach to the object oriented model, following an enforcing
concepts from more functional languages. From rust, for example, wright uses a trait and polymorphism system, along with
tagged unions and lifetimes and borrows to prevent null pointers and related exceptions. Wright also takes from Java's
inheritance model, but slightly tweaks it, prevent inheritance based issues and confusion.

##### Wright is inspired by
* Rust
    * Traits
    * Lifetimes / Borrowing
    * Tagged Unions
    * Type aliases
    * Structs, refrences
    * Functional Programming, Lambda expressions.
    * Mutability vs Immutability
    * Type inference
    * Memory safety
    * Concurrency
* Java
    * Inheritance*
    * Bytecode format
    * Type inference (In Java 10)
* Kotlin
    * Inheritance*
    * Bytecode format
    * Mutability vs Immutability
    * Type Inference
* Python
    * Type Inference
    * Ease of use

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

### Dependencies:
* Rust (1.21.0 or greater)