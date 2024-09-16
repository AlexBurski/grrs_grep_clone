# grrs

`grrs` is a simple command-line tool for searching patterns in files, inspired by the `grep` command. It supports both sequential and parallel searching.

## Features

- Search for patterns in text files
- Case-sensitive and case-insensitive search options
- Parallel search for improved performance on large files

## Installation

### Prerequisites

- Rust programming language (https://www.rust-lang.org/tools/install)
- Git (optional, for cloning the repository)

### Steps

1. Clone the repository (or download the source code):
   ```
   git clone https://github.com/yourusername/grrs.git
   cd grrs
   ```

2. Build and install the tool:
   ```
   cargo install --path .
   ```

This will install the `grrs` binary in your Cargo binary directory (usually `~/.cargo/bin/`). Make sure this directory is in your PATH.

## Usage

The basic syntax for using `grrs` is:

```
grrs [OPTIONS] <PATTERN> <PATH>
```

### Arguments:
- `<PATTERN>`: The pattern to search for
- `<PATH>`: The path to the file to search in

### Options:
- `-i, --ignore-case`: Perform case-insensitive search
- `-p, --parallel`: Use parallel processing for large files
- `-h, --help`: Print help information

## Examples

1. Search for "TODO" in a file:
   ```
   grrs TODO src/main.rs
   ```

2. Case-insensitive search for "error" using parallel processing:
   ```
   grrs -i -p error large_log_file.txt
   ```

3. Get help information:
   ```
   grrs --help
   ```

## Acknowledgments

This project is based on:
- Examples from the [Rust CLI Book](https://rust-cli.github.io/book/index.html)
- Code generated with assistance from Anthropic's Claude AI
- Original contributions by Alex Burski

## Contributing

Contributions are welcome! Here's how you can contribute:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

Please make sure to update tests as appropriate and adhere to the existing coding style.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
