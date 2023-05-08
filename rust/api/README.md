Playing around with backend development in Rust

# Setup

```
cargo install --version="~0.6" sqlx-cli --no-default-features --features rustls,postgres
cargo install bunyan
cargo install cargo-udeps
cargo install cargo-watch
cargo install cargo-audit
```


# Build - Run

```
./dev.sh

cargo build

cargo run

http localhost:8000

```

## Sqlx offline mode

keep up-to-date using `cargo sqlx prepare -- --lib`  

## debug test case

```
TEST_LOG=true cargo test health_check_should_return_ok | bunyan
```
(bunyan CLI = prettify outputted logs)

## check for unused dependencies

```
cargo +nightly udeps
```

## monitor source code -> trigger command on file changes

```
cargo watch -x check
```

`cargo-watch` supports command chaining *(start with check / follow with testing, and if tests pass: launch application)*:

```
cargo watch -x check -x test -x run
```

## Testing

Cargo knows about `tests` folder -> integration tests.
Each file within _tests_ folder -> gets compiled as its own crate

# Misc

Audit dependencies for vulnerabilities: `cargo audit`

# Extra

Topics to revisit:

- Serialisation in Rust: serde
  - set of interfaces / data model
  - Serialisation: implementation of `Serializer` trait
  - Serialize trait: implementation of `Serialize::serialize` for Rust type = decomposition using the methods available on `Serializer` trait
- Code coverage

# Resources

- [Zero To Production In Rust](https://www.zero2prod.com/)
