  * [Section 4: Rust tooling](#section-4-rust-tooling)
    <!-- TODO: add exercises testing understanding of Rust mod system. -->
  * [Section 5 Vectors and slices](#section-5-vectors-and-slices)
  * [Section 6 for loops; strings](#section-6-for-loops-strings)

## Section 4: Rust tooling

This section's exercises are meant to be run on your local machine.
Make sure you have successfully installed Rust and Cargo on your local
machine; ask a helper for assistance if necessary!

### Core Exercises

#### Exercise 4.1: Hello World, locally

Compile and run Hello World locally via `rustc`

`hello.rs`{ .filename }
``` {.rust}
pub fn main() { println!("Hello World"); }
```

#### Exercise 4.2: Timing a Rust program

Put the following code into a file named `count.rs`.

`count.rs`{ .filename }
```rust
fn count_to(p: &mut u64, mask: u64, max: u64) {
    let mut i = *p;
    while i < max {
        if i & mask != 0 {
            *p += 1;
        }
        i += 1;
    }
}

fn main() {
    let mut x = 0;
    count_to(&mut x, !0, 1 << 25);
    println!("x: {}", x);
}
```

Compile the program with `rustc` with no other flags, and then time how long it
takes to run. For example, on Unix systems:

```
$ rustc count.rs
$ time ./count
```

Now compile the program with optimizations turned on (the `-O` flag), and compare how the timing
results turn out:

```
$ rustc -O count.rs
$ time ./count
```

How much of difference does this make on your platform?

You can investigate differences in the generated code via command-line options:

  * `rustc --emit=asm` will produce assembly for your target platform
  * `rustc --emit=llvm-ir` will produce LLVM intermediate representation 

(Also, you may have noticed that the program is quite a bit more complicated
than something that just counts to `max`. Feel free to try replacing the
`i & mask != 0` condition with `true` and seeing how the results change.)


#### Exercise 4.3: Hello Cargo

Make a new cargo library and run its unit tests:

```
$ cargo new my_new_library
$ cd my_new_library
$ cargo build
...
$ cargo test
...
```

#### Exercise 4.4: test I/O

Revise your new cargo library so that the unit test
prints out "Hello World" via `println!`{.rust} when it runs.

Note: `cargo test` captures such output by default, so you
will not see it when you run `cargo test` with no other options.

You can see options to adjust the latter behavior,
that you can pass to the generated tester program,
by running `cargo test -- --help`.

(Note also that `cargo test --help` is quite different from
`cargo test -- --help`.)

#### Exercise 4.5: crates.io

Revise your new cargo library to print out a randomly generated number.

To do this, you should not write your own random number generator;
instead, you should grab one off [crates.io].

Do a search there for one, and then add the appropriate line to a
`[dependencies]` section in `my_new_library/Cargo.toml`.

[crates.io]: https://crates.io/

----

## Section 5: Vectors and Slices

This section asks you to write various functions.

To write and test your code, you can either:

  * Put the code into a cargo library and run `cargo test`,

  * Put the code into a `.rs` file, compile it with `rustc --test`,
    and run the resulting binary, or,

  * Put the code into a `.rs` file with a `fn main` function and
    compile + run it the same way you did the earlier "Hello
    World"-type examples.

(We recommend `cargo`.)

Also:

  * The `Vec` API is visible at:

    [https://doc.rust-lang.org/stable/std/vec/struct.Vec.html]

  * The `[T]` (slice) API is visible at:

    [https://doc.rust-lang.org/stable/std/primitive.slice.html]

[https://doc.rust-lang.org/stable/std/vec/struct.Vec.html]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html

[https://doc.rust-lang.org/stable/std/primitive.slice.html]: https://doc.rust-lang.org/stable/std/primitive.slice.html

### Core Exercises

#### Exercise 5.1

Write a function:

``` {.rust}
fn zeroes(count: usize) -> Vec<usize>
```

that creates a vector of length `count` that is filled with 0.

#### Exercise 5.2

Write a function:

``` {.rust}
fn histogram(input_data: &[usize]) -> Vec<usize>
```

that reports, at each index `i` of its result vector, the number of
times that `i` occurs in `input_data`.

So for example, these assertions should hold:

``` {.rust}
assert_eq!(histogram(&[4, 0, 4, 4]),
           [1, 0, 0, 0, 3]);
assert_eq!(histogram(&[4, 0, 4, 4, 5, 0, 9, 9, 9, 9, 9]),
           [2, 0, 0, 0, 3,
            1, 0, 0, 0, 5]);
```

#### Exercise 5.3

You may have seen an earlier note that it is not idiomatic in Rust to
take an immutably-borrowed `&Vec<T>` argument; idiomatic Rust instead
uses borrowed slices `&[T]`.

Do you think this reasoning applies also to `&mut Vec<T>`?  That
is, for any function that takes `&mut Vec<T>`, could we make a
replacement function that instead takes `&mut [T]` and everything
still works out?

If you are not sure of the answer: Go back over the previous
section with exercises writing functions that took `&mut Vec<i32>`,
and write those same functions but now taking `&mut [i32]` instead.

----

## Section 6: for loops; strings

To run them, you can either:

  * Put the code into a cargo library and run `cargo test`,

  * Put the code into a `.rs` file, compile it with `rustc --test`,
    and run the resulting binary, or,

  * Put the code into a `.rs` file, replace `fn no_longer_main` with
    `fn main` and compile it the same way you did the earlier "Hello
    World"-type examples.

Also:

  * The `str` API is visible at:

    [https://doc.rust-lang.org/stable/std/primitive.str.html]

  * The `String` API is visble at:

    [https://doc.rust-lang.org/stable/std/string/struct.String.html]

[https://doc.rust-lang.org/stable/std/primitive.str.html]: https://doc.rust-lang.org/stable/std/primitive.str.html

[https://doc.rust-lang.org/stable/std/string/struct.String.html]: https://doc.rust-lang.org/stable/std/string/struct.String.html

The exercises in this section concern the code at the following link,
which uses unit tests alone for its illustrations:

  * gist:    [https://gist.github.com/pnkfelix/e0916c0a8abda3331293]
  * playpen: [https://play.rust-lang.org/?gist=e0916c0a8abda3331293]

You may prefer to put the code on your local machine (either in its
own `.rs` file, or in a cargo project).

[https://gist.github.com/pnkfelix/e0916c0a8abda3331293]: https://gist.github.com/pnkfelix/e0916c0a8abda3331293

[https://play.rust-lang.org/?gist=e0916c0a8abda3331293]: https://play.rust-lang.org/?gist=e0916c0a8abda3331293

### Core Exercises

#### Exercise 6.1

What is being printed out at the end of `fn no_longer_main`?

Hint: Running the program is a reasonable way to resolve this question!

#### Exercise 6.2

Uncomment the lines beneath the one labelled "(*)" above, starting
with `let borrowed = ...;`

Re-run the test suite. Can you explain what you see?

Hint: setting the environment variable `RUST_BACKTRACE` to 1
will make Rust programs print out a stack trace when they panic.

So for example, `RUST_BACKTRACE=1 cargo test` may provide
further information about what is happening.

#### Exercise 6.3

Write a function

``` {.rust}
fn listing(input: &[&str]) -> String
```

that makes a comma-delimited list of all the input strings.

Examples:

  * ["apple", "pear", "banana"] goes to "apple, pear, banana"

  * An emply slice goes to "".
