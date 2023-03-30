use crate::EntryType::*;
use std::error::Error;
use clap::{Arg, ArgAction, Command, ValueEnum};
use clap::builder::PossibleValue;
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

type FindResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

// https://docs.rs/clap/latest/clap/trait.ValueEnum.html
// https://github.com/clap-rs/clap/blob/master/examples/tutorial_builder/04_01_enum.rs
impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Dir, File, Link]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Dir => PossibleValue::new("d"),
            File => PossibleValue::new("f"),
            Link => PossibleValue::new("l"),
        })
    }
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> FindResult<Config> {
    let matches = Command::new("rfind")
        .version("0.1.0")
        .author("Filip Maelbrancke")
        .about("Find command in Rust")
        .arg(
            Arg::new("paths")
                .value_name("PATH")
                .help("Search paths")
                .default_value(".")
                .num_args(0..)
        )
        .arg(
            Arg::new("names")
                .value_name("NAME")
                .short('n')
                .long("name")
                .help("Name")
                .value_parser(Regex::new)
                .action(ArgAction::Append)
                .num_args(0..)
        )
        .arg(
            Arg::new("types")
                .value_name("TYPE")
                .short('t')
                .long("type")
                .help("Entry type")
                .value_parser(clap::value_parser!(EntryType))
                .action(ArgAction::Append)
                .num_args(0..)
        )
        .get_matches();

    Ok(Config {
        paths: matches.get_many("paths")
            .expect("paths required")
            .cloned()
            .collect(),
        names: matches.get_many("names")
            .unwrap_or_default()
            .cloned()
            .collect(),
        entry_types: matches.get_many("types")
            .unwrap_or_default()
            .cloned()
            .collect(),
    })
}

pub fn run(config: Config) -> FindResult<()> {
    let type_filter = |entry: &DirEntry| {  // closure to filter entries by type
        config.entry_types.is_empty() ||    // no entry types defined -> true
            config
                .entry_types
                .iter()
                .any(|entry_type| match entry_type {
                    Link => entry.path_is_symlink(),
                    Dir => entry.file_type().is_dir(),
                    File => entry.file_type().is_file()
                })
    };

    let name_filter = |entry: &DirEntry| {  // closure to filter entries by name
        config.names.is_empty() ||      // no name regexes present -> true
            config
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in config.paths {
        let entries = WalkDir::new(path)
            // turn WalkDir into iterator
            .into_iter()
            // remove and print bad results to STDERR (while allowing Ok results to pass through)
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry)
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();

        println!("{}", entries.join("\n"));     // print found entries joined on newlines
    }

    Ok(())

    /*
    // naive filtering - too ugly/difficult of an approach if wanting to add more selection criteria (file size, ownership, modification time, ...)
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {

                    if (config.entry_types.is_empty() || config.entry_types.iter().any(|entry_type| {
                        match entry_type {
                            Link => entry.file_type().is_symlink(),
                            Dir => entry.file_type().is_dir(),
                            File => entry.file_type().is_file()
                        }
                    }))
                    && (config.names.is_empty() || config.names.iter().any(|re| {
                        re.is_match(&entry.file_name().to_string_lossy())
                    }))
                    {
                        println!("{}", entry.path().display());
                    }
                }
            }
        }
    }
    Ok(())*/
}
