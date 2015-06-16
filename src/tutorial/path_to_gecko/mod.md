% Path to Gecko
% Felix Klock
% Thursday 25 June

# Rust: How do we get to Gecko?

## Goals

* Convince you Rust is awesome
* Provide seeds of knowledge
  * One can only cultivate so much during a three hour window

## Outline

* Playpen: Ownership and Borrowing; Arrays and Strings
* Local Development: Cargo; Crates and Modules
* More Fundamentals: Data; More Borrowing; Traits
* Systems Development: Concurrency and I/O; FFI

# Part 1: Playpen

## Playpen

* For now, open `http://play.rust-lang.org/` in a web browser.

* For later: Install Rust from the USB stick when it comes to you

## Rust Basics

```rust
pub fn main() {
    println!("Hello World!");
}
```

## Ownership and Move Semantics

```rust
pub fn fill_a_vec() {
	let mut vec = Vec::new();
	vec.push(22);
	vec.push(44);
	vec.push(66);
}
```

## Borrowing (immutably), Scopes (no explicit lifetimes)

## Exclusive Access versus Ownership; &mut borrows

## Vectors and Arrays and Slices

## for and iterators; String and &str

# Part 2: Everyone's installed; what is local development like?
## Getting started with cargo: cargo {new, build, test, run}
## crates versus modules; `mod f;` sugar and the file system. privacy.
## cargo's dependency handling; crates.io

# Part 3: Back to Language Fundamentals
## structs and enums; Option and Result
## Scopes and Lifetimes. Lifetime bindings. Borrowed return values.
## Generic items; Trait-bounded polymorphism
## Traits as Objects

# Part 4: Systems Development
## Concurrency: `Send`, `Sync`, `thread::scoped`
## I/O in Rust, `try!`
## Concurrency: `thread::spawn`, channels
## unsafe code, FFI
## Rust in Gecko (demonstration)
