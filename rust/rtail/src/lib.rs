use crate::TakeValue::*;
use clap::{Arg, ArgAction, Command};
use regex::Regex;
use std::error::Error;
use once_cell::sync::OnceCell;

type TailResult<T> = Result<T, Box<dyn Error>>;

static NUM_REGEX: OnceCell<Regex> = OnceCell::new();

#[derive(Debug, PartialEq)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

impl Default for TakeValue {
    fn default() -> Self {
        PlusZero
    }
}

#[derive(Debug, Default)]
pub struct Config {
    files: Vec<String>,
    lines: TakeValue,
    bytes: Option<TakeValue>,
    quiet: bool,
}

pub fn get_args() -> TailResult<Config> {
    let matches = Command::new("rtail")
        .version("0.1.0")
        .author("Filip Maelbrancke")
        .about("Tail command in rust")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .conflicts_with("lines")
                .help("Number of bytes"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("Suppress headers"),
        )
        .get_matches();

    let lines = matches
        .get_one("lines")
        .cloned()
        .map(|v: String| parse_num(v.as_str()))
        .transpose()
        .map_err(|e| format!("illegal line count -- {e}"))?;

    let bytes = matches
        .get_one("bytes")
        .cloned()
        .map(|v: String| parse_num(v.as_str()))
        .transpose()
        .map_err(|e| format!("illegal byte count -- {e}"))?;

    //let default_config: Config = Default::default();
    Ok(Config {
        files: matches
            .get_many("files")
            .expect("files required")
            .cloned()
            .collect(),
        lines: lines.unwrap(),
        bytes,
        quiet: matches.get_flag("quiet"),
    })
}

fn parse_num(val: &str) -> TailResult<TakeValue> {
    // regex to find optional leading + or - sign followed by one or more numbers
    //let num_regex = Regex::new(r"^([+-])?(\d+)$").unwrap();
    let num_regex = NUM_REGEX.get_or_init(|| Regex::new(r"^([+-])?(\d+)$").unwrap());

    match num_regex.captures(val) {
        Some(captures) => {
            // if regex matches -> optional sign is first capture // assume minus sign when no match
            let sign = captures.get(1).map_or("-", |m| m.as_str());
            // digits of the number = second capture
            let num = format!("{}{}", sign, captures.get(2).unwrap().as_str());
            if let Ok(val) = num.parse() {
                // attempt to parse number as an i64 (Rust infers from function return type)
                if sign == "+" && val == 0 {
                    Ok(PlusZero)
                } else {
                    Ok(TakeNum(val))
                }
            } else {
                Err(From::from(val)) // return unparseable number as error
            }
        }
        _ => Err(From::from(val)), // return invalid argument as error
    }
}

pub fn run(config: Config) -> TailResult<()> {
    println!("{:#?}", config);
    Ok(())
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::{parse_num, TakeValue::*};

    #[test]
    fn test_parse_num() {
        // All integers should be interpreted as negative numbers
        let res = parse_num("3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        // A leading "+" should result in a positive number
        let res = parse_num("+3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(3));

        // An explicit "-" value should result in a negative number
        let res = parse_num("-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        // Zero is zero
        let res = parse_num("0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(0));

        // Plus zero is special
        let res = parse_num("+0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), PlusZero);

        // Test boundaries
        let res = parse_num(&i64::MAX.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = parse_num(&(i64::MIN + 1).to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = parse_num(&format!("+{}", i64::MAX));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MAX));

        let res = parse_num(&i64::MIN.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN));

        // A floating-point value is invalid
        let res = parse_num("3.14");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "3.14");

        // Any non-integer string is invalid
        let res = parse_num("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "foo");
    }
}
