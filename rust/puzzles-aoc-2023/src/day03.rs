use crate::get_input_as_string;
use itertools::Itertools;
use ndarray::Array2;

pub fn solve() {
    let input = get_input_as_string("day03", "input");

    let schematic = load_engine_schematic_in_2dimensional_array(input, 140);

    let result_part1 = calculate_part_numbers_sum(&schematic);
    println!("Result part 1: {result_part1}");
}

pub fn calculate_part_numbers_sum(schematic: &Array2<char>) -> u32 {
    let size1 = schematic.shape()[0];
    let size2 = schematic.shape()[1];

    let mut current_number: Option<u32> = None;
    let mut has_adjacent_symbol: bool = false;
    let mut sum: u32 = 0;

    for (x, y) in (0..size1).cartesian_product(0..size2) {
        match (current_number, schematic[(x, y)].to_digit(10)) {
            (None, None) => (),
            (None, Some(d)) => {
                current_number = Some(d as u32);
                has_adjacent_symbol = is_adjacent_to_symbols(&schematic, (x, y));
            }
            (Some(n), None) => {
                if has_adjacent_symbol {
                    sum += n;
                }
                current_number = None;
                has_adjacent_symbol = false;
            }
            (Some(n), Some(d)) => {
                current_number = Some(n * 10 + d as u32);
                has_adjacent_symbol =
                    has_adjacent_symbol || is_adjacent_to_symbols(&schematic, (x, y));
            }
        }
    }

    sum
}

pub fn load_engine_schematic_in_2dimensional_array(input: String, size: usize) -> Array2<char> {
    let mut schematic: Array2<char> = Array2::from_elem((size, size), '.');
    for (line_number, line) in input.lines().enumerate() {
        for (position, character) in line.chars().enumerate() {
            schematic[[line_number, position]] = character;
        }
    }

    //println!("{:?}", schematic);

    schematic
}

fn adjacent_characters((x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    (-1..=1isize)
        .cartesian_product(-1..=1isize)
        .filter(|(dx, dy)| !(*dx == 0 && *dy == 0))
        .map(move |(dx, dy)| (x.saturating_add_signed(dx), y.saturating_add_signed(dy)))
}

fn is_adjacent_to_symbols(schematic: &Array2<char>, (x, y): (usize, usize)) -> bool {
    adjacent_characters((x, y)).any(|position| {
        let matrix_size = schematic.shape()[0];
        return if position.0 >= matrix_size || position.1 >= matrix_size {
            false
        } else {
            schematic[position].is_valid_symbol()
        };
    })
}

pub trait CheckExt {
    fn is_valid_symbol(&self) -> bool;
}

impl CheckExt for char {
    fn is_valid_symbol(&self) -> bool {
        !self.is_ascii_digit() && *self != '.'
    }
}

#[cfg(test)]
mod tests {
    use crate::day03::{
        calculate_part_numbers_sum, load_engine_schematic_in_2dimensional_array, CheckExt,
    };
    use crate::get_input_as_string;

    #[test]
    fn test_valid_character_symbol() {
        assert_eq!('*'.is_valid_symbol(), true);
        assert_eq!('#'.is_valid_symbol(), true);
        assert_eq!('+'.is_valid_symbol(), true);
        assert_eq!('$'.is_valid_symbol(), true);
        assert_eq!('.'.is_valid_symbol(), false);
    }

    #[test]
    fn test_aoc_example() {
        let test_input = get_input_as_string("day03", "example");
        let schematic = load_engine_schematic_in_2dimensional_array(test_input, 10);
        assert_eq!(calculate_part_numbers_sum(&schematic), 4361);
    }
}
