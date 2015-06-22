## crates versus modules

  * Crate: A unit of compilation

  * Module: A collection of items

  * Each crate is a tree of modules

<!--
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
-->

## module hierarchy

``` {.rust}
mod a {
    mod b { pub type I = i32; }
    mod c {

        pub fn add3(x: super::b::I) -> super::b::I { x + 3 }
    }
}
```

(whoa that's ugly)

``` {.rust .fragment}
mod a {
    mod b { pub type I = i32; }
    mod c {
        use super::b;
        pub fn add3(x: b::I) -> b::I { x + 3 }
    }
}
```

## can `use` any kind of item

``` {.rust}
mod a {
    mod b { pub type I = i32; }
    mod c {
        use super::b;
        pub fn add3(x: b::I) -> b::I { x + 3 }
    }
}
```

#### or even rename {.fragment data-fragment-index="1" }

``` {.rust .fragment data-fragment-index="1" }
mod a {
    mod b { pub type I = i32; }
    mod c {
        use super::b::I as J;
        pub fn add3(x: J) -> J { x + 3 }
    }
}
```

(consult your local style guidelines)

## module hierarchy and file system

``` {.rust}
mod a {
    mod b { pub type I = i32; }
    mod c {
        use super::b::I;
        pub fn add3(x: I) -> I { x + 3 }
    }
}
```

Or '`mod name;`{.rust}' with subfiles at proper paths

`src/lib.rs`{.filename}
```rust
mod a {
    mod b { pub type I = i32; }
    mod c;
}
```
<!-- Above block is why there is `a/c.rs` file floating around;
     It is keeping us honest. :)
-->

`src/a/c.rs`{.filename}
``` {.rust}
        use super::b::I;
        pub fn add3(x: I) -> I { x + 3 }
```

(Obviously do not indent your code this way.)

## `mod foo;` versus `use foo;`

  * The syntax `mod foo;`{.rust} is *just* sugar for

    ``` {.rust}
    mod foo {
      << insert contents of foo.rs here >>
    }
    ```

  * The `mod`{.rust}-syntax *creates* definitions

  * The `use`{.rust}-syntax imports bindings into namespace

  * Why confusing?

    * `use foo;`{.rust} is not necessarily an error
