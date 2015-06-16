# Ownership and Move Semantics

## Owned vs Copied
  * Consider assignment
    ``` {.rust}
    left_side = right_side;
    ```

  * Owned data
    * *moves* `right_side` into `left_side`
    * `right_side` becomes inaccessible
    * one can still opt into explicit duplication,
      e.g. '`new = owned.clone();`{.rust}'

  * Copied data
    * *memcpy's* `right_side` into `left_side`
    * one can freely keep using the original

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
