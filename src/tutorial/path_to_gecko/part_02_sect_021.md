## using pub items

```rust
mod d {
    pub type I = i32;
	pub fn add3(x: I) -> I { add3priv(x) }
	fn add3priv(x: I) -> I { x + 3 }
}

mod e {
	use super::d::add3;
	#[test] fn t() { assert_eq!(add3(1), 4); }
}
```
