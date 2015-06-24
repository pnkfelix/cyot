# Borrowing

## Moves insufficient on their own

Imagine programming without reuse

``` { .rust .compile_error }
#[test]
fn moves_insufficient() {
    let vec = expensive_vector_computation();

    let result1 = some_vec_calculation(vec); // <-- `vec` moved here

    let result2 = other_calculation(vec); // oops, `vec` is *gone*

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

} // (`vec` is destroyed/freed aka "dropped" here)
```

``` {.fragment}
                                    &vec
                                    ~~~~
                                      |
                              a borrow expression
```

. . .

("mo' features, mo' problems")

## Big Question

* Why are safety violations generally hard to detect?

. . .

* It is due to *aliasing*

* Borrows *reintroduce* aliasing

### Q: How to ensure safety in presence of aliasing?  {.fragment}

### A: Restrict the aliasing {.fragment}

## Simple metaphor: RW lock

  * Read-only operations do not require exclusive access

  * Exclusive access requires there are no other readers

Rust uses analogous model (at compile-time) for borrows

## Borrowing: Basic Mental Model { .left_align }

. . .

   * Base types `T`{.rust}
      * e.g. `char`{.rust}, `Vec<i32>`{.rust}
      * If type copyable, then you can always copy it
      * You can *move* it only if no borrow active

. . .

   * Immutable borrows: `&T`{.rust}
      * "Read-only." Freely aliasable; copyable
      * (i.e. "many readers")

. . .

   * Mutable borrows: `&mut T`{.rust}
      * Read/Write. Exclusive access; non-copy
      * (i.e. "at most one writer")

<!--
  * (Why the quotes? "[interior mutability]")
-->

[interior mutability]: https://doc.rust-lang.org/nightly/book/mutability.html#interior-vs.-exterior-mutability

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
    fn consume(v: Vec<i32>) { /* `v` *dropped* at scope end */ }
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

## Built on lexical scopes, but non-trivial { data-transition="slide-in fade-out" }

```rust
#[test]
fn copying_can_extend_a_borrows_lifetime_1() {
    fn foo<'a>(v: &'a Vec<i32>) { println!("v[1]: {}", v[1]); }
    let v1 = vec![1, 2, 3]; //         +
    let v2 = vec![4, 5, 6]; //         |    +
    let r2 = {              //         |    |
        let r1 = &v1;       //  +      |    |
        //       ~~~        //  |      |    |
        foo(r1);            // 'r1     |    |
        &v2                 //  |     'v1  'v2
    };                      //  +  +   |    |
    // (maybe mutate `v1`   //     |   |    |
    // here someday?)       //     |   |    |
                            //    'r2  |    |
    foo(r2);                //     |   |    |
}                           //     +   +    +
```

How long should the borrow `&v1` last?

## Built on lexical scopes, but non-trivial { data-transition="fade-in slide-out" }

```rust
#[test]
fn copying_can_extend_a_borrows_lifetime_2() {
    fn foo<'a>(v: &'a Vec<i32>) { println!("v[1]: {}", v[1]); }
    let v1 = vec![1, 2, 3]; //         +
    let v2 = vec![4, 5, 6]; //         |    +
    let r2 = {              //         |    |
        let r1 = &v1;       //  +      |    |
        //       ~~~        //  |      |    |
        foo(r1);            // 'r1     |    |
        r1                  //  |     'v1  'v2
    };                      //  +  +   |    |
    // (maybe mutate `v1`   //     |   |    |
    // here someday?)       //     |   |    |
                            //    'r2  |    |
    foo(r2);                //     |   |    |
}                           //     +   +    +
```

How long should the borrow `&v1` last now?

<!--

## Lifetime Nesting  { data-transition="slide" }

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

## Lexical Scopes, but Nontrivial { data-transition="slide-in fade-out" }

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

-->

## imm-borrows: can be copied freely { data-transition="slide-out" }

(super super useful to be able to share readable data!)

## imm-borrows: can be copied freely { data-transition="slide-in fade-out" }

Implications:

  * must assume aliased (perhaps by another thread)
  * therefore *not safe* to mutate in general

``` {.rust .compile_error}
#[test]
fn demo_cannot_mutate_imm_borrow() {
    let mut v1 = vec![1, 2, 3];
    let b = &v1;
    let (b1, b2, b3) = (b, b, b);
    try_modify(b);
    println!("v1: {:?}", v1);
}

fn try_modify(v: &Vec<i32>) {
    v.push(4);
}
```

```{ .fragment .compile_error }
error: cannot borrow immutable borrowed content `*v` as mutable
    v.push(4);
    ^
```

## imm-borrows: can be copied freely { data-transition="fade-in slide-out" }

Implications:

  * must assume aliased (perhaps by another thread)
  * therefore *not safe* to mutate in general

