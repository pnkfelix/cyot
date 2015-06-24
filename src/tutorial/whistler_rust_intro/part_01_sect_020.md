# Borrowing

## Moves insufficient on their own

``` { .rust .compile_error }
#[test]
fn moves_insufficient() {
    let vec = expensive_vector_computation();

    let result1 = some_vec_calculation(vec); // <-- `vec` moved here

    let result2 = other_calculation(vec); // oops

    combine(result1, result2);

}
```
<!--
fn expensive_vector_computation() -> Vec<i32> { vec![] }
fn some_vec_calculation<T>(t: T) { }
fn other_calculation<T>(t: T) { }
fn combine(x: (), y: ()) { }
-->

``` {.fragment}
error: use of moved value: `vec` [E0382]
    let result2 = other_calculation(vec); // oops
                                    ^~~
note: `vec` moved here because it has type
      `collections::vec::Vec<i32>`, which is non-copyable
    let result1 = some_vec_calculation(vec); // <-- `vec` moved here
                                       ^~~
```

## Want: access to owned data *without* consuming it

## Thus, "borrowing"

``` { .rust }
#[test]
fn moves_insufficient() {
    let vec = expensive_vector_computation();

    let result1 = some_vec_calculation(&vec); // <-- lend out `vec`

    let result2 = other_calculation(&vec); // <-- lend again, no prob

    combine(result1, result2);

} // (`vec` is destroyed/freed here)
```

``` {.fragment}
                                    &vec
                                    ~~~~
                                      |
                              a borrow expression
```

## Big Question

* Bugs are hard to detect, due to *aliasing*

* Borrows *reintroduce* aliasing

### Q: How to ensure safety in presence of aliasing? { .fragment }

### A: Restrict the aliasing { .fragment }

## Simple metaphor: RW lock

  * Read-only operations do not require exclusive access

  * Exclusive access requires there are no other readers

Rust uses analogous model (at compile-time) for borrows

## Borrowing: Basic Mental Model

  * Base types `T`

  * Immutable borrows: `&T`{.rust}

    * "Read-only." Freely aliasable; copyable

  * Mutable borrows: `&mut T`{.rust}

    * Read/Write. Exclusive access; non-copy

## Immutable borrows

## Borrowing (immutably) { data-transition="fade-out" }

```rust
#[test]
fn show_some_borrows() {

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    let r1 = &v1;
    let r2 = &v2;
    foo(r1);
    foo(r2);

}
```
<!-- -->
```rust
fn foo<'a>(v: &'a Vec<i32>) { println!("v[1]: {}", v[1]); }
```

`&v1` and `&v2` are *borrowing* `v1` and `v2`.

## Scopes and Lifetimes { data-transition="fade-in" }

```rust
#[test]
fn show_some_lifetimes() {

    let v1 = vec![1, 2, 3]; //                 +
    let v2 = vec![4, 5, 6]; //            +    |
                            //            |    |
    let r1 = &v1;           //       +    |    |
    let r2 = &v2;           //  +    |    |    |
    foo(r1);                //  |    |    |    |  
    foo(r2);                // 'r2  'r1  'v2  'v1
                            //  |    |    |    | 
}                           //  +    +    +    +
```

``` {.rust}
fn foo<'a>(v: &'a Vec<i32>) { println!("v[1]: {}", v[1]); }
```

Each borrow selects "appropriate" lifetime `'a`.

## Borrow Checking Prevents Errors { data-transition="fade-out" }

``` {.rust .compile_error}
fn borrow_checking_prevents_errors() {

    let v1 = vec![1, 2, 3];      //        +
                                 //        |
    let r1 = &v1;                //  +    'v1
                                 //  |     |
    consume(v1);                 // 'r1   (moved)
    foo(r1);                     //  |
}                                //  +
```

```{.rust}
    fn foo<'a>(v: &'a Vec<i32>) { println!("v[1]: {}", v[1]); }
    fn consume(v: Vec<i32>) { }
```

`foo(r1)` attempts an indirect read of `v1`

``` {.fragment}
error: cannot move out of `v1` because it is borrowed
    consume(v1);
            ^~
note: borrow of `v1` occurs here
    let r1 = &v1;
              ^~
```

## Lifetimes and Lexical Scope { data-transition="fade-in" }

``` {.rust .compile_error}
fn borrow_checking_may_seem_simple_minded() {

    let v1 = vec![1, 2, 3];      //        +
                                 //        |
    let r1 = &v1;                //  +    'v1
                                 //  |     |
    consume(v1);                 // 'r1   (moved)
    // (no call to read)         //  |
}                                //  +
```

```{.rust}
    fn foo<'a>(v: &'a Vec<i32>) { println!("v[1]: {}", v[1]); }
    fn consume(v: Vec<i32>) { }
```

```
error: cannot move out of `v1` because it is borrowed
    consume(v1);
            ^~
note: borrow of `v1` occurs here
    let r1 = &v1;
              ^~
```

(artifact of lexical-scope based implementation)

## Lifetime nesting  { data-transition="slide" }

```rust
#[test]
fn lifetime_nesting() {
    let v1 = vec![1, 2, 3];         //                      +
    let r1;                         //                      |
    {                               //                      |
        let v2 = vec![4, 5, 6];     //                 +    |
        {                           //                 |    |
            let r2 = &v2;           //  +              |    |
            let v3 = vec![7,8,9];   //  |         +   'v2  'v1
            r1 = &v1;               // 'r2   +   'v3   |    |
            foo(r2);                //  |    |    |    |    |
        }                           //  +   'r1   +    |    |
    }                               //       |         +    |
    foo(r1);                        //       |              |
}                                   //       +              +
```

``` {.rust}
fn foo<'a>(v: &'a Vec<i32>) { println!("v[1]: {}", v[1]); }
```

borrow `&v2` is for at least `'r2`, but at most `'v2`

## Lexical scopes, but Nontrivial { data-transition="slide-in fade-out" }

```rust
#[test]
fn copying_can_extend_a_borrows_lifetime() {
    fn foo<'a>(v: &'a Vec<i32>) {
        println!("v[1]: {}", v[1]);
    }
    let v1 = vec![1, 2, 3]; //         +
    let r2 = {              //         |
        let r1 = &v1;       //  +      |
        //       ^~~        //  |      |
        foo(r1);            // 'r1     |
        r1                  //  |     'v1
    };                      //  +  +   |
                            //     |   |
    foo(r2);                //    'r2  |
                            //     |   |
}                           //     +   +
```

``` {.rust}
fn foo<'a>(v: &'a Vec<i32>) { println!("v[1]: {}", v[1]); }
```

(How long does the borrow `&v1` last? Does `'r1` suffice?)

## Lexical Scopes, but Nontrivial { data-transition="fade-in" }

``` {.rust}
#[test]
fn copying_can_extend_a_borrows_lifetime() {
    fn foo<'a>(v: &'a Vec<i32>) {
        println!("v[1]: {}", v[1]);
    }
    let v1 = vec![1, 2, 3]; //         +
    let r2: &'y Vec<i32> = {//         |         'y >= 'r2
        let r1 = &'z v1;    //  +      |  'v1 >= 'z >= 'r1
        //       ^~~~~~     //  |      |
        foo(r1);//  |       // 'r1     |
        r1      // (caveat) //  |     'v1
    };                      //  +  +   |         'z == 'y
                            //     |   |
    foo(r2);                //    'r2  |
                            //     |   |
}                           //     +   +
```

``` {.rust}
fn foo<'a>(v: &'a Vec<i32>) { println!("v[1]: {}", v[1]); }
```

`'r1` too short! (caveat: above is not legal Rust)
