# Part 5: Systems Development

## IO in Rust

Many input/output (IO) routines can encounter errors

<!--  (`panic!` from std lib not suitable for external errors) -->

1.0 philosophy: `Result<T, Error>`; callers handle

```
impl File { fn open<P:AsRef<Path>>(path: P) -> io::Result<File> }
impl<B: BufRead> Iterator for Lines<B> {       ~~~~~~~~~~
    type Item = io::Result<String>
}               ~~~~~~~~~~
```

```rust
#[test]
fn demo_io() {
    use std::fs::File;
    use std::io::{self, BufReader, BufRead};
    fn do_io() -> Result<(), io::Error> {
        let f = try!(File::open("/etc/ssh_config"));
        let buffered = BufReader::new(f);
        for l in buffered.lines().take(10) {
            println!("line: {}", try!(l));
        }
        Ok(()) // (need to remember the result!)
    }
    do_io().unwrap()
}
```

<!--
``` {.rust .heads_up .fragment }
try!(EXPR) rewrites to:
(match EXPR { Result::Ok(val) => val,
    Result::Err(err) => { return Result::Err(From::from(err)) } })
```
-->

<!--
``` {.rust .fragment}
macro_rules! try {
    ($expr:expr) => (match $expr {
        $crate::result::Result::Ok(val) => val,
        $crate::result::Result::Err(err) => {
            return $crate::result::Result::Err($crate::convert::From::from(err))
        }
    })
}
```
-->

<!--
FIXME: This slide could perhaps be at end of part 3.
(and presumably we could then move the Closures material here;
it just needs to precede discussion of thread APIs.)
-->

<!--

Anyway, the reason this slide was over here was that it needs to
precede our discussion of channels since those follow the I/O api
patterns of using `Result` and `try!`.

Pretty soon I'll draw up that dependency graph, using these slides as
a starting point.  :)

-->

## Concurrency

Rust's killer feature:

### Data-race freedom

built atop same foundation as memory safety

## Here's what one concurrency API looks like

## `thread::spawn`

```rust
pub fn main() {
    use std::thread;
    let al = "long lost pal";
    thread::spawn(move || {

        println!("i can be your {}", al);
    });

    println!("why am i soft in the middle");
    // Note: might exit before spawned thread gets chance to print
}
```

## No data races: What about our precious mutation?

## No data races 1: "direct" assign { data-transition="fade-out" }

``` {.rust}
#[test] fn demo_catch_direct() {
    fn fib(x: i64) -> (i64, i64) {
        if x <= 1 { (x,1) } else { (x, fib(x-1).1 + fib(x-2).1) }
    }
    use std::thread;
    let al = "al";
    let mut f_10_recv = (0, 0);

    thread::spawn(move || {
        f_10_recv = fib(10);
        println!("you can call me {}", al);
    });
    let f_15 = fib(15).1;
    while f_10_recv.0 == 0 { }  // <-- many alarm bells
    let f_10 = f_10_recv.1;
    println!("why am i short of attention");
    assert_eq!((f_10, f_15), (89, 987));
}
```

#### compiles; does not work (no actual communication; implicit copying) {.fragment }

## No data races 2: mut-ref  { data-transition="fade-in" }

``` {.rust .compile_error}
#[test] fn demo_catch_mutref() {
    fn fib(x: i64) -> (i64, i64) {
        if x <= 1 { (x,1) } else { (x, fib(x-1).1 + fib(x-2).1) }
    }
    use std::thread;
    let al = "al";
    let mut f_10_recv = (0, 0);
    let ptr_recv = &mut f_10_recv; // <-- Okay, say what we meant
    thread::spawn(move || {
        *ptr_recv = fib(10);
        println!("you can call me {}", al);
    });
    let f_15 = fib(15).1;
    while f_10_recv.0 == 0 { }  // <-- many alarm bells
    let f_10 = f_10_recv.1;
    println!("why am i short of attention");
    assert_eq!((f_10, f_15), (89, 987));
}
```

#### does not compile: `spawn` can't share ref to stack-local {.fragment}

## channels for message passing

```rust
#[test] fn demo_channel() {
    fn fib(x: i64) -> (i64, i64) {
        if x <= 1 { (x,1) } else { (x, fib(x-1).1 + fib(x-2).1) }
    }
    use std::thread;
    use std::sync::mpsc::channel;
    let (tx, rx) = channel();
    let al = "al";
    thread::spawn(move || {
        tx.send(fib(10));
        println!("you can call me {}", al);
    });
    let f_15 = fib(15).1;
    println!("why am i short of attention");
    let f_10 = rx.recv().unwrap().1;
    assert_eq!((f_10, f_15), (89, 987));
}
```

#### channels are abstraction, data-race free {.fragment}

## Here's a totally different concurrency API

## `thread::scoped`

```rust
fn seq_max(partial_data: &[u8]) -> u8 {
    *partial_data.iter().max().unwrap()
}

fn par_max(data: &[u8]) -> u8 {
    if data.len() <= 4 { return seq_max(data); }
    let len_4 = data.len() / 4; // DATA = [A..B..C..D..]
    let (q1, rest) = data.split_at(len_4); // (A.. \ B..C..D..)
    let (q2, rest) = rest.split_at(len_4); //  (B.. \ C..D..)
    let (q3, q4)   = rest.split_at(len_4); //   (C.. \ D..)
    let t1 = ::std::thread::scoped(|| seq_max(q1)); // fork A..
    let t2 = ::std::thread::scoped(|| seq_max(q2)); // fork B..
    let t3 = ::std::thread::scoped(|| seq_max(q3)); // fork C..
    let v4 = seq_max(q4); //                                D..
    let (v1, v2, v3) = (t1.join(), t2.join(), t3.join());
    return seq_max(&[v1, v2, v3, v4]);
}
```

