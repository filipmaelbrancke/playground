pub mod day01;

use std::fs;
pub fn get_input_as_string(day: &str, input: &str) -> String {
    fs::read_to_string(format!("input/{day}/{input}.txt")).expect("Unable to read input file")
}
