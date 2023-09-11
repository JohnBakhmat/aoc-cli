# Advent of Code CLI

A project that aims to make it easier to work on the [Advent of Code](https://adventofcode.com/) challenges, by providing a CLI to generate folder structures and add your favourite language's config.

## Installation

You need to have [Rust](https://www.rust-lang.org/) installed to build the project. (I'm not a rust guy, i used 1.74.0-nightly while building it. Use your brain to figure out the minimum version required <3)

```sh
cargo install --git https://github.com/johnbakhmat/aoc-cli.git
```

## Usage

### Initialize project
Command belove will create folder structure for **current year**.
```bash
mkdir ./aoc
cd ./aoc
aoc-cli init
```

You can also initialize the project with `aoc-cli init <path> <year>`

```bash
aoc-cli init ./aoc 2017
```

* `path` - is optional and defaults to `./` (but if you specify year it is **REQUIRED**)
* `year` - is optional and defaults to the current year, but also accepts specific years, like `2017` or `2020` (Advent of Code started at `2015` so its a bottom limit). If you want to create folders for all years, use `all`.

### Add some language

You can add your favourite language to the project with `aoc-cli add <language> <path>`. 

```bash
aoc-cli add python ./aoc
```

* `<language>` - is required. If the language you are trying to add is not supported, please open an issue or PR at [aoc-templates](https://github.com/johnbakhmat/aoc-templates) repo.
* `<path>` - is optional and defaults to `./`

## License
Go wild, MIT license. 