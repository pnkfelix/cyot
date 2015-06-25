## crates.io { .left_align }

. . .

* Cargo's main feature: dependency management

* Before hacking something up, check [crates.io] first

* Adding a third-party crate like `quickcheck` is as simple as
  adding this to the `Cargo.toml`

`Cargo.toml`{.filename}
```INI
[dependencies]
quickcheck = "0.2.20"
```

. . .

Then add this:

`lib.rs`{.filename}
``` {.rust}
extern crate quickcheck;
```

to your `lib.rs`, and you have access to quickcheck!

[crates.io]: http://crates.io/

<!--
## Diamonds

`add3/src/lib.rs (v1.0.0)`{.filename}
``` {.rust}
pub fn add3(x: i32) -> i32 { x + 3 }
```

`add3/src/lib.rs (v2.0.0)`{.filename}
``` {.rust}
pub fn add3(x: i64) -> i64 { x + 3 }
```

`add6/Cargo.toml`{.filename}
```INI
[dependencies]
add3 = "1.0.0"
```

`add6_64/Cargo.toml`{.filename}
```INI
[dependencies]
add3 = "2.0.0"
```

`diamond/Cargo.toml`{.filename}
```INI
[dependencies.add6_64]
path = "/Users/fklock/Dev/Rust/add6_64/"

[dependencies.add6]
path = "/Users/fklock/Dev/Rust/add6/"
```
-->
