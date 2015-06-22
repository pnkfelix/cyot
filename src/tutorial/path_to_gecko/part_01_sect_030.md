## Immutable versus Mutable Borrows

## imm-borrows: can copy freely

Implications:

  * must assume aliased
  * therefore *not safe* to mutate

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

## `&mut` borrows


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

## Data has at most one `&mut` borrow { data-transition="slide-in fade-out" }

``` { .rust .compile_error }
#[test]
fn demo_cannot_mut_borrow_multiple_times() {
    let mut v1 = vec![1, 2, 3];
    let mut v2 = vec![1, 2, 3];
    take2(&mut v1, &mut v2); // <-- this is okay
    take2(&mut v1, &mut v1);
}
```

```rust
fn take2<'a>(v1: &'a mut Vec<i32>, v2: &'a Vec<i32>) { }
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

## Cannot alias `&mut`-borrowed data { data-transition="fade" }

``` {.rust .compile_error }
#[test]
fn demo_cannot_alias_mut_borrowed_data() {
    let mut v1 = vec![1, 2, 3];
    let mut v2 = vec![1, 2, 3];
    take2(&mut v1, &v2); // <-- this is okay
    take2(&mut v1, &v1);
}
```

```{.rust}
fn take2<'a>(v1: &'a mut Vec<i32>, v2: &'a Vec<i32>) { }
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

## `&mut T` is non-copy { data-transition="fade-in slide-out" }

``` {.rust .compile_error }
#[test]
fn demo_cannot_copy_mut_borrows() {
    let mut v1 = vec![1, 2, 3];
    let b = &mut v1;
    let c = b;
    take2(b, c);
}
```

```{.rust}
fn take2<'a>(v1: &'a mut Vec<i32>, v2: &'a Vec<i32>) { }
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

``` {.rust .compile_error}
#[test]
fn demo_exclusive_access_versus_ownership() {
    let mut v1 = vec![1, 2, 3];
    let mut v2 = vec![7, 8, 9];
    take_by_value(v1);
    take_mut_borrow(&mut v2);
    println!("v1: {:?} v2: {:?}", v1, v2);
}

fn take_by_value(v: Vec<i32>) { let mut v = v; v.push(4);  }
fn take_mut_borrow(b: &mut Vec<i32>) { b.push(10); }
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
