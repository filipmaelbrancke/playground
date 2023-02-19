A Rust version of the wc command ( https://en.wikipedia.org/wiki/Wc_(Unix) )

```
wc command in Rust -- reads either standard input or a list of computer files and generates one or more of the following statistics: newline count, word count, and byte count


Usage: rwc [OPTIONS] [FILE]...

Arguments:
  [FILE]...  Input file(s) [default: -]

Options:
  -w, --words    Show word count
  -c, --bytes    Show byte count
  -m, --chars    Show character count
  -l, --lines    Show line count
  -h, --help     Print help
  -V, --version  Print version
```
