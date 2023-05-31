A Rust version of the ls command ( https://en.wikipedia.org/wiki/Ls )

```
ls command in Rust -- list directory contents

Usage: rls [OPTIONS] [PATH]...

Arguments:
  [PATH]...  Files and/or directories [default: .]

Options:
  -l, --long     Long listing
  -a, --all      Show all files
  -h, --help     Print help
  -V, --version  Print version
```

### Documentation

Comments with three slashes (///) create a special kind of comment that has the #[doc] attribute.     
(the doc comment should precede the function declaration)

Cargo crate documentation:

```
cargo doc --open --document-private-items
```
