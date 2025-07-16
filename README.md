# sourcecfg-rs
A Rust library and CLI tool for parsing and formatting [config files](https://developer.valvesoftware.com/wiki/CFG) from source engine games.

## Usage
Compile and run the formatter using either cargo directly by invoking `cargo run -- <params to sourcecfg_fmt>` or by first compiling the program using `cargo build --release`
and using the produced binary found in: `target/release`.<br>
Output from `--help`:
```
Usage: sourcecfg_fmt [OPTIONS]

Options:
  -i, --input <INPUT>  Read CFG from a file instead of stdin
  -m, --minify         Minify instead of prettifying
  -h, --help           Print help
  -V, --version        Print version
```

Examples of using the formatters from the library can be found in `lib/examples` or by examining the code for the CLI.
