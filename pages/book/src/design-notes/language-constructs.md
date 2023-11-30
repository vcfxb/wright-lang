
# Language constructs

There are a variety of constructed abstractions in a language that not only give it logical/expressive power, but help 
guide and define the user-friendlyness of the language. A good example of this is Rust's lifetime and borrowing rules. 
While these rules make it possible to write and express programs that would be difficult to keep track of in a language 
like C, it also steepens the language's learning curve. 

In designing Wright, I want to make a wide variety of language constructs available to the user to help make the language
more elegant, without making it too much more difficult to use. In designing these language constructs, a few principles 
should be kept in mind. 

1. Wright aims to be a relatively simple, easy to use language. 
2. Wright aims to protect the user, to the greatest extent possible, from writing buggy code. 
3. Wright aims to show an appropriate amount of the language's internals. Users should be able to reason about how their
    code runs and what allocates, or doesn't.
4. Wright is a multipurpose programming language. No one paradigm or style should be expressly promoted over others. 

With those principles in mind, we can start to establish a set of features to guide the language's design. 

1. Wright is strongly typed, and will infer types wherever possible. 
2. ~~Wright is garbage collected.~~ -- I changed my mind on this -- Wright will have lifetimes and borrowing similar to 
    Rust. 
3. Wright has traits.
4. Wright has enumerations.
5. Wright has tagged unions.
6. Wright has ~~classes~~ record types.
7. Wright has type aliases. 
8. Wright has constraints (to be discussed further).
9. Wright has inheritance for traits, enumerations, tagged unions, and constraints. 
10. Functions are first class language constructs in Wright. 
11. Wright does not have macros -- most macro-style meta programming should be achievable with generics. 
12. Wright has abstract types -- representation & implementation can be dependent on the generic used. 

## On Constraints:

Wright will be one of the few multipurpose languages that I know of to use constraints. Constraints can be a very
powerful tool for logical induction. They allow a programmer to define and check parts of their program at compile time. 
Wright constraints will be invokable both at compile time and at runtime. There may be some exceptions if we ever decide
to allow definition of compile-time only (`const constraint`) constraints. Constraints will be strongly bound to a type,
but that type may be generic (so constraints on lists and arrays will be possible). Constraints will act very similarly
to functions, carrying zero sense of state or instantiation like a class might. 

## Note

This document is a work in progress, and may be changed or updated further at a later date. 
