`cyot` is a Choose Your Own Tutorial system for teaching and learning Rust.

The system is built upon the [Tango] literate programming tool for Rust,
as well as [pandoc]'s support for slideshows via [reveal.js].

[Tango]: https://github.com/pnkfelix/tango/
[pandoc]: http://pandoc.org/
[reveal.js]: http://lab.hakim.se/reveal-js/

You should be able to jump to [path_to_gecko.md] and see some nicely
rendered text; note that this document is merely the *source* for the
target slideshow; it is not the slideshow itself.

[path_to_gecko.md]: src/tutorial/path_to_gecko.md

## Tangling with Tango and pandoc.

The main prerequisite is that you need to have [pandoc] already
installed and on your `$PATH`.

With that in place, you should also be able to clone this repo and
`cargo build` (and `cargo test`, etc) will work right out of the box,
even though much of the Rust code is stored within Markdown files.

The slideshows are autoamtically generated into `target/slides` during
the build (before the Rust code is even compiled or tested, in fact).

To get the nice [reveal.js] rendering of the slides, you will need to
put a `reveal.js` directory into `target/slides` (or anywhere
alongside where you end up placing the generated .html file.
