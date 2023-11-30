
# The Backend(s) of the Wright Compiler and Interpreter.

I have had a many thoughts, opinions, and different stances on what I wanted to build for Wright's backend. I have 
changed my mind more times than I can count, and I'm certain I will continue to change my mind on this several more 
times. 

So far it appears there are a few main target options:

|         | LLVM         | Cranelift                                                | JVM / Java Bytecode | Bespoke bytecode compiler & interpreter             | Bespoke bytecode compiler & transpiler |
|---      | ---          | ---                                                      | ---                 | ---                                                 | ---                                    |
| Output  | Machine code | Machine code                                             | .class file         | Custom bytecode                                     | Custom bytecode & transpiler targets   |
| Targets | vey many     | `x86_64`,  `aarch64` (ARM64), `s390x` (IBM Z), `riscv64` | JVM                 | Anything that the rust based interpreter can run on | very many (assuming transpile to LLVM) |

Right now I'm largely tempted to target both a bespoke bytecode interpreter (perhaps in addition to a transpiler) and 
LLVM. I like the idea of compiling to Cranelift as well, but the additional work for it may be more than it's worth. 
Compiling to the JVM would be cool for interoperability with Java/Scala/Kotlin/etc programs, but my language is so 
different from them that there would be a significant amount lost in translation from Wright to the JVM. I will start
with the bespoke interpreter/transpiler. 



