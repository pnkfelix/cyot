% Path to Gecko
% Felix Klock
% Thursday 25 June

# Rust: How do we get to Gecko?

## Goals for today

  * Convince you Rust is awesome
  * Provide seeds of knowledge
    * One can only cultivate so much during a three hour window

## Goals for Rust

  * Safe. Concurrent. Fast.

  * Specifics
    * Memory safety without garbage collection
    * Concurrency without data races
    * Abstraction without overhead

  * Generalization: HACK WITHOUT FEAR!

## Let us see some code

  * These are *"amuse bouches"*
    * not the main course
    * (not even an appetizer)

  * Do not worry if it goes too fast
  
## High-level, but fast

The below [loop demo] compiles down to tight code:

<!--
```rust
#[allow(dead_code)]
fn main() {
    let v1: Vec<i32> = (-100..10).collect();
    let s1 = sum_pos(&v1);
    let v2: Vec<i32> = (-100..1000).collect();
    let s2 = sum_pos(&v2);
    println!("v1.len: {} s1: {} v2.len: {} s2: {}", v1.len(), s1, v2.len(), s2);
}

#[allow(dead_code)]
#[inline(never)]
```
-->

```rust
// sums all the positive values in `v`
fn sum_pos(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in v.iter().filter(|i| **i > 0) {
        sum += *i;
    }
    sum
}
```

[loop demo]: https://play.rust-lang.org/?gist=23a69161dd4421e2925f

----

Generated x86_64 machine code for `fn sum_pos`{.rust}:

```nasm
	leaq	(%rdi,%rsi,4), %rcx
	xorl	%eax, %eax
	jmp	.LBB5_1
.LBB5_3:
	addl	%edx, %eax
	.align	16, 0x90
.LBB5_1:
	cmpq	%rdi, %rcx
	je	.LBB5_4
	movl	(%rdi), %edx
	addq	$4, %rdi
	testl	%edx, %edx
	jle	.LBB5_1
	jmp	.LBB5_3
.LBB5_4:
	retq
```

(when compiled in "release mode")

## Memory safety

Example: catches iterator invalidation bugs

``` {.rust}
fn this_wont_compile(v: &mut Vec<i32>) -> i32 {
    let mut sum = 0;
    for &i in v.iter() {
        if i > 0 { v.push(0); } sum += i;
    }
    sum
}
```

``` {.fragment}
error: cannot borrow `*v` as mutable because it is also borrowed
       as immutable
        if i > 0 { v.push(0); } sum += i;
                   ^
note: previous borrow of `*v` occurs here; the immutable borrow
      prevents subsequent moves or mutable borrows of `*v` until
      the borrow ends
    for &i in v.iter() {
              ^
```

## Concurrency

See also [Fearless Concurrency] blog post.

[Fearless Concurrency]: http://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html

```rust
use std::thread;
fn par_merge_sort<T: Ord + Send>(x: &mut [T]) {
    if x.len() <= 1 { return }
    let half = x.len() / 2;
    let (l, r) = x.split_at_mut(half);
    let g = thread::scoped(|| { par_merge_sort(r);
                                r });
    par_merge_sort(l);
    let r = g.join();
    merge(l, r);
}
```

(caveat: above is using unstable `thread::scoped` API)

<!--
```rust
// left[0] <= left[1] <= ... <= left[last] <= right[0] <= ...
use std::mem;
#[allow(dead_code)]
fn merge<T: Ord>(left: &mut [T], right: &mut [T]) {

    let mut i = 0;
    loop {
        // println!("i: {} left: {:?} right: {:?}", i, left, right);
        if i >= left.len() || 0 >= right.len() { break; }
        if left[i] > right[0] {
            mem::swap(&mut left[i], &mut right[0]);
            let mut j = 0;
            while j+1 < right.len() && right[j] > right[j+1] {
                let (pre, post) = right.split_at_mut(j+1);
                mem::swap(&mut pre[j], &mut post[0]);
                j = j+1;
            }
        }
        i += 1;
    }
}

#[test]
fn hi() {
    let mut v = vec![2, 1, 10, 9, 8, 7, 6, 5, 4, 3];
    par_merge_sort(&mut v);
    assert_eq!(v, [1,2,3,4,5,6,7,8,9,10]);
}
```
-->

# Lets dive in

## Outline for Tutorial

* Goals
* Ownership and Borrowing; Arrays and Strings
* Local Development: Cargo; Crates and Modules
* More Fundamentals: Data; More on Borrowing; Traits
* Systems Development: Concurrency and I/O; FFI
