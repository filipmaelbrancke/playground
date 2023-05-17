A Rust version of the fortune command ( https://en.wikipedia.org/wiki/Fortune_(Unix) )

```
fortune command in Rust -- print a random, hopefully interesting, adage

Usage: rfortune [OPTIONS] <FILE>...

Arguments:
  <FILE>...  Input files or directories

Options:
  -m, --pattern <PATTERN>  Pattern
  -i, --insensitive        Case-insensitive pattern matching
  -s, --seed <SEED>        Random seed
  -h, --help               Print help
  -V, --version            Print version
```

Examples::
```
cargo run tests/inputs
cargo run tests/inputs/ascii-art
cargo run tests/inputs/jokes tests/inputs/ascii-art
cargo run -- -m 'Yogi Berra' tests/inputs
cargo run -- -m 'Mark Twain' tests/inputs
case-insensitive matching:
cargo run -- -i -m 'yogi berra' tests/inputs
```
