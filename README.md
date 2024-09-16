# grrs

`grrs` is a simple command-line tool for searching patterns in files, inspired by the `grep` command. It supports both sequential and parallel searching.

## Features

- Search for patterns in text files
- Case-sensitive and case-insensitive search options
- Parallel search for improved performance on large files

## Installation

To install `grrs`, you need to have Rust installed on your system. Then, you can install it using cargo:

```
cargo install --path .
```

## Usage

```
grrs [OPTIONS] <PATTERN> <PATH>

Arguments:
  <PATTERN>  The pattern to search for
  <PATH>     The path to the file to search in

Options:
  -i, --ignore-case  Perform case-insensitive search
  -p, --parallel     Use parallel processing for large files
  -h, --help         Print help information
```

## Examples

Search for "TODO" in a file:
```
grrs TODO src/main.rs
```

Case-insensitive search for "error" using parallel processing:
```
grrs -i -p error large_log_file.txt
```

## Acknowledgments
This project is based on:

Examples from the Rust CLI Book
Code generated with assistance from Anthropic's Claude AI
Original contributions by Alex Burski

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
