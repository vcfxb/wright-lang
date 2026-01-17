# Exposing Optimizations to Users

This one is several months later than all the other ones, after the project once again stalled out with me getting 
busy with work and life. I've been dwelling on where I want this project to go lately though, and figured I'd jot down
some thoughts, both so that I don't forget but also so that others can see the evolution of this looking back on it
perhaps (though I kinda doubt that anyone will ever read most of this stuff lol). For a while now, part of what's killed
my motivation to work on this has been that it's felt too close to Rust. Sure they can share similarities, and it's
not uncommon for languages to be inspired by one another, but if this ends up being effectively the same language as
Rust (from a user perspective) then that means I haven't done anything interesting or new. 

With that in mind, I've been considering what I like and dislike about Rust (and other languages) and have come up with
the following. Many languages hide their optimizations behind interfaces that are easier to abstractly understand. If
you're lucky, some may mention them somewhere in some documentation, but it's very rare that you're ever able to
directly interact with the optimizer. The surface of the language is abstract concepts like structs, classes,
interfaces, functions, etc. and then all of the stuff that takes you down to the hardware is like magic glue. But what
if it didn't have to be that way. What if you wrote (and/or could read) the glue yourself when you wanted to, without
getting too messy.

An example of what I'm talking about comes with rust's optimizations around `Option`s containing references. Options
are special-cased such that for all other types, they hold a byte of data storing the discriminant, and then store 
the actual contents of the option adjacently. The exceptions to this are when using `Option`s containing references
or `NonZero<Integer>` types. In those cases, it stores the `None` variant just as a 0 or a null pointer (also a 0) and
the type size remains unchanged. This is not something that you could do for a type you wrote -- If you had a type you
wrote that could compress types containing it into a smaller size, it would be cool if there was a way to express that
to the compiler without mucking with bytes manually, or using gross and unreliable `mem::transmute`s (in rust at least).

We also must remember the relationship languages have with each other -- it is very rare that a large project will
truly all be in one language. Many rust projects use wrapper crates and generated bindings around C libraries, and the
same is true for other languages. Generating and maintaining those bindings can be tricky, and is non-trival --
the difference in ABI sometimes causes extra instructions to be generated in some places, which can add up if a function
is being called tens of millions of times a second. 

What I come to at the end of this is an idea for a language that is explicitly aware of everything. This means the
hardware it's running on (to the extent possible), the ABI of external libraries it's using, and the specific memory
layout and calling conventions for each of the structures and functions defined by the user.

If this is sounding lower-level than the language described by previous design notes, that's because it is! Rust has
already for years been moving towards being more of a C++ replacement than a C replacement, which it does well. This
language would live down closer to C, llvm IR, and maybe zig (which I've still never used at time of writing). The
difference with those languages I'm hoping to achieve here is both a better user experience (C especially has years of 
history baked into its spec), and the ability to have the language get out of your way more when you don't need the
lower level control.
