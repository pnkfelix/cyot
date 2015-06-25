## Section 1: Move Semantics

All of the "Move Semantics" exercises involve manipulating the code at
the following playpen link:

  * [`https://play.rust-lang.org/?gist=b30f72fe3eb559c83994&version=stable`][Move Semantics Playpen]

[Move Semantics Playpen]: https://play.rust-lang.org/?gist=b30f72fe3eb559c83994&version=stable

### Core Exercises

#### Exercise 1.1

Remove the `mut` from the binding of `vec1` on the line marked `(*)` in
the [Move Semantics Playpen]. Why does the code stop compiling?

(Put the `mut` back afterward.)


#### Exercise 1.2

Try to change the first `println!` call in the
[Move Semantics Playpen] so that it reports the length of `vec0` after
`fill_vec` returns.

(One quick way to do this is to just replace
`vec1` with `vec0` in the `println!` call.)

Why doesn't the code compile successfully anymore?

### Extra Exercises

(You can skip these exercises and come back to them after you have done
the other core exercises, if you like.)

#### Exercise 1.3

On the line marked `(**)` in the [Move Semantics Playpen], the whole
line (that is, the rebinding of `vec` to `mut vec`) can be removed if
we just add the keyword `mut` to another spot in `fn fill_vec`. Figure
out how to remove the line marked `(**)`.

(Hint: in general, the use of `mut` is not attached to `let` --
rather, such a `mut` is attached to *bindings*)

#### Exercise 1.4

Refactor the code in the [Move Semantics Playpen] so that instead of
creating the vector in `fn main`, we instead create it within `fn
fill_vec`, and transfer the freshly created vector from `fill_vec` to its
caller. Note this will require revising the function signature for
`fill_vec`.
