use crate::Column::*;
use clap::{Arg, ArgAction, Command};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

type CommResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Default)]
pub struct Config {
    file1: String,
    file2: String,
    show_col1: bool,
    show_col2: bool,
    show_col3: bool,
    case_insensitive: bool,
    delimiter: String,
}

enum Column<'a> {
    // each column holds a &str, which requires a lifetime annotation
    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
}

pub fn get_args() -> CommResult<Config> {
    let matches = Command::new("rcomm")
        .version("0.1.0")
        .author("Filip Maelbrancke")
        .about("Comm command in rust")
        .arg(
            Arg::new("file1")
                .value_name("FILE1")
                .help("Input file 1")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("file2")
                .value_name("FILE2")
                .help("Input file 2")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("suppress_col1")
                .short('1')
                .action(ArgAction::SetTrue)
                .help("Suppress printing of column 1"),
        )
        .arg(
            Arg::new("suppress_col2")
                .short('2')
                .action(ArgAction::SetTrue)
                .help("Suppress printing of column 2"),
        )
        .arg(
            Arg::new("suppress_col3")
                .short('3')
                .action(ArgAction::SetTrue)
                .help("Suppress printing of column 3"),
        )
        .arg(
            Arg::new("insensitive")
                .short('i')
                .action(ArgAction::SetTrue)
                .help("Case-insensitive comparison of lines"),
        )
        .arg(
            Arg::new("delimiter")
                .short('d')
                .long("output-delimiter")
                .value_name("DELIM")
                .help("Output delimiter")
                .default_value("\t"),
        )
        .get_matches();

    Ok(Config {
        file1: matches.get_one("file1").cloned().unwrap(),
        file2: matches.get_one("file2").cloned().unwrap(),
        show_col1: !matches.get_flag("suppress_col1"),
        show_col2: !matches.get_flag("suppress_col2"),
        show_col3: !matches.get_flag("suppress_col3"),
        case_insensitive: matches.get_flag("insensitive"),
        delimiter: matches.get_one("delimiter").cloned().unwrap(),
    })
}

pub fn run(config: Config) -> CommResult<()> {
    // println!("{:#?}", config);

    let file1 = &config.file1;
    let file2 = &config.file2;

    if file1 == "-" && file2 == "-" {
        return Err(From::from("Both input files cannot be STDIN (\"-\")"));
    }

    let handle_case_sensitivity = |line: String| {
        if config.case_insensitive {
            line.to_lowercase()
        } else {
            line
        }
    };

    let print = |col: Column| {
        let mut columns = vec![];
        match col {
            Col1(val) => {
                if config.show_col1 {
                    columns.push(val);
                }
            }
            Col2(val) => {
                if config.show_col2 {
                    if config.show_col1 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
            Col3(val) => {
                if config.show_col3 {
                    if config.show_col1 {
                        columns.push("");
                    }
                    if config.show_col2 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
        };

        if !columns.is_empty() {
            println!("{}", columns.join(&config.delimiter));
        }
    };

    let mut lines1 = open(file1)?
        .lines()
        .filter_map(Result::ok)
        .map(handle_case_sensitivity);
    let mut lines2 = open(file2)?
        .lines()
        .filter_map(Result::ok)
        .map(handle_case_sensitivity);
    let mut line1 = lines1.next();
    let mut line2 = lines2.next();

    while line1.is_some() || line2.is_some() {
        match (&line1, &line2) {
            (Some(val1), Some(val2)) => match val1.cmp(val2) {
                Equal => {
                    print(Col3(val1));
                    line1 = lines1.next();
                    line2 = lines2.next();
                }
                Less => {
                    print(Col1(val1));
                    line1 = lines1.next();
                }
                Greater => {
                    print(Col2(val2));
                    line2 = lines2.next();
                }
            },
            (Some(val1), None) => {
                print(Col1(val1));
                line1 = lines1.next();
            }
            (None, Some(val2)) => {
                print(Col2(val2));
                line2 = lines2.next();
            }
            _ => (),
        }
    }

    Ok(())
}

fn open(filename: &str) -> CommResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| format!("{filename}: {e}"))?,
        ))),
    }
}
