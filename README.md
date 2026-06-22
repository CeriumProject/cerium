# Cerium Programming Language
## Warning
This language has not even reached version 0.1 and as such still has many TODOs and missing/incomplete features.
Still, it is turing-complete, and you can do some fun things with it (see examples)!

## Introduction
Cerium is a low-level procedural programming language designed to be able to compile to bytecode for the Amine Virtual Machine.
Amine is a CPU standard I defined after participating in my uni's computer architecture and operating systems course.
Since I made it up myself, there is no other compiler (backend) able to Amine.
As such I'm currently designing this language and implementing its compiler, so I can finally stop writing assembly code.

## Design Philosophy
Cerium is similar in its low-level-ness to C, but comes with Rust-style syntax (since I prefer that) and some features I prefer/need.
It is a procedural language with *no* runtime guardrails, but enforced strong static-typing and (hopefully) helpful compiler errors.
I want working with function pointers and arbitrary memory access/manipulation to feel like a breeze.
Seamless integration into existing Amine projects is another medium-term goal I want to fulfil.
I also want all memory allocations, usages of pointers, and weird (possibly not-intended) operations to be explicit.

## Data Types and Memory Management
Currently, Cerium has the following data types:
- u16 (unsigned 16-bit integer)
- i16 (signed 16-bit integer)
- f16 (16-bit floating point)
- bool (boolean)
- &<type> (reference/pointer)
- Struct (structs, currently only accessible through pointers)
- fn(parameters) -> result (functions, only accessible through pointers)

## Language Features
Cerium programs are made up of three different kinds of top-level definitions

Functions can accept multiple and currently return one explicitly type value.
Side-behaviour is allowed, though I consider adding a pure keyword to mark functions, which cannot exhibit such behaviour.
Example:
```rust
fn add(x: f16, y: f16) -> f16 {
    x + y
}
```
Note: Your markdown renderer may tell you that code is Rust.
That's because if I told it, it was Cerium, it would most likely have no highlighting.

You can define structs like this.
Currently, they can only be accessed through pointers because the backend does not yet work with values of sizes above 16 bits.
```rust
struct FloatList {
    floats: &f16,
    len: u16,
    cap: u16,
}
```

You can use the const-keyword to declare constant (and static!) values like this.
I explicitly decided against using a static-keyword for static values, because those are just constant pointers to sections of memory, if you really think about it!
Note: I use const(ant) in the sense of being stored in the binary instead of stack/heap.
```rust
const WIDTH: u16 = 128;
const STATIC_I16: &i16 = &+0;
const CORNERS: &&Vertex = &[
    &Vertex { x: 0.0, y: 0.0 },
    &Vertex { x: 0.0, y: 1.0 },
    &Vertex { x: 1.0, y: 0.0 },
];
```
Note: all i16s have to be prefixed with plus or minus.
Otherwise, Cerium will read the number as an u16.

Explanation of language features that can be used within functions will be added later.
The same goes for installation and compilation instructions.