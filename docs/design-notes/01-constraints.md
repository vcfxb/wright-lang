# Constraints in wright

One of the first things I considered when re-designing wright from scratch for approximately the 3rd or 4th time in 2022
was what I liked most in other languages and what I wanted to pull from and extend upon. One of the biggest things in my
mind when I considered this was the rust lifetimes system. The additional constraint was frustrating and difficult to 
learn at first, but signifigantly reduced the number of bugs that ended up in my code, especially memory bugs. i had
the idea for a more constrained, and more customizably constrained language based on this. What if you could specify
constraints to not only eliminate memory bugs at compile time, but also largely eliminate logic bugs. The more I thought
about this the more I realized that doing something like this entirely at compile time might be difficult. Doing even
part of this at compile time though would be useful, and the rest can be managed using a slightly gradual type system. 
While the full type of a variable in wright should be immutable at runtime, letting the developer apply additional
constraints to it as they do error checking or other things would be useful. These constrtaints exist at runtime on the 
type of the variable, not the value. If an integer variable is constrained to a certain range of values, the value can
be also copied / aliased to another integer variable with fewer constraints (or more, as long as they are met). 

These constraints could also be used for optimization reasons. If a variable is constrained to be of a certain length,
or between a cetrain set of values, that can reduce the memory used to represent it. Some constraints may even be 
applied to variables invisibly by static analysis for similar reasons. 

The most difficult parts of this problem in my mind, writing this now, are:
1. the implementation -- deciding what to check at compile time and what to check at runtime and how.
2. the developer experience -- how do I make constraints seem natural and easy to use but also powerful. How do I help 
    developers leverage this feature and extend constraints to eachother with proofs. 
3. (i forget the 3rd one right now, I'm very tired but it had something to do with documentation/learning curve)
