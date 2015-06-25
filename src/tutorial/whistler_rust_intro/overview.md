% Rust Intro
% Felix and the Rust Team
% 25 June 2015; [http://bit.ly/1N93I0P]

# Rust: What? Why? (How?)

## Goals for today

  * Convince you Rust is awesome
  * Provide seeds of knowledge
    * One can only cultivate so much during a three hour window

## These slides

[http://bit.ly/1N93I0P]

~

[http://pnkfelix.github.io/cyot/tutorial/slides/whistler_rust_intro.html]

[http://bit.ly/1N93I0P]: http://bit.ly/1N93I0P
[http://pnkfelix.github.io/cyot/tutorial/slides/whistler_rust_intro.html]: http://pnkfelix.github.io/cyot/tutorial/slides/whistler_rust_intro.html

## Goals for Rust

  * Safe. Concurrent. Fast.

  * Specifics
    * Abstraction without overhead
    * Memory safety without garbage collection
    * Concurrency without data races

  * Generalization: HACK WITHOUT FEAR!

## A taste

  * Three fast *"amuse bouches"*
    * not the main course
    * (not even an appetizer)

## Abstraction without overhead { data-transition="fade-out" }

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

[loop demo]: http://is.gd/weGnJ0
<!-- https://play.rust-lang.org/?gist=23a69161dd4421e2925f -->

## Abstraction without overhead { data-transition="fade-in" }

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

``` {.rust .compile_error}
fn this_wont_compile(v: &mut Vec<i32>) -> i32 {
    let mut sum = 0;
    for &i in v.iter() {
        sum += i;
        if i > 0 { v.push(0); }
        //         ~~~~~~~~~ invalid! (might realloc
        //                   the backing storage for `v`)
    }
    sum
}
```

``` {.fragment}
error: cannot borrow `*v` as mutable because it is also borrowed
       as immutable
        if i > 0 { v.push(0); }
                   ^
note: previous borrow of `*v` occurs here; the immutable borrow
      prevents subsequent moves or mutable borrows of `*v` until
      the borrow ends
    for &i in v.iter() {
              ^
```

## Slick, Fearless Concurrency

See also [Fearless Concurrency] blog post.

[Fearless Concurrency]: http://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html

<!--
```rust
fn seq_max(partial_data: &[u8]) -> u8 {
    *partial_data.iter().max().unwrap()
}
```
-->

```rust
use std::thread;
fn par_max(data: &[u8]) -> u8 {
    if data.len() <= 4 { return seq_max(data); }
    let len_4 = data.len() / 4; // DATA = [A .., B .., C .., D..]
    let (q1, rest) = data.split_at(len_4);    // (A.. \ B..C..D..)
    let (q2, rest) = rest.split_at(len_4);    //  (B.. \ C..D..)
    let (q3, q4)   = rest.split_at(len_4);    //   (C.. \ D..)
    let t1 = thread::scoped(|| seq_max(q1));  // fork A..
    let t2 = thread::scoped(|| seq_max(q2));  // fork B..
    let t3 = thread::scoped(|| seq_max(q3));  // fork C..
    let v4 = seq_max(q4);                     // compute D..
    let (v1, v2, v3) = (t1.join(), t2.join(), t3.join()); // join!
    return seq_max(&[v1, v2, v3, v4]);
}
```

(caveat: above is using unstable `thread::scoped` API)

<!--
```rust
#[test]
fn hi() {
    let mut v = vec![2, 1, 10, 9, 8, 7, 6, 5, 4, 3];
    let m = par_max(&v);
    assert_eq!(m, 10);
}
```
-->
