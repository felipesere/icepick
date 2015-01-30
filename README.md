# Selecta reimplemented in Rust

[![Build Status](https://travis-ci.org/felipesere/selecta.svg?branch=master)](https://travis-ci.org/felipesere/selecta)

A fuzzy text selector for files and anything else you need to select. Use it from vim, from the command line, or anywhere you can run a shell command.

See [the original Ruby implementation](https://github.com/garybernhardt/selecta) for more information.

## Installation

Currently requires a rustc 1.0.0-alpha nightly and compatible cargo.

Run:

```
cargo build
```

and put the resulting `target/selecta` executable on your PATH.
