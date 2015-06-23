# Ownership and Move Semantics

## Creation and Consumption

Create (and modify) owned:
```rust
#[test]
pub fn create_owned() {
    let mut vec = Vec::new();         //  + (`vec` initialized)
    vec.push(2000);                   //  |
    vec.push( 400);                   // 'vec
    vec.push(  60);                   //  |
    println!("vec: {:?}", vec);       //  |
    let the_sum = sum(vec);           // (moved)
    println!("the_sum: {}", the_sum); 
}
```

Consume owned:
```rust
fn sum(v: Vec<i32>) -> i32 {          //  +
   let mut accum = 0;                 //  |
   for i in v { accum += i; }         // 'v
   accum                              //  |
}                                     //  + (`v` destroyed/freed)
```

## Move vs Copy

``` {.rust .compile_error}
#[test]
fn demo_owned_vs_copied() {
    let moving_value = vec![1, 2, 3];
    let copied_value = 17;
    let tuple = (moving_value, copied_value);

    println!("copied_value: {:?}", copied_value);
    println!("moving_value: {:?}", moving_value);
}
```

``` {.compile_error .fragment}
error: use of moved value: `moving_value` [E0382]
    println!("moving_value: {:?}", moving_value);
                                   ^~~~~~~~~~~~

note: `moving_value` moved here because it has type
      `collections::vec::Vec<i32>`, which is non-copyable
    let tuple = (moving_value, copied_value);
                 ^~~~~~~~~~~~
```

<!--
----

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

-->
