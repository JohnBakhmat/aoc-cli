# Advent of Code CLI

A project that aims to make it easier to work on the [Advent of Code](https://adventofcode.com/) challenges, by providing a CLI to generate folder structures and add your favourite language's config.

## Installation

You need to have [Rust](https://www.rust-lang.org/) installed to build the project. (I'm not a rust guy, i used 1.74.0-nightly while building it. Use your brain to figure out the minimum version required <3)

```sh
cargo install --git https://github.com/johnbakhmat/aoc-cli.git
```

## Usage

1. First initialize the project with `aoc-cli init <path> <year>`

```fish
aoc-cli init ./aoc 2017
```

* `path` - is optional and defaults to `./` but is **VERY RECOMMENDED**
* `year` - is optional and defaults to the current year, but also accepts specific years, like `2017` or `2020`. If you want to create folders for all years, use `all`.

