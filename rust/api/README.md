Playing around with backend development in Rust

# Setup

```
cargo install --version="~0.6" sqlx-cli --no-default-features --features rustls,postgres
```


# Build - Run

```
./dev.sh

cargo build

cargo run

http localhost:8000

```

## debug test case

```
TEST_LOG=true cargo test health_check_should_return_ok | bunyan
```
(bunyan CLI = prettify outputted logs)

# Misc

Audit dependencies: `cargo audit`

# Extra

Topics to revisit:
- Serialisation in Rust: serde
  - set of interfaces / data model
  - Serialisation: implementation of `Serializer` trait
  - Serialize trait: implementation of `Serialize::serialize` for Rust type = decomposition using the methods available on `Serializer` trait
  - 
