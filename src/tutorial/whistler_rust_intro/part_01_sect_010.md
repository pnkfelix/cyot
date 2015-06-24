## Lets dive in

```rust
pub fn main() {
    println!("Hello World!"); // (`foo!` means macro)
    print_ten();                                               
}

fn print_ten() {
    println!("Ten: {}", 10);
    //             ~~   ~~
    //             |     | 
    //    placeholder   argument
}
```

<!--
## Usual stuff

  * Local variables: `let x = 3;`{.rust}
    * Optional type annotations: `let x: i32 = 3;`{.rust}

  * If expressions:
    `if test { then(); also() } else { other() }`{.rust}

  * Loops: `while test { first(); second(); ... }`{.rust}
    * Also `for i in iterator { f(); g(); ... }`{.rust}

  * Standalone functions:

    ```rust
    fn add8(x: i32) -> i32 {
        println!("add8");
        return x + 8;
    }
    ```

## Unusual stuff 1

  * Blocks have optional tail expressions
  * Tuples, Pattern binding
  * Algebraic Data, Pattern matching

``` {.rust}
fn add8(x: i32) -> i32 { return x + 8; }
fn add8(x: i32) -> i32 { x + 8 }
```

``` {.rust}
let (x, y): (i32, i32) = int_pair;
(x + 8, y + 8)
```

``` {.rust}
let v = match number_result {
    Ok(x)  => x + 8,
    Err(e) => { println!("error: {}", e); break; }
};
```

## Unusual stuff 2

  * Generics, Trait Bounds
  * Objects via Traits
  * Ownership and Borrowing

  We start with *Ownership and Borrowing*;
  a critical concept for understanding Rust.

-->
