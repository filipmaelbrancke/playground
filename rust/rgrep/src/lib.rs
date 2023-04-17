use clap::{Arg, ArgAction, Command};
use regex::{Regex, RegexBuilder};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{fs, io, mem};
use walkdir::WalkDir;

type GrepResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pattern: Regex,
    files: Vec<String>,
    recursive: bool,
    count: bool,
    invert_match: bool,
}

pub fn get_args() -> GrepResult<Config> {
    let matches = Command::new("rgrep")
        .version("0.1.0")
        .author("Filip Maelbrancke")
        .about("Grep command in rust")
        .arg(
            Arg::new("pattern")
                .value_name("PATTERN")
                .help("Search pattern")
                .required(true),
        )
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("insensitive")
                .short('i')
                .long("insensitive")
                .help("Case-insensitive")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .help("Recursive search")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help("Count occurrences")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("invert")
                .short('v')
                .long("invert-match")
                .help("Invert match")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let pattern: String = matches.get_one("pattern").cloned().unwrap();
    let regex_pattern = RegexBuilder::new(&pattern)
        .case_insensitive(matches.get_flag("insensitive"))
        .build()
        .map_err(|_| format!("Invalid pattern \"{pattern}\""))?;

    Ok(Config {
        pattern: regex_pattern,
        files: matches
            .get_many("files")
            .expect("files required")
            .cloned()
            .collect(),
        recursive: matches.get_flag("recursive"),
        count: matches.get_flag("count"),
        invert_match: matches.get_flag("invert"),
    })
}

pub fn run(config: Config) -> GrepResult<()> {
    let entries = find_files(&config.files, config.recursive);
    let num_files = entries.len();
    // closure to print the output (show filenames depending on number of input files)
    let print = |fname: &str, val: &str| {
        if num_files > 1 {
            print!("{}:{}", fname, val);
        } else {
            print!("{val}");
        }
    };

    for entry in entries {
        match entry {
            Err(e) => eprintln!("{e}"),
            Ok(filename) => match open(&filename) {
                Err(e) => eprintln!("{filename}: {e}"),
                Ok(file) => match find_lines(file, &config.pattern, config.invert_match) {
                    Err(e) => eprintln!("{e}"),
                    Ok(matches) => {
                        if config.count {
                            print(&filename, &format!("{}\n", matches.len()));
                        } else {
                            for line in &matches {
                                print(&filename, line);
                            }
                        }
                    }
                },
            },
        }
    }

    Ok(())
}

fn open(filename: &str) -> GrepResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn find_lines<T: BufRead>(
    mut file: T,
    pattern: &Regex,
    invert_match: bool,
) -> GrepResult<Vec<String>> {
    let mut matches = vec![];
    let mut line = String::new();

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if pattern.is_match(&line) ^ invert_match {
            // ^ = bitwise XOR comparison
            matches.push(mem::take(&mut line));
        }
        line.clear();
    }

    Ok(matches)
}

fn find_files(paths: &[String], recursive: bool) -> Vec<GrepResult<String>> {
    let mut results = vec![];

    for path in paths {
        match path.as_str() {
            "-" => results.push(Ok(path.to_string())),
            _ => match fs::metadata(path) {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        if recursive {
                            for entry in WalkDir::new(path)
                                .into_iter()
                                .flatten()
                                .filter(|e| e.file_type().is_file())
                            {
                                results.push(Ok(entry.path().display().to_string()));
                            }
                        } else {
                            results.push(Err(From::from(format!("{path} is a directory"))));
                        }
                    } else if metadata.is_file() {
                        results.push(Ok(path.to_string()));
                    }
                }
                Err(e) => results.push(Err(From::from(format!("{path}: {e}")))),
            },
        }
    }

    results
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::{find_files, find_lines};
    use rand::distributions::Alphanumeric;
    use rand::Rng;
    use regex::{Regex, RegexBuilder};
    use std::io::Cursor;

    #[test]
    fn test_find_files() {
        // find a file know to exist
        let files = find_files(&["./tests/inputs/fox.txt".to_string()], false);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/fox.txt");

        // reject a directory without the recursive option
        let files = find_files(&["./tests/inputs".to_string()], false);
        assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs is a directory");
        }

        // find_files function should recurse and find 4 files in the directory
        let result = find_files(&["./tests/inputs".to_string()], true);
        let mut files: Vec<String> = result
            .iter()
            .map(|r| r.as_ref().unwrap().replace('\\', "/"))
            .collect();
        // TODO: book sorts files and then assert_eq with expected
        // check whether the below can be written with a bit nicer assertion syntax
        assert_eq!(files.len(), 4);
        let expected = vec![
            "./tests/inputs/bustle.txt".to_string(),
            "./tests/inputs/empty.txt".to_string(),
            "./tests/inputs/fox.txt".to_string(),
            "./tests/inputs/nobody.txt".to_string(),
        ];
        assert!(files.iter().all(|file| expected.contains(file)));

        // random string -> nonexistant file
        let bad_file: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        let files = find_files(&[bad_file], false);
        assert_eq!(files.len(), 1);
        assert!(files[0].is_err());
    }

    #[test]
    fn test_find_lines() {
        let text = b"Lorem\nIpsum\r\nDOLOR";

        // _or_ pattern should match 1 line
        let regex1 = Regex::new("or").unwrap();
        let matches = find_lines(Cursor::new(&text), &regex1, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);

        // inverted, that function should then match the other 2 lines
        let matches = find_lines(Cursor::new(&text), &regex1, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // case-insensitive regex
        let regex2 = RegexBuilder::new("or")
            .case_insensitive(true)
            .build()
            .unwrap();

        // 2 lines should match ("Lorem" and "DOLOR")
        let matches = find_lines(Cursor::new(&text), &regex2, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // inverted -> one line should match
        let matches = find_lines(Cursor::new(&text), &regex2, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);
    }
}
