# Part 3: Back to Language Fundamentals

## structs

```rust
#[derive(Copy, Clone)]
struct Point { pub x: i32, pub y: i32 }

fn proj_x_axis(p: &Point) -> Point {
    Point { x: p.x, y: 0 } }

fn nudge_left(p: &mut Point) { p.x -= 10; }

#[derive(Copy, Clone)]
struct Angle(pub f64);

struct Pair<X,Y>(X, Y);
```

----

  * FIXME does this slide actually pay for itself?
    * (`impl` alone?)

```{.rust}
fn proj_x_axis(p: &Point) -> Point {
    Point { x: p.x, y: 0 }
}
```

```{.rust}
fn proj_x_axis(p: &Point) -> Point {
    Point { y: 0, ..p }
}
```

<!--
```{.rust}
fn proj_x_axis_check(p: &Point) -> Point {
    Point { y: 0, ..p }
}
```
-->

```rust
impl Point {
    fn proj_x_axis(&self) -> Point {
        Point { x: self.x, y: 0 }
    }
}
```

## enums

```rust
enum Line {
    Segment { start: Point, end: Point },
    Vector(Point, Angle),
}

fn start(l: &Line) -> Point {
    match *l {
        Line::Vector(s, _) => s,
        Line::Segment { start: s, .. } => s,
    }
}
```

## Option and Result

``` {.rust}
enum Option<T> { Some(T), None }

enum Result<T, E> { Ok(T), Err(E) }
```

```rust
use std::num::ParseIntError;
fn read_u32(s: &str) -> Result<u32, ParseIntError> {
    s.parse()
}
#[test]
fn demo() {
    assert_eq!(read_u32("4"), Ok(4));
    assert!(read_u32("4_no_no").is_err());

    assert_eq!(read_u32("4").ok(), Some(4));
}
```

## Scopes and Lifetimes.

```rust
#[test]
fn show_some_lifetimes() {
    let v1 = vec![1, 2, 3]; //                 +
    let v2 = vec![4, 5, 6]; //            +    |
    let b1 = &v1;           //       +    |    |
    let b2 = &v2;           //  +    |    |    |
    foo(b1);                //  |    |    |    |  
    foo(b2);                // 'b2  'b1  'v2  'v1
                            //  |    |    |    | 
}                           //  +    +    +    +

fn foo(v: &Vec<i32>) { println!("v[1]: {}", v[1]); }
```

## Lifetime nesting

```rust
#[test]
fn lifetime_nesting() {
    fn read(v: &Vec<i32>) { println!("v[1]: {}", v[1]); }
    let v1 = vec![1, 2, 3];         //                      +
    let b1;                         //                      |
    {                               //                      |
        let v2 = vec![4, 5, 6];     //                 +    |
        {                           //                 |    |
            let b2 = &v2;           //  +              |    |
            let v3 = vec![7,8,9];   //  |         +   'v2  'v1
            b1 = &v1;               // 'b2   +   'v3   |    |
            read(b2);               //  |    |    |    |    |
        }                           //  +   'b1   +    |    |  
    }                               //       |         +    |  
    read(b1);                       //       |              | 
}                                   //       +              +
```

## Borrow Checking Prevents Errors { data-transition="fade-out" }

``` {.rust .compile_error}
fn borrow_checking_prevents_errors() {
    fn read(v: &Vec<i32>) { println!("v[1]: {}", v[1]); }
    fn consume(v: Vec<i32>) { }
    let v1 = vec![1, 2, 3];      //        +
                                 //        |
    let b1 = &v1;                //  +    'v1
                                 //  |     |
    consume(v1);                 // 'b1   (moved)
    read(b1);                    //  |
}                                //  +
```

``` {.fragment}
error: cannot move out of `v1` because it is borrowed
    consume(v1);                 // 'b1   (moved)
            ^~
note: borrow of `v1` occurs here
    let b1 = &v1;                //  +    'v1
              ^~
```

## Lifetimes and Lexical Scope { data-transition="fade-in" }

