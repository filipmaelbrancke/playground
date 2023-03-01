use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use clap::{Arg, ArgAction, Command};

type RUniqResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool
}

pub fn get_args() -> RUniqResult<Config> {
    let matches = Command::new("runiq")
        .version("0.1.0")
        .author("Filip Maelbrancke")
        .about("Uniq command in Rust")
        .arg(
            Arg::new("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                .default_value("-")
        )
        .arg(
            Arg::new("out_file")
                .value_name("OUT_FILE")
                .help("Output file")
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .action(ArgAction::SetTrue)
                .help("Show counts")
        )
        .get_matches();

    Ok(Config {
        in_file: matches.get_one("in_file").cloned().unwrap(),
        out_file: matches.get_one("out_file").cloned(),
        count: matches.get_flag("count")
    })
}

pub fn run(config: Config) -> RUniqResult<()> {
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;

    let mut out_file: Box<dyn Write> = match &config.out_file { // open output with given filename or STDOUT
        Some(out_file_name) => Box::new(File::create(out_file_name)?),
        _ => Box::new(io::stdout())
    };

    //let print = |count: u64, text: &str|  {       // print closure for formatting and writing the output
    // print close must be declared mut since out_file filehandle is borrowed
    let mut print = |count: u64, text: &str| -> RUniqResult<()> {
        if count > 0 {
            if config.count {
                write!(out_file, "{:>4} {}", count, text)?;    // print count right-justified on four characters wide
            } else {
                write!(out_file, "{}", text)?;
            }
        };
        Ok(())
    };

    let mut line = String::new();
    let mut previous_line = String::new();
    let mut count: u64 = 0;

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 { // EOF -> break out of the loop
            break;
        }
        if line.trim_end() != previous_line.trim_end() { // trim trailing whitespace
            print(count, &previous_line)?;
            previous_line = line.clone();
            count = 0;
        }
        count += 1;
        line.clear();
    }

    print(count, &previous_line)?;   // last line of the file

    Ok(())
}

fn open(filename: &str) -> RUniqResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