``` {.rust .compile_error}
#[test]
fn demo_cannot_mutate_imm_borrow() {
    let mut v1 = vec![1, 2, 3];
    let b = &v1;
    let (b1, b2, b3) = (b, b, b);
    try_modify(b);
    println!("v1: {:?}", v1);
}

fn try_modify(v: &Vec<i32>) {
    v.push(4);
}
```

```
WHAT
      A
         BUMMER!!!
```

## "... i want my imperative algorthms! ..."

## `&mut`{.rust} borrows


```rust
#[test]
fn demo_can_mutate_mut_borrow() {
    let mut v1 = vec![1, 2, 3];
    modify(&mut v1);
    println!("v1: {:?}", v1);
}

fn modify(v: &mut Vec<i32>) {
    v.push(4);
}
```

```{ .fragment }
v1: [1, 2, 3, 4]
```

## What does `&mut`{.rust} mean (crucial) { data-transition="zoom-out" }

`&mut`{.rust} is *not* about "being the only way to mutate"

  * It is about *exclusive access*

An operation requiring exclusive access should either:

  * take ownership, or,

  * take an `&mut`{.rust}-reference

## `&mut`{.rust} is about exclusive access { data-transition="zoom-in" }

"`mut`{.rust} means 'mutate' ..." is a fiction

. . .

For many types, safe mutation *does* require exclusive access

```{.rust}
vec.push(4);
// requires `vec: &mut Vec<_>`, for safe manipulation of backing store
```

"`mut`{.rust} means 'mutate' ..." is a *convenient fiction*

. . . 

(For related naming drama, do a search for: "mutpocalypse")

## `&mut`{.rust} safety enforcement


## Data has at most one `&mut`{.rust} borrow { data-transition="slide-in fade-out" }

```rust
fn take2<'a>(v1: &'a mut Vec<i32>, v2: &'a Vec<i32>) { }
```

``` { .rust .compile_error }
#[test]
fn demo_cannot_mut_borrow_multiple_times() {
    let mut v1 = vec![1, 2, 3];
    let mut v2 = vec![1, 2, 3];
    take2(&mut v1, &mut v2); // <-- this is okay
    take2(&mut v1, &mut v1);
}
```

``` { .fragment .compile_error }
error: cannot borrow `v1` as mutable more than once at a time
    take2(&mut v1, &mut v1);
                        ^~
note: previous borrow of `v1` occurs here; the mutable borrow
      prevents subsequent moves, borrows, or modification of
      `v1` until the borrow ends
    take2(&mut v1, &mut v1);
               ^~


```

## Cannot alias `&mut`{.rust}-borrowed data { data-transition="fade" }

```{.rust}
fn take2<'a>(v1: &'a mut Vec<i32>, v2: &'a Vec<i32>) { }
```

``` {.rust .compile_error }
#[test]
fn demo_cannot_alias_mut_borrowed_data() {
    let mut v1 = vec![1, 2, 3];
    let mut v2 = vec![1, 2, 3];
    take2(&mut v1, &v2); // <-- this is okay
    take2(&mut v1, &v1);
}
```

``` { .fragment .compile_error }
error: cannot borrow `v1` as immutable because it is also borrowed
       as mutable
    take2(&mut v1, &v1);
                    ^~
note: previous borrow of `v1` occurs here; the mutable borrow 
      prevents subsequent moves, borrows, or modification of `v1`
      until the borrow ends
    take2(&mut v1, &v1);
               ^~
```

## `&mut T`{.rust} is non-copy { data-transition="fade-in slide-out" }

```{.rust}
fn take2<'a>(v1: &'a mut Vec<i32>, v2: &'a Vec<i32>) { }
```

``` {.rust .compile_error }
#[test]
fn demo_cannot_copy_mut_borrows() {
    let mut v1 = vec![1, 2, 3];
    let b = &mut v1;
    let c = b;
    take2(b, c);
}
```

``` { .fragment .compile_error }
error: use of moved value: `*b` [E0382]
    take2(b, c);
          ^
note: `b` moved here because it has type
      `&mut collections::vec::Vec<i32>`, which is moved by default
    let c = b;
        ^
```

(ensures exclusive access)

## Exclusive Access versus Ownership

```rust
fn take_by_value(v: Vec<i32>) { let mut v = v; v.push(4);  }
fn take_mut_borrow(b: &mut Vec<i32>) { b.push(10); }
// seemingly similar in power
```
. . .

``` {.rust .compile_error}
#[test]
fn demo_exclusive_access_versus_ownership() {
    let mut v1 = vec![1, 2, 3];
    let mut v2 = vec![7, 8, 9];
    take_by_value(v1);
    take_mut_borrow(&mut v2);
    println!("v1: {:?} v2: {:?}", v1, v2);
}
```

``` { .fragment .compile_error }
error: use of moved value: `v1` [E0382]
    println!("v1: {:?} v2: {:?}", v1, v2);
                                  ^~
note: `v1` moved here because it has type
      `collections::vec::Vec<i32>`, which is non-copyable
    take_by_value(v1);
                  ^~
```

. . .

ownership ⇒ moves; power + responsibility for dropping
