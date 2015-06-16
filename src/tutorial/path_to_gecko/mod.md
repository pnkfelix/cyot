Okay, how does all this work.  :)

First off, this `mod.md` file (which is partnered via `tango` with
`mod.rs`) is the *only* `.md` file in this directory that will not be
interpreted by `pandoc` as part of the slideshow.

*Every* other `.md` file (and thus also every other `.rs` file, due to
`tango`) will be included by `pandoc` into the slideshow.

Second, we list all of the modules that we want to be included in the
module hierarchy here. It is totally unrelated to the slideshow; this
is solely about telling `rustc` what Rust files we want to be included
during the build, and thus what `#[test]`-marked functions should
actually be treated as unit tests.

```rust
pub mod overview;

pub mod part_01;
pub mod part_02;
pub mod part_03;
pub mod part_04;

pub mod part_01_sect_010;
pub mod part_01_sect_011;
pub mod part_01_sect_020;
pub mod part_01_sect_030;
pub mod part_01_sect_040;
pub mod part_01_sect_050;
```

Thus, if you have a file that you want to be in the slideshow but do
not want to be included in the compile, just make sure that it is not
in the list above (and is not included as any submodule via `#[path]`
tricks).

Third, the order of declaration above has nothing to do with the
presentation order in the slideshow. In particular, the slides will be
presented according to the alphabetical sorting of the paths; that's
why I am choosing the module names carefully so that when sorted,
`part_01_sect_010` will come immediately after `part_01`.
