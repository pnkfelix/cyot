Hello world

### A path to Gecko

* Part 1: Playpen (in parallel with distribution via USB stick)
  * Ownership and Move Semantics
  * Borrowing (immutably), Scopes (no explicit lifetimes)
  * Exclusive Access versus Ownership; &mut borrows
  * Vectors and Arrays and Slices
  * for and iterators; String and &str
* Part 2: Everyone's installed; what is local development like?
  * Getting started with cargo: cargo {new, build, test, run}
  * crates versus modules; `mod f;` sugar and the file system. privacy.
  * cargo's dependency handling; crates.io
* Part 3: Back to Language Fundamentals
  * structs and enums; Option and Result
  * Scopes and Lifetimes. Lifetime bindings. Borrowed return values.
  * Generic items; Trait-bounded polymorphism
  * Traits as Objects
* Part 4: Systems Development
  * Concurrency: `Send`, `Sync`, `thread::scoped`
  * I/O in Rust, `try!`
  * Concurrency: `thread::spawn`, channels
  * unsafe code, FFI
  * Rust in Gecko (demonstration)

```rust
pub mod whistler_rust_intro;
pub mod ex_part_1;
pub mod ex_part_3;
```

### FFI and callbacks

Quicksort

```rust
// The basics of Rust comments
mod comments;


// Themes: Getting Started, Ownership and Move Semantics
#[cfg(test)] mod section010;
// Theme: Borrowing
#[cfg(test)] mod section020;
// Theme: Ownership and Exclusive Access, Mutable Borrowing
#[cfg(test)] mod section030;
// Theme: Vector versus Array versus Slice
#[cfg(test)] mod section040;
// Theme: Iterators; String/str/[u8]/[char]
#[cfg(test)] mod section050;

// Theme: Borrowing revisited: Generic Lifetime Bindings
#[cfg(test)] mod section060 { }

// At this point my hope is that everyone in the room has run the Rust
// installer and thus we can shift from doing exercises via the
// playpen to each doing exercises on their own laptop, and thus we
// can shift to discussing tools like `cargo` directly.

// Theme: Getting started with Cargo
#[cfg(test)] mod section070 { }


mod exercise_ffi;

#[test] fn ex010() { section010::main(); }
#[test] fn ex020() { section020::main(); }
#[test] fn ex030() { section030::main(); }
#[test] fn ex040() { section040::no_longer_main(); }
#[test] fn ex050() { section050::no_longer_main(); }
```
