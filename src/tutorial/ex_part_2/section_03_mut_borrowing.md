----

## Section 3: Mutably Borrowing

All of the "Mutably Borrowing" exercises involve manipulating the code at
the following playpen link:

  * [`https://play.rust-lang.org/?gist=a2b72a81c8e366e117e0&version=stable`][Mutably Borrowing Playpen]

[Mutably Borrowing Playpen]: https://play.rust-lang.org/?gist=a2b72a81c8e366e117e0&version=stable

### Core Exercises

#### Exercise 3.1

Continuing the theme from a previous section: try
lifting the `let borrowed = &vec1;` out of its expression block
in [Mutably Borrowing Playpen], so
that the `let borrowed` binding is on the same level as the other
statements in `fn main`.

What goes wrong?

#### Exercise 3.2

Can we simplify the code by replacing the four lines
below `(*)` in [Mutably Borrowing Playpen] with the single:

``` {.rust}
let the_sum = sum(&vec1);
```

Why does this work? How does this differ from exercise 1?


#### Exercise 3.3

Write a function

``` {.rust}
fn zero_fill(v: &mut Vec<usize>, count: usize)
```

that pushes `count` zeroes onto the end of `v`.

#### Exercise 3.4

Is the `usize` annotation on the line labelled `(***)` in
[Mutably Borrowing Playpen] necessary? Try removing it and see;
the code should still compile.

So, what type is being assigned to `i` when the annotation is missing?

Can you do an experiment ot prove that `i` is *not* being assigned
the type `i32` when the annotation is missing?

What types do you think are being assigned to `c` and `sum`?


### EXTRA EXERCISES

HINT: You may need to use methods we have not yet seen to do
these exercises; the `Vec` API is visible at:

  [https://doc.rust-lang.org/stable/std/vec/struct.Vec.html]
  
[https://doc.rust-lang.org/stable/std/vec/struct.Vec.html]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html

#### Exercise 3.5

Write a function that takes a `&mut Vec<i32>` and
imperatively replaces its contents with its prefix-sum:

`[v1, v2, v3, ...]` is replaced with `[v1, v1+v2, v1+v2+v3, ...]`.

Examples:

 * `[1, 0, 1, 0]` is replaced with `[1, 1, 2, 2]`

 * `[1, 2, 3, 4]` is replaced with `[1, 3, 6, 10]`


#### Exercise 3.6

Write a function that takes a `&mut Vec<u32>` and
imperatively removes all of its zero entries, effectively
filtering it so that it contains only positive values.

Examples:

  * `[1, 0, 1, 0]` is replaced with `[1, 1]`

  * `[1, 2, 3, 4]` is replaced with `[1, 2, 3, 4]`.