``` {.rust .compile_error}
fn borrow_checking_may_seem_simple_minded() {
    fn read(v: &Vec<i32>) { println!("v[1]: {}", v[1]); }
    fn consume(v: Vec<i32>) { }
    let v1 = vec![1, 2, 3];      //        +
                                 //        |
    let b1 = &v1;                //  +    'v1
                                 //  |     |
    consume(v1);                 // 'b1   (moved)
    // (no call to read)         //  |
}                                //  +
```

```
error: cannot move out of `v1` because it is borrowed
    consume(v1);                 // 'b1   (moved)
            ^~
note: borrow of `v1` occurs here
    let b1 = &v1;                //  +    'v1
              ^~
```

## Lifetimes: Nonetheless Nontrivial { data-transition="fade-out" }

```rust
#[test]
fn copying_can_extend_a_borrows_lifetime() {
    fn foo(v: &Vec<i32>) {
        println!("v[1]: {}", v[1]);
    }
    let v1 = vec![1, 2, 3]; //         +
    let b2 = {              //         |
        let b1 = &v1;       //  +      |
        //       ^~~        //  |      |
        foo(b1);            // 'b1     |
        b1                  //  |     'v1
    };                      //  +  +   |
                            //     |   |
    foo(b2);                //    'b2  |
                            //     |   |
}                           //     +   +
```

## Lifetimes: Nonetheless Nontrivial { data-transition="fade-in" }

``` {.rust}
#[test]
fn copying_can_extend_a_borrows_lifetime() {
    fn foo(v: &Vec<i32>) {
        println!("v[1]: {}", v[1]);
    }
    let v1 = vec![1, 2, 3]; //         +
    let b2 = {              //         |
        let b1 = &'a v1;    //  +      |
        //       ^~~~~~     //  |      |
        foo(b1);//  |       // 'b1     |
        b1      // (caveat) //  |     'v1 == 'a
    };                      //  +  +   |
                            //     |   |
    foo(b2);                //    'b2  |
                            //     |   |
}                           //     +   +
```

## Lifetime Bindings 1

```rust
#[test]
fn explicit_lifetime_binding_1() {
    fn print<'a>(ints: &'a Vec<i32>) {
        println!("v_1: {}", ints[1]);
    }
    let v1 = vec![1, 2, 3];
    print(&v1)
}
```

## Lifetime Bindings 2  { data-transition="fade-out" }

```rust
#[test]
fn explicit_lifetime_binding_2() {
    fn print<'a, 'b>(ptrs: &'a Vec<&'b i32>) {
        println!("v_1: {}", ptrs[1]);

    }
    let one = 1;
    let two = 2;
    let three = 3;
    let four = 4;
    let v1 = vec![&one, &two, &three];
    print(&v1)
}
```

## Lifetime Bindings 3  { data-transition="fade" }

```rust
#[test]
fn explicit_lifetime_binding_3() {
    fn print<'a, 'b>(ptrs: &'a mut Vec<&'b i32>, ptr: &'b i32) {
        println!("v_1: {}", ptrs[1]);
        ptrs.push(ptr);
    }
    let one = 1;
    let two = 2;
    let three = 3;
    let four = 4;
    let mut v1 = vec![&one, &two, &three];
    print(&mut v1, &four);
}
```

## Lifetime Bindings 4  { data-transition="fade-in" }

```rust
#[test]
fn explicit_lifetime_binding_4() {
    fn print<'a, 'b>(ptrs: &'a mut Vec<&'b i32>, ptr: &'b i32) {
        println!("v_1: {}", ptrs[1]);//~~~            ~~~
        ptrs.push(ptr);            //   |              |
    }                              // this must match that,
    let one = 1;                   // otherwise push is bogus
    let two = 2;
    let three = 3;
    let four = 4;
    let mut v1 = vec![&one, &two, &three];
    print(&mut v1, &four);
}
```

## Lifetime Bindings 5  { data-transition="fade-in" }

