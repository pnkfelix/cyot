# Ownership and Move Semantics

## Creating an owned value

```rust
pub fn main() {
    let mut vec = Vec::new();
    vec.push(2000);
    vec.push( 400);
    vec.push(  60);
    println!("vec: {:?}", vec);
    let the_sum = sum(vec);
    println!("the_sum: {}", the_sum);
}
```

## Consuming an owned value

```rust
fn sum(v: Vec<i32>) -> i32 {
   let mut accum = 0;
   for i in v { accum += i; }
   accum
}
```
