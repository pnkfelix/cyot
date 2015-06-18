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

## Lifetime bindings.

## Borrowed return values.

## Generic items

## Trait-bounded polymorphism

## Traits as Objects
