# Perm: Curly Text Converter
> A simple tool for curling straight/dumb quotation marks ("") and apostrophes (') into their curly/smart (“”’) equivalents.

Perm is a command line utility that accepts either a text file or stdin and converts all of the quotation marks and apostrophes to either their smart/curly form or the dumb/straight form.

## Installation

Perm is written in Rust and can be installed using cargo.

```sh
cargo install perm
```

## Usage

This utility is meant to be a good command line citizen and accept both files and stdin and also output both files and stdout depending on the options selected.

```
USAGE:
    perm [FLAGS] <input> [output]

FLAGS:
    -h, --help          Prints help information
    -s, --straighten    Straighten quotes instead of curling them
    -V, --version       Prints version information

ARGS:
    <input>     Input file, pass a dash ("-") to receive stdin
    <output>    Output file, stdout if not present
```

## Release History

- Version 1.0.0: Initial release.
