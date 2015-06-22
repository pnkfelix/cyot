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
pub mod part_05;

#[allow(dead_code)]
pub mod part_01_sect_010;
pub mod part_01_sect_011;
pub mod part_01_sect_020;
pub mod part_01_sect_030;
pub mod part_01_sect_040;

pub mod part_02_sect_010;
pub mod part_02_sect_020;
pub mod part_02_sect_021;
pub mod part_02_sect_022;
pub mod part_02_sect_030;
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

## Testing

Most of the slides are written with `#[test]` markers explicitly
on them.

However, some are really only intended to be used as standalone
programs (e.g. in the playpen or on a local machine). To ensure
that I at least exercise those code paths as well regularly,
I am explicitly including calls to those `main` functions here.

If need be, this single monolithic function can later be factored into
individual calls so that the testing report is more finely
grained. But I do not yet know if that is necessary.

```
#[test]
fn main_functions() {
    overview::main();
    part_01_sect_010::main();
    part_01_sect_011::main();
    part_04::main();
}
```

## Formatting Notes

Every H2 (i.e. `## Title`, followed by content) starts off a new
slide.

An hline (`----`) will immediate end a slide, so that follow-on
content is on its own successor slide.

An H1 (i.e. `# Title`) is used to delimit a new vertical section.  It
*must* be followed by an H2, or else it screws up pandoc's inference
of what header level defines the start of a slide.
