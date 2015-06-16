## Rust Basics

```rust
pub fn main() {
    println!("Hello World!"); // (`!` means macro)
    print_ten();                                               
}

fn print_ten() {
    println!("Ten: {}", 10);
    //             ~~   ~~
    //             |     | 
    //    placeholder   argument
}
```

## "Usual" stuff

  * Local variables: `let x = 3;`{.rust}
    * Optional type annotations: `let x: i32 = 3;`{.rust}

  * If expressions:
    `if test { then(); and(also) } else { other() }`{.rust}

  * Loops: `while test { first(); second(); ... }`{.rust}
    * Also `for i in iterator { f(); g(); ... }`{.rust}

  * Standalone functions
    `fn add8(x: i32) { println!("add8"); return x + 8; }`{.rust}

## Unusual stuff

  * Blocks have optional tail expressions
    `fn add8(x: i32) { println!("add8"); return x + 8; }`{.rust}
    `fn add8(x: i32) { println!("add8"); x + 8 }`{.rust}

  * Tuples, Pattern binding

```rust
fn add_pair8(p: (i32, i32)) -> (i32, i32) {
    let (x,y) = p;
    (x + 8, y + 8)
}
```

## Language and API docs

  * All linked from top of [http://www.rust-lang.org/]

  * Starting points
    * The Book: [https://doc.rust-lang.org/stable/book/]
    * Standard API: [https://doc.rust-lang.org/stable/std/]

[http://www.rust-lang.org/]: http://www.rust-lang.org/
[https://doc.rust-lang.org/stable/book/]: https://doc.rust-lang.org/stable/book/
[https://doc.rust-lang.org/stable/std/]: https://doc.rust-lang.org/stable/std/]
