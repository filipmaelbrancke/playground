build() {
  while read path; do
    printf "Building project %s\n" "$path"
    cargo build --verbose --manifest-path "$path"
    cargo test --verbose --manifest-path "$path"
    # cargo clippy --verbose --manifest-path "$path"
  done
}

# start at current directory
# find directories starting with letter r
# for each directory, look inside for file named Cargo.toml
find . -type d -name "r*" -exec find {} -name "Cargo.toml" \; | sort -u | build