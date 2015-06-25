# Exercises

## Exercises 3

[http://pnkfelix.github.io/cyot/tutorial/exercises/ex_part_3.html][ex_part_3]

[ex_part_3]: http://pnkfelix.github.io/cyot/tutorial/exercises/ex_part_3.html

# Part 4: Back to Language Fundamentals

## structs

```rust
#[derive(Copy, Clone)]
struct Point { pub x: i32, pub y: i32 }

fn proj_x_axis(p: &Point) -> Point {
    Point { x: p.x, y: 0 }
}

fn nudge_left(p: &mut Point) { p.x -= 10; }
```

Or add a method:

```rust
impl Point {
    fn proj_x_axis(&self) -> Point {
        Point { x: self.x, y: 0 }
    }
}
```

## tuple-structs, generics

```rust
#[derive(Copy, Clone)]
struct Angle(pub f64);   // construct with e.g. `Angle(90.0)`

struct Pair<X,Y>(X, Y);  // now `Pair(1,2)` or `Pair("a","b")` work
```


<!--

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
-->

<!--
```{.rust}
fn proj_x_axis_check(p: &Point) -> Point {
    Point { y: 0, ..p }
}
```
-->

## enums

```rust
enum Line {
    Segment { start: Point, end: Point },
    Vector(Point, Angle),
}
```

Pattern-matching:
```rust
fn start(l: &Line) -> Point {
    match *l {
        Line::Vector(s, _) => s,
        Line::Segment { start: s, .. } => s,
    }
}
```

## Pattern-matching

``` {.rust}
fn start(l: &Line) -> Point {
    match *l {
        Line::Vector(s, _) => s,
        Line::Segment { start: s, .. } => s,
    }
}
```

Richer than C-style `switch`:
``` {.rust}
    match *l { // (not doing anything meaningful)
        Line::Vector(Point { x: 0, y }, _) => y,
        Line::Segment { start: Point { x, y: 0 }, .. } => x,
        _ => false,
    }
```

## Option and Result

``` {.rust}
enum Option<T> { Some(T), None }

enum Result<T, E> { Ok(T), Err(E) }

// impl<T, E> Result<T, E> { pub fn ok(self) -> Option<T> { ... } }
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


## Lifetime Bindings 1

We saw this kind of thing before:

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

You can bind distinct lifetimes:

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

Encode constraints by reusing same lifetime:

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

Encode constraints by reusing same lifetime:

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

Compiler catches missing necessary constraints:

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
error: cannot infer an appropriate lifetime for automatic coercion
       due to conflicting requirements
        ptrs.push(ptr);
                  ^~~
help: consider using an explicit lifetime parameter as shown:
    fn print<'a, 'b>(ptrs: &'a mut Vec<&'b i32>, ptr: &'b i32)
```

## Borrowed return values 1 { data-transition="fade-out" }

```rust
fn first_n_last<'a>(ints: &'a Vec<i32>) -> (&'a i32, &'a i32) {
    //                                      ~~~~~~~  ~~~~~~~
    (&ints[0], &ints[ints.len() - 1])
}
```

<!--
TODO: Exercise idea: Try to write `fn first_and_last_mut`. Why is it impossible
in general?
-->

. . .

```rust
#[test]
fn demo_borrowed_return_values() {
	let v = vec![1, 2, 3, 4];
	let (first, fourth) = first_n_last(&v);
	assert_eq!(*first, 1);
	assert_eq!(*fourth, 4);
}
```

(compiler ensures borrow `&v`{.rust} lasts long enough to satisfy
 reads of `first` and `fourth`)

## Borrowed return values 2  { data-transition="fade-in" }

``` {.rust .compile_error}
fn first_n_last<'a>(ints: Vec<i32>) -> (&'a i32, &'a i32) {
    //                    ~~~~~~~~ (hint)
    (&ints[0], &ints[ints.len() - 1])
}
```

Why doesn't this work?

``` {.fragment data-fragment-index="1" }
error: `ints` does not live long enough
    (&ints[0], &ints[ints.len() - 1])
      ^~~~
note: reference must be valid for the lifetime 'a ...
note: ...but borrowed value is only valid for the scope of
note:    parameters for function
```

. . .

caller chooses `'a`{.rust}; `fn` body must work for any such choice

(Parameters dropped at scope end; won't live long enough)

## Lifetime Elision

## All the `'a`{.rust}, `'b`{.rust}, ... are ugly

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

## Uniformity of `T` in `Vec<T>` is why

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

## OOP is nice; how about Functional Programming?

## Closures 1

* Can pass functions around as first class entities

* Functions can *close* over externally defined state

Reminder from Javascript:

`closures.js`{.filename}
```javascript
function add3(x) { return x + 3; }

// take function as parameter:
function do_twice(f, y) { return f(f(y)); }

// return function that references outer parameter `z`
function make_adder(z) {
    return function(w) { return w + z; };
}

var add4 = make_adder(4);
var ten = do_twice(add4, 2);
```

## Closures 2

  * In (classic) Javascript, closure syntax is:
    ```javascript
    function (args, ...) { body; ... }
    ```
    where `body` can refer to things from outside.

  * In Rust, the analogous closure expression syntax is:

    ``` {.rust}
    |args, ...| { body; ... }
    ```
    with a few extra details:

. . .

  * opt. `move`{.rust} (forces capture-by-move)

  * opt. arg. and return types (inferred when omitted)

## Closures 3

```rust
#[test]
fn demo_closure() {
    fn add3(x: i32) -> i32 { x + 3 } // <- fn, *not* a closure
    fn do_twice1<F:Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(f(x)) }
    //             ~~~~~~~~~~~~~~ closure type
    fn do_twice2(f: &Fn(i32) -> i32, x: i32) -> i32 { f(f(x)) }

    fn make_adder(y: i32) -> Box<Fn(i32) -> i32> {
        Box::new(move |x| { x + y })
            //   ~~~~~~~~~~~~~~~~~~ closure expression
    }

    let add4 = make_adder(4);
    let six = do_twice1(&add3, 0); let ten = do_twice1(&*add4, 2);
    assert_eq!((six, ten), (6, 10));
    let six = do_twice2(&add3, 0); let ten = do_twice2(&*add4, 2);
    assert_eq!((six, ten), (6, 10));
}
```
