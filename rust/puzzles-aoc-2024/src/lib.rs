pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;

use std::fs;
pub fn get_input_as_string(day: &str, input: &str) -> String {
    fs::read_to_string(format!("input/{day}/{input}.txt")).expect("Unable to read input file")
}
