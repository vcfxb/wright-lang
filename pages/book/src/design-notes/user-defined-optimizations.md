
# User defined optimizations 

One of the hardest things for me to reconcile as I build this language is how to make it high-level, while still
providing the ability to do relatively low-level things. I would make it completely low-level, however Rust already
exists as a well-liked, mature, production-ready, memory-safe language with many of the same features I hope to build
into Wright. Building Wright as another low-level language with a borrow checker and functional programming elements
would not only make it completely derivative of Rust, but also introduce many of the same drawbacks that Rust has in
terms of expressing Futures & other complex memory-related types and in terms of learning-curve (especially around 
the borrow checker). 

In order to do both, the vast majority of programming in wright will be covered under a garbage 
collector. Programmers will write classes, enums, and unions, without ever thinking too hard about memory allocation or
management. 

... TBD
