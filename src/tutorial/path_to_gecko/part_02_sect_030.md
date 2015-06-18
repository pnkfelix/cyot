## crates.io

Cargo's main feature: dependency management

FIXME do not jump straight into diamond problem

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
