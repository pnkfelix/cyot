## rustc

```sh-session
[bash]$ cat /tmp/hello.rs
```
`/tmp/hello.rs`{.filename}
``` {.rust}
fn main() { println!("hello world"); }
```
```sh-session
[bash]$ rustc /tmp/hello.rs -o /tmp/hello
[bash]$ /tmp/hello
hello world
[bash]$
```

## Getting started with cargo

```sh-session
[bash]$ cargo new my-new-crate
[bash]$ cd my-new-crate
[bash]$ find * -type f
Cargo.toml
src/lib.rs
[bash]$ cat src/lib.rs
```
`src/lib.rs`{.filename}
```rust
#[test]
fn it_works() {
}
```
```sh-session
[bash]$ cargo build
   Compiling my-new-crate v0.1.0 (file:///private/tmp/my-new-crate)
[bash]$ 
```

----

```sh-session
[bash]$ cargo test
   Compiling my-new-crate v0.1.0 (file:///private/tmp/my-new-crate)
     Running target/debug/my_new_crate-7ad82271427661a1

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests my-new-crate

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

[bash]$ 
```

----

```sh-session
[bash]$ cd ..
[bash]$ cargo new --bin my-rust-program
[bash]$ cd my-rust-program
[bash]$ find * -type f
Cargo.toml
src/main.rs
[bash]$ cat src/main.rs
```
`src/main.rs`{.filename}
``` {.rust}
fn main() {
    println!("Hello, world!");
}
```
```sh-session
[bash]$ cargo run
   Compiling my-rust-program v0.1.0 (file:///private/tmp/my-rust-program)
     Running `target/debug/my-rust-program`
Hello, world!
[bash]$ 
```
