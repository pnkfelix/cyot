Okay, how does all this work.  :)

First off, this `mod.md` file (which is partnered via `tango` with
`mod.rs`) is the *only* `.md` file in this directory that will not be
interpreted by `pandoc` as part of the slideshow.

 * *Every* other `.md` file (and thus also every other `.rs` file, due to
   `tango`) will be included by `pandoc` into the slideshow.

 * The invocations of `tango` and `pandoc` are both driven by the
   `build.rs` script at the root of this crate.

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
pub mod part_05;

#[allow(dead_code)]
pub mod part_01_sect_010;
pub mod part_01_sect_011;
pub mod part_01_sect_020;
pub mod part_01_sect_030;

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

### Breaking into Columns

An H1 (i.e. `# Title`) is used to delimit a new vertical section.  It
*must* be followed by an H2, or else it screws up pandoc's inference
of what header level defines the start of a slide.

I need to repeat the previous paragraph, because it is so important.

* This is good:

  ```
  # Column Title

  ## Next Slide In Column

  Optional Content
  ```

* This is bad:

  ```
  # Column Title

  Content without a preceding line prefixed with `##`.
  ```

If you do the latter bad thing, it messes up the *entire* slideshow,
since pandoc will re-interpret the whole slide show such that
each column is squished into a single slide.

(And it can be hard to track down what the source of the squishing
is.)

There is probably some option I could be passing to `pandoc` to
force the `#/##` interpretation and avoid this issue with it
inferring some other structure, but I have not yet bothered to
try to find it and/or encode it in the build script.

### Code Snippets

Every `.md`-file is mapped into a `.rs`-file by `tango`. The content
in such files will be the blocks delimited by fences like:

```
    ```rust { .other_css_stuff }
    /* real rust code that must parse and compile */
    ```
```

Note that `tango` (at least in version 0.2.0) will *only* interpret
blocks that literally start with that exact sequence of characters:
three backquotes and then "rust".

In particular, blocks delimited by *this* fence are ignored by `tango`:

```
    ``` { .rust .other_css_stuff }
    /* code that will be rendered by pandoc but ignored by tango */
    ```
```

This provides a convenient way to include code blocks in a slide that
you do not want to be included in the compilation, for whatever
reason.  Examples include code blocks that demonstrate syntax errors,
or blocks that include redundant definitions from a previous slide.

(Note that a slightly more robust way to deal with redundant
definitions is to factor the presentation into multiple files which
then are mapped to separate modules, thus avoiding the name conflicts.
But putting content into a separate file is not always convenient.)

### Compile error demos

I use the following pattern for code snippets that are meant to
illustrate errors that `rustc` catches at compile-time:

```
    ``` {.rust .compile_error}
    fn this_wont_compile(v: &mut Vec<i32>) -> i32 {
    ...
    }
```

The `.compile_error` bit will attach a class name that is then
specified in the .css file for the code-rendering; look at the
`.css` files in the root of the repository.

### Slide Transitions

For the most part I use the default reveal.js transition, but in some
cases a sequence of slides will deliberately attempt to make only
minute changes to the content (e.g. when we are incrementally
modifying code to fix it, or to illustrate a compile error).

In such cases, you will see notation like:

```
## Slide 1 { data-transition="fade-out" }

Start'n Content

## Slice 2 { data-transition="fade-in" }

Similar Content
```

You need both the `fade-out` and the `fade-in` to get the right effect.
(Use `data-transition="fade" for the middle slides when there are more
than two slides involved.)
