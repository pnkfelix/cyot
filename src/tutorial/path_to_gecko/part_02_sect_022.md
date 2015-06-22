## privacy { data-transition="fade-in" }

```rust
mod d {
    pub type I = i32;
	pub fn add3(x: I) -> I { add3priv(x) }
	fn add3priv(x: I) -> I { x + 3 }
}
```

``` {.rust .compile_error}
mod f {
	use super::d::add3priv;
	#[test] fn t() { assert_eq!(add3priv(1), 4); }
}
```

```shell-session
error: function `add3priv` is private
	use super::d::add3priv;
	    ^~~~~~~~~~~~~~~~~~
```
