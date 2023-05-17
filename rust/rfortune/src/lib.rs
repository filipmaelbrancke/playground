use clap::{Arg, ArgAction, Command};
use rand::prelude::SliceRandom;
use rand::{rngs::StdRng, SeedableRng};
use regex::{Regex, RegexBuilder};
use std::{
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};
use walkdir::WalkDir;

type FortuneResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    sources: Vec<String>,
    pattern: Option<Regex>,
    seed: Option<u64>,
}

#[derive(Debug)]
pub struct Fortune {
    source: String,
    text: String,
}

pub fn get_args() -> FortuneResult<Config> {
    let matches = Command::new("rfortune")
        .version("0.1.0")
        .author("Filip Maelbrancke")
        .about("Fortune command in rust")
        .arg(
            Arg::new("sources")
                .value_name("FILE")
                .num_args(1..)
                .required(true)
                .help("Input files or directories"),
        )
        .arg(
            Arg::new("pattern")
                .value_name("PATTERN")
                .short('m')
                .long("pattern")
                .help("Pattern"),
        )
        .arg(
            Arg::new("insensitive")
                .short('i')
                .long("insensitive")
                .help("Case-insensitive pattern matching")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("seed")
                .value_name("SEED")
                .short('s')
                .long("seed")
                .value_parser(clap::value_parser!(u64))
                .help("Random seed"),
        )
        .get_matches();

    let pattern = matches
        .get_one("pattern")
        .cloned()
        .map(|val: String| {
            RegexBuilder::new(val.as_str())
                .case_insensitive(matches.get_flag("insensitive"))
                .build()
                .map_err(|_| format!("Invalid --pattern \"{val}\""))
        })
        .transpose()?; // Option<Result> -> Result<Option>

    Ok(Config {
        sources: matches
            .get_many("sources")
            .expect("sources required")
            .cloned()
            .collect(),
        seed: matches.get_one("seed").cloned(),
        pattern,
    })
}

pub fn run(config: Config) -> FortuneResult<()> {
    //println!("{:#?}", config);
    let files = find_files(&config.sources)?;
    let fortunes = read_fortunes(&files)?;

    if let Some(pattern) = config.pattern {
        let mut prev_source = None;
        for fortune in fortunes
            .iter()
            .filter(|fortune| pattern.is_match(&fortune.text))
        {
            if prev_source.as_ref().map_or(true, |s| s != &fortune.source) {
                eprintln!("({})\n%", fortune.source);
                prev_source = Some(fortune.source.clone());
            }
            println!("{}\n%", fortune.text);
        }
    } else {
        println!(
            "{}",
            pick_fortune(&fortunes, config.seed)
                .or_else(|| Some("No fortunes found".to_string()))
                .unwrap()
        )
    }

    Ok(())
}

fn find_files(paths: &[String]) -> FortuneResult<Vec<PathBuf>> {
    let mut files = vec![];

    for path in paths {
        match fs::metadata(path) {
            Err(e) => return Err(format!("{path}: {e}").into()),
            Ok(_) => files.extend(
                WalkDir::new(path)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|f| f.file_type().is_file())
                    .map(|f| f.path().into()),
            ),
        }
    }

    files.sort();
    files.dedup();
    Ok(files)
}

fn read_fortunes(paths: &[PathBuf]) -> FortuneResult<Vec<Fortune>> {
    let mut fortunes = vec![];
    let mut buffer = vec![];

    for path in paths {
        // convert file_name from OsStr to String (lossy in case it is not valid utf-8)
        // result is a clone-on-write smart pointer, so use into_owned to clone if not already owned
        let basename = path.file_name().unwrap().to_string_lossy().into_owned();
        // open file or return error message
        let file = File::open(path)
            .map_err(|e| format!("{}: {}", path.to_string_lossy().into_owned(), e))?;

        for line in BufReader::new(file).lines().filter_map(Result::ok) {
            if line == "%" {
                // single % indicates end of record
                if !buffer.is_empty() {
                    fortunes.push(Fortune {
                        source: basename.clone(),
                        text: buffer.join("\n"),
                    });
                    buffer.clear();
                }
            } else {
                buffer.push(line.to_string());
            }
        }
    }

    Ok(fortunes)
}

fn pick_fortune(fortunes: &[Fortune], seed: Option<u64>) -> Option<String> {
    if let Some(val) = seed {
        let mut rng = StdRng::seed_from_u64(val);
        fortunes.choose(&mut rng).map(|f| f.text.to_string())
    } else {
        let mut rng = rand::thread_rng();
        fortunes.choose(&mut rng).map(|f| f.text.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::{find_files, read_fortunes};
    use std::path::PathBuf;

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let res = find_files(&["./tests/inputs/jokes".to_string()]);
        assert!(res.is_ok());

        let files = res.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(
            files.get(0).unwrap().to_string_lossy(),
            "./tests/inputs/jokes"
        );

        // Fails to find a bad file
        let res = find_files(&["/path/does/not/exist".to_string()]);
        assert!(res.is_err());

        // Finds all the input files, excludes ".dat"
        let res = find_files(&["./tests/inputs".to_string()]);
        assert!(res.is_ok());

        // Check number and order of files
        let files = res.unwrap();
        assert_eq!(files.len(), 5);
        let first = files.get(0).unwrap().display().to_string();
        assert!(first.contains("ascii-art"));
        let last = files.last().unwrap().display().to_string();
        assert!(last.contains("quotes"));

        // Test for multiple sources, path must be unique and sorted
        let res = find_files(&[
            "./tests/inputs/jokes".to_string(),
            "./tests/inputs/ascii-art".to_string(),
            "./tests/inputs/jokes".to_string(),
        ]);
        assert!(res.is_ok());
        let files = res.unwrap();
        assert_eq!(files.len(), 2);
        if let Some(filename) = files.first().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "ascii-art".to_string())
        }
        if let Some(filename) = files.last().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "jokes".to_string())
        }
    }

    #[test]
    fn test_read_fortunes() {
        // One input file
        let res = read_fortunes(&[PathBuf::from("./tests/inputs/jokes")]);
        assert!(res.is_ok());

        if let Ok(fortunes) = res {
            // Correct number and sorting
            assert_eq!(fortunes.len(), 6); // empty fortune should be removed
            assert_eq!(
                fortunes.first().unwrap().text,
                "Q. What do you call a head of lettuce in a shirt and tie?\n\
                A. Collared greens."
            );
            assert_eq!(
                fortunes.last().unwrap().text,
                "Q: What do you call a deer wearing an eye patch?\n\
                A: A bad idea (bad-eye deer)."
            );
        }
        // Multiple input files
        let res = read_fortunes(&[
            PathBuf::from("./tests/inputs/jokes"),
            PathBuf::from("./tests/inputs/quotes"),
        ]);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 11);
    }
}
