use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

type RHeadResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: u64,
    bytes: Option<u64>,
}

pub fn get_args() -> RHeadResult<Config> {
    let matches = Command::new("rhead")
        .version("0.1.0")
        .author("Filip Maelbrancke")
        .about("Head command in Rust")
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .value_parser(clap::value_parser!(u64).range(1..))
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .num_args(0..=1)
                .conflicts_with("lines")
                .value_parser(clap::value_parser!(u64).range(1..))
                .help("Number of bytes"),
        )
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .default_value("-"),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many("files")
            .expect("file(s) required")
            .cloned()
            .collect(),
        lines: matches.get_one("lines").cloned().unwrap(),
        bytes: matches.get_one("bytes").cloned(),
    })
}

pub fn run(config: Config) -> RHeadResult<()> {
    let number_of_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),

            Ok(mut file) => {
                if number_of_files > 1 {
                    // only print headers if multiple files requested
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" }, // print newlines after first file
                        filename
                    );
                }

                if let Some(num_bytes) = config.bytes {
                    // if config.bytes is Some number of bytes to read
                    let mut handle = file.take(num_bytes); // read requested number of bytes
                    let mut buffer = vec![0; num_bytes as usize]; // create mutable buffer of length num_bytes (filled with zeros)
                    let bytes_read = handle.read(&mut buffer)?; // read asked number of bytes from filehandle into buffer / bytes_read = actual number of bytes read, can be fewer than requested
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                // convert bytes into String, if invalid UTF-8 convert to the unknown character
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> RHeadResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

/*
Instead of trying to parse the incoming string to an integer ourselves, moved over
to Clap's value parser

fn parse_positive_int(val: &str) -> RHeadResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val))
    }
}

#[test]
fn test_parse_positive_int() {
    // 42 is valid integer
    let res = parse_positive_int("42");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 42);

    // strings are no valid integers
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // 0 is not a valid positive integer - 0 is neither positive nor negative
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}*/
