## A really big idea

# Ownership and Move Semantics

## Creation and Consumption

Once initialized, local owns its data (vector's backing store)
```rust
#[test]
pub fn create_owned() {
    let mut vec = Vec::new();         //  + (`vec` initialized)
    vec.push(2000);                   //  |   ... and
    vec.push( 400);                   //  |        also
    vec.push(  60);                   //  |         modified ...
    println!("vec: {:?}", vec);       //  |
    let the_sum = sum(vec);           // (... and moved)
    println!("the_sum: {}", the_sum); 
}
```

At scope end, initialized variables are cleaned up
```rust
fn sum(v: Vec<i32>) -> i32 {          //  +
   let mut accum = 0;                 //  |
   for i in v.iter() { accum += *i; } //  |
   accum // (p.s. where is `return`?) //  |
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

# Exercises

## Exercises 1

[http://pnkfelix.github.io/cyot/tutorial/exercises/ex_part_1.html][ex_part_1]

[ex_part_1]: http://pnkfelix.github.io/cyot/tutorial/exercises/ex_part_1.html
