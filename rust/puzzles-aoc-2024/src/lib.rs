pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

use std::fs;
pub fn get_input_as_string(day: &str, input: &str) -> String {
    fs::read_to_string(format!("input/{day}/{input}.txt")).expect("Unable to read input file")
}