``` {.rust .compile_error}
#[test]
fn explicit_lifetime_binding_5() {
    fn print<'a, 'b, 'c>(ptrs: &'a mut Vec<&'b i32>, ptr: &'c i32) {
        println!("v_1: {}", ptrs[1]);  //  ~~~            ~~~
        ptrs.push(ptr);                //   |              |
    }                                  // this must match that,
    let one = 1;                       // otherwise push is bogus
}
```

``` {.fragment}
error: cannot infer an appropriate lifetime for automatic coercion due to conflicting requirements
        ptrs.push(ptr);
                  ^~~
help: consider using an explicit lifetime parameter as shown:
    fn print<'a, 'b>(ptrs: &'a mut Vec<&'b i32>, ptr: &'b i32)
```

## Borrowed return values.

```rust
#[test]
fn borrowed_return_values() {
    fn first_and_last<'a>(ints: &'a Vec<i32>) -> (&'a i32, &'a i32) {
        (&ints[0], &ints[ints.len() - 1])
    }
}
```

TODO: Exercise idea: Try to write `fn first_and_last_mut`. Why is it impossible
in general?

----

How about:

``` {.rust .compile_error}
#[test]
fn borrowed_return_values_bad() {
    fn first_and_last<'a>(ints: Vec<i32>) -> (&'a i32, &'a i32) {
        (&ints[0], &ints[ints.len() - 1])
    }
}
```

Why doesn't this work?

``` {.fragment}
error: `ints` does not live long enough
        (&ints[0], &ints[ints.len() - 1])
          ^~~~
note: reference must be valid for the lifetime 'a ...
note: ...but borrowed value is only valid for the scope of
note:    parameters for function
```

## Lifetime Elision 1 { data-transition="fade-out" }

```rust
#[test]
fn lifetime_elision_1() {
    fn print1<'a>(ints: &'a Vec<i32>) {
        println!("v_1: {}", ints[1]);
    }
    fn print2<'a, 'b>(ptrs: &'a Vec<&'b i32>) {
        println!("v_1: {}", ptrs[1]);

    }
    fn print3<'a, 'b>(ptrs: &'a mut Vec<&'b i32>, ptr: &'b i32) {
        println!("v_1: {}", ptrs[1]);
        ptrs.push(ptr);
    }
}
```

## Lifetime Elision 2 { data-transition="fade" }

```rust
#[test]
fn lifetime_elision_2() {
    fn print1    (ints: &   Vec<i32>) {
        println!("v_1: {}", ints[1]);
    }
    fn print2        (ptrs: &   Vec<&   i32>) {
        println!("v_1: {}", ptrs[1]);

    }
    fn print3<    'b>(ptrs: &   mut Vec<&'b i32>, ptr: &'b i32) {
        println!("v_1: {}", ptrs[1]);
        ptrs.push(ptr);
    }
}
```

## Lifetime Elision 3 { data-transition="fade-in" }

```rust
#[test]
fn lifetime_elision_3() {
    fn print1(ints: &Vec<i32>) {
        println!("v_1: {}", ints[1]);
    }
    fn print2(ptrs: &Vec<&i32>) {
        println!("v_1: {}", ptrs[1]);

    }
    fn print3<'b>(ptrs: &mut Vec<&'b i32>, ptr: &'b i32) {
        println!("v_1: {}", ptrs[1]);
        ptrs.push(ptr);
    }
}
```

## Generic items

## Generic items 1 { data-transition="fade-out" }

```rust
#[test]
fn generic_items_1() {
    fn push_twice<'b>(ptrs: &mut Vec<&'b i32>, ptr: &'b i32) {
        ptrs.push(ptr);
        ptrs.push(ptr);
    }
    let (one, two, three, four) = (1, 2, 3, 4);
    let mut v = vec![&one, &two, &three];
    push_twice(&mut v, &four);
}
```

This obviously generalizes beyond `i32`!

## Generic items 2 { data-transition="fade-in" }

```rust
#[test]
fn generic_items_2() {
    fn push_twice<'b, T>(ptrs: &mut Vec<&'b T>, ptr: &'b T) {
        ptrs.push(ptr);
        ptrs.push(ptr);
    }
    let (one, two, three, four) = (1, 2, 3, 4);
    let mut v = vec![&one, &two, &three];
    push_twice(&mut v, &four);
}
```

