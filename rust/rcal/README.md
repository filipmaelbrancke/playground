A Rust version of the cal command ( https://en.wikipedia.org/wiki/Cal_(command) )

```
Cal command in rust

Usage: rcal [OPTIONS] [YEAR]

Arguments:
  [YEAR]  Year (1-9999)

Options:
  -m <MONTH>      Month name or number (1-12)
  -y, --year      Show whole current year
  -h, --help      Print help
  -V, --version   Print version
```

Examples::
```
cargo run -- -h
cargo run
cargo run -- 1066
cargo run -- 0
```
