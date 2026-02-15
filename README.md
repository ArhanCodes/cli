# naan (CLI)

A small Rust CLI that behaves like a simplified `grep`.

## Features
- Search a file for a **literal string** or a **regex**
- Print matching lines with **line numbers**
- `--ignore-case`, `--count`, `--invert-match`

## Install / Run (dev)

```bash
cd cli
cargo run -- "pattern" path/to/file.txt
```

## Usage

```bash
naan [OPTIONS] <PATTERN> <PATH>

Arguments:
  <PATTERN>  Literal pattern (default) or regex (with --regex)
  <PATH>     Path of the file to search

Options:
  -i, --ignore-case     Case-insensitive search
  -r, --regex           Treat PATTERN as a regular expression
  -n, --line-number     Print line numbers (default: on)
  -c, --count           Print only the number of matching lines
  -v, --invert-match    Select non-matching lines
  -h, --help            Print help
  -V, --version         Print version
```

## Examples

```bash
# Find lines containing "error"
naan error ./logs.txt

# Case-insensitive
naan -i error ./logs.txt

# Regex search
naan -r "^WARN|^ERROR" ./logs.txt

# Count matches
naan -c -i error ./logs.txt
```

## Notes
This is intentionally small and easy to extend. Next upgrades you can add:
- Search directories recursively
- `--before-context/--after-context`
- JSON output mode
- Streaming large files line-by-line (already done) + mmap
