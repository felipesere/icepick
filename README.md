#Icepick is a reimplementation of Selecta in Rust

![Build Status](https://github.com/erwinvaneijk/icepick/actions/workflows/build.yml/badge.svg)

A fuzzy text selector for files and anything else you need to select.
Use it from vim, from the command line, or anywhere you can run a shell command.

## Installation

Currently requires a rustc nightly and compatible cargo.

Clone the repo and then run:

```
cargo build --release
```

and put the resulting `target/release/icepick` executable on your `PATH`.

Then you can pipe input to it and fuzzy select on it:

```
find . -name "*.css" | icepick | xargs rm
```

The above commend would allow you to match on all CSS files in your current
directory and remove the selected one.

For more uses see [the original Ruby implementation](https://github.com/garybernhardt/selecta) by Gary Bernhardt.

## Contributing

* Fork it
* Add a test
* Make it pass
* Open a pull request

All kinds of fixes and improvements are welcome, from improvements in the algorithm to more idomatic rust or documentation improvement.
Even adding an issue for something that is broken is awesome!

If you have an idea to improve performance, run `cargo bench` and see how the results compare.

## Contributors:

@felipesere
@heruku
@carols10cents
@erwinvaneijk