## `thread::scoped` shows a new trick

  * `thread::spawn` disallowed passing refs to stack-local data

  * Allowing that is the whole point of `thread::scoped`

    * (caveat: `thread::scoped` API is unstable, and undergoing revision due
      to subtle soundness issue)

## Benchmarking `par_max` 1

```rust
extern crate test; use std::iter;
const LIL: usize = 20 * 1024;
const BIG: usize = LIL * 1024;

fn make_data(count: usize) -> Vec<u8> {
    let mut data: Vec<u8> = iter::repeat(10).take(count).collect();
    data.push(200); data.push(3); return data;
}

#[bench]
fn bench_big_seq(b: &mut test::Bencher) {
    let data = make_data(BIG);
    b.iter(|| assert_eq!(seq_max(&data), 200));
}
#[bench]
fn bench_big_par(b: &mut test::Bencher) {
    let data = make_data(BIG);
    b.iter(|| assert_eq!(par_max(&data), 200));
}
```

```
bench_big_par ... bench:   3,763,711 ns/iter (+/- 1,140,321)
bench_big_seq ... bench:  21,633,799 ns/iter (+/- 2,522,262)
```

## Benchmarking `par_max` 2

```{.rust}
const LIL: usize = 20 * 1024;
const BIG: usize = LIL * 1024;
```

```
bench_big_par ... bench:   3,763,711 ns/iter (+/- 1,140,321)
bench_big_seq ... bench:  21,633,799 ns/iter (+/- 2,522,262)
```

```rust
#[bench]
fn bench_lil_seq(b: &mut test::Bencher) {
    let data = make_data(LIL);
    b.iter(|| assert_eq!(seq_max(&data), 200));
}
#[bench]
fn bench_lil_par(b: &mut test::Bencher) {
    let data = make_data(LIL);
    b.iter(|| assert_eq!(par_max(&data), 200));
}
```

```
bench_lil_par ... bench:      59,274 ns/iter (+/- 7,756)
bench_lil_seq ... bench:      15,432 ns/iter (+/- 1,961)
```

(`fn par_max` could tune threshold for seq. path)

## What was that about preventing data races?

## `Send`, `Sync`

  * If `T: Send`, then passing (e.g. moving) a `T` to another thread is safe.

  * If `T: Sync`, then copying a `&T` to another thread is safe.

  * (For Rust, "safe" includes "no data races exposed.")

<!-- FIXME: elaborate, add e.g. counter-examples
Or maybe just drop this slide entirely.
-->

## unsafe code

Rust's safety checking is incomplete

Bypass it (and take responsibility for safety)
via `unsafe`{.rust}

```rust
#[should_panic]
#[test]
fn demo_out_of_bounds_access() {
    let cap = {
        let v0 = Vec::from("Goodbye World!");
        v0.capacity()
    }; // <--- `v0` is freed here
    let mut new_v: Vec<u8> = Vec::with_capacity(cap);
    unsafe { new_v.set_len(cap); }
    println!("v[0..4]: {:?} b'Good': {:?}", &new_v[0..4], b"Good");
    panic!("unsafe demo");
}
```

On my machine, prints:
```
v[0..4]: [71, 111, 111, 100] b'Good': [71, 111, 111, 100]
```

##### ("Yay," we can still have security bugs!) {.fragment}

## native pointers

Type classification, for any type `T`

``` { .rust }
// Safe References
&T
&mut T
```

``` { .rust }
// Unsafe pointers
*mut T
*const T
```

```rust
#[test]
fn demo_unsafe_pointer() {
    let x = [3, 4, 5];
    let p = &x[0] as *const i32;
    unsafe { println!("p[0]: {} p[2]: {}", *p, *p.offset(2)); }
}
```

prints:
``` { .fragment data-frament-index="1" }
p[0]: 3 p[2]: 5
```
## FFI

<!--
     void
     qsort(void *base, size_t nel, size_t width,
         int (*compar)(const void *, const void *));
-->


```rust
use libc::c_int as int; use libc::c_void as void; use libc::size_t;
use std::mem::size_of;
extern "C" {
    fn qsort(base: *mut void, nel: size_t, width: size_t,
             compar: *const extern fn (*const void,
                                       *const void) -> int);
}
extern "C" fn compar_i32(x: *const void, y: *const void) -> int {
    unsafe { *(x as *const i32) - *(y as *const i32) }
}
#[test]
fn demo_ffi() {
    let mut data: [i32; 9] = [9, 8, 1, 2, 7, 6, 3, 4, 5];
    unsafe { let ptr = &mut data[0] as *mut i32;
        qsort(ptr as *mut void, 9, 4, compar_i32 as *const _);
    }
    assert_eq!(data, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
```

<!--
TODO: potential exercise: Felix iteratively developed the qsort inputs based
on compiler errors. Maybe explore the space of small modifications to the
code above and see (1.) what errors it yields, and (2.) whether you can come
up with a simpler expression that still compiles and works.
-->

<!--
## Rust in Gecko (demonstration)
-->
