## crates versus modules

  * Crate: A unit of compilation

  * Module: A collection of items

## crates

  * A crate is a unit of compilation
  * Code can link to other previously compiled crates
  * `extern crate other_crate;`{.rust}
  * compiler finds it in file system, extracts names and types of its exports
  * Makes `other_crate` available as a name to reference

## modules

  * Module is a collection of items
  * Purpose: namespacing and encapsulation
  * Note: a submodule is a kind of item.
    * i.e. modules form a hierarchy
  * Modules can have cyclic references, *within* a single crate

## module hierarchy and file system

```rust
mod a { mod b { pub type I = i32; }
        mod c { use super::b::I;
    			pub fn add3(x: I) -> I { x + 3 }
		}
}
```

Or '`mod name;`{.rust}' and subfiles at proper paths

`src/lib.rs`{.filename}
``` {.rust}
mod a { mod b { pub type I = i32; }
        mod c;
}
```

`src/a/c.rs`{.filename}
``` {.rust}
                use super::b::I;
                pub fn add3(x: I) -> I { x + 3 }
```
