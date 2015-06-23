----

## Section 2: Borrowing (Immutably)

All of the "Borrowing (Immutably)" exercises involve manipulating the code at
the following playpen link:

  * [`https://play.rust-lang.org/?gist=193a107a5e6702d457db&version=stable`][Borrowing Playpen]

[Borrowing Playpen]: https://play.rust-lang.org/?gist=193a107a5e6702d457db&version=stable

### Core Exercises

#### Exercise 1

The distinction between a statement versus a true
expression is sometimes confusing, leading to bugs such as that
exhibited by the code in [Borrowing Playpen]. Identify and fix the
bug.

Note: You will need to complete this exercise to do the others
in this section.

Hint 1. If you are not sure what is "wrong" about the code, the
fact that the lines marked with `(***)` found it necessary to use
`{:?}` to format one supposed sum and `{}` to format another is a
clue; look carefully at the generated output in those two places.

Hint 2. It may be useful to add type annotations to isolate where
"the bad value" is being generated.

Hint 3. Ask your neighbor, or one of the helpers!

#### Exercise 2

Rename the `fn main()` in the [Borrowing Playpen] to
`fn any_name_but_main()`, and hit the Run button again.

What changed?

#### Exercise 3a

Is it perhaps silly to write `&vec1` on the line marked `(**)`
in [Borrowing Playpen] ?

After all, `let borrowed: &Vec<i32> = &vec1;` already binds the result
of that expression, right?

What will happen if you *just* replace the `&vec1` below the (**) with
`borrowed`, as in:

``` {.rust}
let also_a_sum = sum(borrowed);
```

Try it and see. What is the problem with this change alone?


#### Exercise 3b

Re-attempt to make the change to `(**)` in [Borrowing Playpen] to read:

``` {.rust}
let also_a_sum = sum(borrowed);
```

But this time, address the compiler's complaints in some manner by
making further changes elsewhere.

### Review Exercise

#### Exercise 4

Write a function, `fn choose_vec`, that takes two input
`Vec<i32>` parameters (as moved arguments), and returns one of
them, i.e. it has signature:

``` {.rust}
fn choose_vec(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> { ... }
```

The function should choose whichever vector that has a larger
element at index 0.

Include some unit tests for your `choose_vec`.

### Extra Exercises

#### Exercise 5

Write a function, `fn palindrome`, that takes a
borrowed `&Vec<i32>` and returns a boolean. It returns true if and
only if the series of values in-order is equal to the reversed
series. E.g. the series `[2, 17, 4, 17, 2]` is a palindrome, while
`[2, 17, 17]` is not.

#### Exercise 6

It is not idiomatic in Rust to define a function that
takes an immutably-borrowed `&Vec<T>` argument. Instead, one uses a
borrowed slice `&[T]`, which is more general.

We will be seeing more with slices in the future; for now, just try
changing the signature of `fn sum` so that it takes a `&[i32]`
instead of `&Vec<i32>`, and see if you can modify the resulting
code to compile.

(Make sure you try to compile it first after making the change; the
compiler often provides useful hints as to what needs changing.)