This is going so smoothly; lets try printing `v_1` again!

## Generic items 3

``` { .rust .compile_error }
#[test]
fn generic_items_3() {
    fn push_twice<'b, T>(ptrs: &mut Vec<&'b T>, ptr: &'b T) {
        println!("v_1: {}", ptrs[1]);
        ptrs.push(ptr);
        ptrs.push(ptr);
    }
    let (one, two, three, four) = (1, 2, 3, 4);
    let mut v = vec![&one, &two, &three];
    push_twice(&mut v, &four);
}
```

```{.fragment}
error: trait `core::fmt::Display` not implemented for the type `T`
        println!("v_1: {}", ptrs[1]);
                            ^~~~~~~
```

(Reminder: Rust is not C++)

## Trait-bounded polymorphism

```rust
trait Dimensioned {
    fn height(&self) -> u32;
    fn width(&self) -> u32;
}

fn stacked_height<S>(v: &[S]) -> u32 where S: Dimensioned {
    let mut accum = 0;
    for s in v { accum += s.height() }
    accum
}
```

## Trait Impls

```rust
struct Rect { w: u32, h: u32 }
struct Circle { r: u32 }

impl Dimensioned for Rect {
    fn height(&self) -> u32 { self.h }
    fn width(&self) -> u32 { self.w }
}

impl Dimensioned for Circle {
    fn height(&self) -> u32 { self.r * 2 }
    fn width(&self) -> u32 { self.r * 2 }
}
```

## Traits in Action

```rust
impl Rect {
    fn square(l: u32) -> Rect { Rect { w: l, h: l } }
}
impl Circle {
    fn with_radius(r: u32) -> Circle { Circle { r: r } }
}

#[test]
fn trait_bounded_polymorphism() {
    let squares = [ Rect::square(1), Rect::square(2) ];
    let circles = [ Circle::with_radius(1), Circle::with_radius(2)];
    assert_eq!(stacked_height(&squares), 3);
    assert_eq!(stacked_height(&circles), 6);
}
```

## Generics do not suffice

``` {.rust .compile_error}
#[test]
fn parametric_fail() {
    let shapes = [Rect::square(1), Circle::with_radius(2)];
    assert_eq!(stacked_height(&shapes), 5);
}
```

``` {.fragment}
error: mismatched types:
 expected `Rect`,
    found `Circle`
    let shapes = [Rect::square(1), Circle::with_radius(2)];
                                   ^~~~~~~~~~~~~~~~~~~~~~
```

## Monomorphization is why
``` {.rust .compile_error}
struct Rect { w: u32, h: u32 }
struct Circle { r: u32 }

fn parametric_fail() {
    let shapes = [Rect::square(1), Circle::with_radius(2)];
    //  ~~~~~~    ~~~~~~~~~~~~~~~  ~~~~~~~~~~~~~~~~~~~~~~
    //    |              |                    |
    //    |       This is 8 bytes     This is 4-bytes
    //    |
    //  There's no uniform array
    //  type to hold both in-line.
}
```

## This is a job for ...

### Object-Oriented Programming! {.fragment}

## Traits as Objects 1

```rust
fn stacked_obj_refs(v: &[&Dimensioned]) -> u32 {
    let mut accum = 0;
    for s in v { accum += s.height() }
    accum
}

#[test]
fn demo_objs_1() {
    let r = Rect::square(1);
    let c = Circle::with_radius(2);
    let shapes: [&Dimensioned; 2] = [&r, &c];
    assert_eq!(stacked_obj_refs(&shapes), 5);
}
```

## Traits as Objects 2

```rust
fn stacked_obj_boxes(v: &[Box<Dimensioned>]) -> u32 {
    let mut accum = 0;
    for s in v { accum += s.height() }
    accum
}

#[test]
fn demo_objs_2() {
    let shapes: [Box<Dimensioned>; 2] =
        [Box::new(Rect::square(1)), Box::new(Circle::with_radius(2))];
    assert_eq!(stacked_obj_boxes(&shapes), 5);
}
```
