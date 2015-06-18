## crates.io

Cargo's main feature: dependency management

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
