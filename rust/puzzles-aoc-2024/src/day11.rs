use crate::get_input_as_string;
use std::collections::HashMap;

pub fn solve() {
    let input = get_input_as_string("day11", "input");

    part_one(input.clone());
    part_two(input.clone());
}

struct Stones {
    stones: Vec<StoneNumber>,
    cache: HashMap<(u64, u64), u64>,
}

impl From<String> for Stones {
    fn from(input: String) -> Self {
        Stones {
            stones: input
                .split_whitespace()
                .map(|x| x.parse::<StoneNumber>().unwrap())
                .collect(),
            cache: HashMap::new(),
        }
    }
}

type StoneNumber = u64;

impl Stones {
    fn simulate(&mut self, number_of_blinks: u64) -> u64 {
        let stone_numbers: Vec<StoneNumber> = self.stones.to_vec();
        stone_numbers
            .iter()
            .map(|&stone_number| self.count(stone_number, number_of_blinks))
            .sum()
    }

    /* Stone transformation rules:
    If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
    If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
    If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.*/
    fn process(stone_number: StoneNumber) -> Vec<StoneNumber> {
        if stone_number == 0 {
            return vec![1];
        }
        let number_of_digits = stone_number.to_string().len() as u32;
        if number_of_digits % 2 == 0 {
            let (left_stone_number, right_stone_number) = Self::split_number(stone_number);
            vec![left_stone_number, right_stone_number]
        } else {
            vec![stone_number * 2024]
        }
    }

    fn split_number(number: StoneNumber) -> (StoneNumber, StoneNumber) {
        let number_str = number.to_string();
        let len = number_str.len();
        let half_len = len / 2;

        let left_half = &number_str[..half_len];
        let right_half = &number_str[half_len..];

        let left_number = left_half.parse::<u64>().unwrap_or(0);
        let right_number = right_half.parse::<u64>().unwrap_or(0);

        (left_number, right_number)
    }

    fn count(&mut self, stone_number: StoneNumber, iterations: u64) -> u64 {
        match (iterations, self.cache.get(&(stone_number, iterations))) {
            (0, _) => 1,
            (_, Some(&count)) => count,
            (_, None) => {
                let count = Self::process(stone_number)
                    .into_iter()
                    .map(|num| self.count(num, iterations - 1))
                    .sum();
                self.cache.insert((stone_number, iterations), count);
                count
            }
        }
    }
}

fn part_one(input: String) {
    let part_one = run_logic(input, 25);
    println!("Part one: {}", part_one);
}

fn part_two(input: String) {
    let part_two = run_logic(input, 75);
    println!("Part two: {}", part_two);
}

fn run_logic(input: String, number_of_blinks: u64) -> u64 {
    Stones::from(input).simulate(number_of_blinks)
}

#[cfg(test)]
mod tests {
    use crate::day11::run_logic;

    fn get_example_input() -> String {
        String::from("125 17")
    }

    #[test]
    fn test_split_stone_number() {
        let (left, right) = super::Stones::split_number(99);
        assert_eq!(left, 9);
        assert_eq!(right, 9);
        let (left, right) = super::Stones::split_number(1000);
        assert_eq!(left, 10);
        assert_eq!(right, 0);
    }

    #[test]
    fn test_process_stone_number() {
        let result = super::Stones::process(0);
        assert_eq!(result, vec![1]);
        let result = super::Stones::process(99);
        assert_eq!(result, vec![9, 9]);
        let result = super::Stones::process(999);
        assert_eq!(result, vec![2021976]);
        let result = super::Stones::process(1000);
        assert_eq!(result, vec![10, 0]);
    }

    #[test]
    fn test_part_one_from_example() {
        assert_eq!(run_logic(get_example_input(), 6), 22);
        assert_eq!(run_logic(get_example_input(), 25), 55312);
    }

    #[test]
    fn test_part_one_solution() {
        let input = super::get_input_as_string("day11", "input");
        assert_eq!(run_logic(input, 25), 216042);
    }

    #[test]
    fn test_part_two_solution() {
        let input = super::get_input_as_string("day11", "input");
        assert_eq!(run_logic(input, 75), 255758646442399);
    }
}
