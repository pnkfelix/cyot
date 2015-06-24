# Part 3: Sequence Types and Iteration

## Vectors and Arrays { data-transition="fade-out" }

Array `[T; k]` vs vector `Vec<T>`

```rust
#[test]
fn demo_vec_and_array() {
    // Heap-allocated vector
    let vec: Vec<i32> = vec![1000, 200, 32, 4];
    // Stack-allocated array
    let array: [i32; 4] = [2000, 400, 32, 4];








}
```

## Iteration over ranges { data-transition="fade" }

Range constructor: `start..limit`

```rust
#[test]
fn demo_range() {

    let vec: Vec<i32> = vec![1000, 200, 32, 4];

    let array: [i32; 4] = [2000, 400, 32, 4];

    for i in 0..2 {
        //   ~~~~ range from 0 (inclusive) to 2 (exclusive)
        assert_eq!(vec[i] * 2, array[i]);
    }
    for i in 2..4 {
        assert_eq!(vec[i], array[i]);
    }
}
```

## Vectors and Arrays and Slices { data-transition="fade-in" }

Shared slice of a sequence: `[T]` (cannot be stored on stack)

```rust
#[test]
fn demo_slice() {

    let vec: Vec<i32> = vec![1000, 200, 32, 4];

    let array: [i32; 4] = [2000, 400, 32, 4];



    let slice_1: &[i32] = &vec[2..4];
    let slice_2: &[i32] = &array[2..4];
    assert_eq!(slice_1, slice_2);

    assert_eq!(slice_1[0], slice_1[1] * 8);
}
```


## `for` and iterators { data-transition="fade-out" }

``` {.rust .compile_error}
#[test] fn demo_iters_1() {
    let zvecs = vec![vec![0,0], vec![0,0], vec![0,0], vec![0,0]];

    // Every `for` loop takes an iterator.

    // Some iterators are made by *consuming* input:
    let v = vec![1000, 1001, 1002, 1003];
    for i in vec![0, 1, 2, 3] {
        assert_eq!(v[i], 1000 + i);
    }

    // Some iterators are made by *borrowing* input:
    for elem in &zvecs {
        let elem: Vec<i32> = elem;     // <-- errors here
        assert_eq!(elem, vec![0,0]);   // <--- and here
    }
}
```

## `for` and iterators { data-transition="fade-in" }

```rust
#[test] fn demo_iters_2() {
    let zvecs = vec![vec![0,0], vec![0,0], vec![0,0], vec![0,0]];

    // Every `for` loop takes an iterator.

    // Some iterators are made by *consuming* input:
    let v = vec![1000, 1001, 1002, 1003];
    for i in vec![0, 1, 2, 3] {
        assert_eq!(v[i], 1000 + i);
    }

    // Some iterators are made by *borrowing* input:
    for elem in &zvecs {
        let elem: &Vec<i32> = elem;    // <-- this is (one)
        assert_eq!(elem, &vec![0,0]); // <--- way to fix
    }
}
```

## `String` and `&str`

* `String` and `Vec<T>`: owned, growable

* `str` and `[T]`: fixed-size, cannot be stored on stack

* Both `String` and `str` are UTF-8 (a safety guarantee)

```rust
#[test]
fn demo_string_and_str() {
    let mut hw: String = String::new();
    hw.push_str("Hello");
    hw.push_str(" ");
    hw.push_str("World!");
    assert_eq!(hw, "Hello World!");

    let h: &str = &hw[0..5];
    let w: &str = &hw[6..11];

    assert_eq!(h, "Hello");
    assert_eq!(w, "World");
}
```

## Iterator API { data-transition="fade-out" }

Every iterator inherits many [high-level methods][iter API]

[iter API]: https://doc.rust-lang.org/nightly/std/iter/index.html

```rust
#[test]
fn demo_iter_methods_1() {
    let v1: Vec<&str> = vec!["Hello", "to", "all", "da", "World!"];
    let v2: Vec<&str> = v1.iter()    // borrowing iterator for vec
        .filter(|w| { w.len() > 3 }) // del entries of length <= 3 
        .map(|p| -> &str { *p })     // deref each by one level
        .collect();                  // collect into target vec
    println!(" v1: {:?} \n v2: {:?}", v1, v2);
}
```

#### prints {.fragment data-fragment-index="1" }

``` {.fragment data-fragment-index="1" }
 v1: ["Hello", "to", "all", "da", "World!"]
 v2: ["Hello", "World!"]


```

## Iterator API  { data-transition="fade" }

There is some cool type-based magic

```rust
#[test]
fn demo_iter_methods_2() {
    let v1: Vec<&str> = vec!["Hello", "to", "all", "da", "World!"];
    let s2: String    = v1.iter()    // borrowing iterator for vec
        .filter(|w| { w.len() > 3 }) // del entries of length <= 3 
        .map(|p| -> &str { *p })     // deref each by one level
        .collect();                  // collect into target string
    println!(" v1: {:?} \n s2: {:?}", v1, s2);
}
```

#### prints {.fragment data-fragment-index="1" }

``` {.fragment data-fragment-index="1" }
 v1: ["Hello", "to", "all", "da", "World!"] 
 s2: "HelloWorld!"


```

## Iterator API  { data-transition="fade" }

All magic needs ingredients to work:

``` { .rust .compile_error }
#[test]
fn demo_iter_methods_3() {
    let v1            = vec!["Hello", "to", "all", "da", "World!"];
    let x2            = v1.iter()    // borrowing iterator for vec
        .filter(|w| { w.len() > 3 }) // del entries of length <= 3 
        .map(|p| -> &str { *p })     // deref each by one level
        .collect();                  // collect into target ?????
    println!(" v1: {:?} \n x2: {:?}", v1, x2);
}
```

#### error  {.fragment data-fragment-index="1" }

``` {.fragment data-fragment-index="1" }
error: unable to infer enough type information about `_`; 
       type annotations or generic parameter binding required
    let x2 = v1.iter()
        ^~
```
