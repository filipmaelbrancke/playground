use crate::get_input_as_string;
use scan_fmt::scan_fmt;
use std::collections::HashSet;

pub fn solve() {
    let input = get_input_as_string("day04", "input");

    let result_part1 = calculate_total_points(input.clone());
    println!("Result part 1: {result_part1}");
}

pub fn calculate_total_points(input: String) -> u32 {
    let result = input
        .lines()
        .map(parse_game_line)
        .map(check_amount_of_successful_numbers)
        .filter(|amount_of_successful_numbers| *amount_of_successful_numbers > 0)
        .map(|amount_of_successful_numbers| 2_u32.pow(amount_of_successful_numbers - 1))
        .sum();
    result
}

fn check_amount_of_successful_numbers(
    (winning_numbers, game_numbers): (HashSet<u32>, HashSet<u32>),
) -> u32 {
    game_numbers.intersection(&winning_numbers).count() as u32
}

fn parse_game_line(game_line: &str) -> (HashSet<u32>, HashSet<u32>) {
    let (winning_numbers, numbers) = scan_fmt!(
        game_line,
        "Card {*d}: {/[0-9 ]*/} | {/[0-9 ]*$/}",
        String,
        String
    )
    .unwrap();
    let winning_numbers = winning_numbers
        .split(' ')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<HashSet<u32>>();
    let numbers = numbers
        .split(' ')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<HashSet<u32>>();
    (winning_numbers, numbers)
}

#[cfg(test)]
mod tests {
    use crate::day04::{
        calculate_total_points, check_amount_of_successful_numbers, parse_game_line,
    };
    use crate::get_input_as_string;
    use std::collections::HashSet;

    #[test]
    fn test_parse_game_line() {
        let test = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let (winning_numbers, game_numbers) = parse_game_line(test);
        assert_eq!(winning_numbers.len(), 5);
        assert_eq!(winning_numbers, HashSet::from([41, 48, 83, 86, 17]));
        assert_eq!(game_numbers, HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]));
    }

    #[test]
    fn test_check_successful_numbers() {
        let test = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let (winning_numbers, game_numbers) = parse_game_line(test);
        assert_eq!(
            check_amount_of_successful_numbers((winning_numbers, game_numbers)),
            4
        );
    }

    #[test]
    fn test_aoc_example() {
        let test_input = get_input_as_string("day04", "example");
        assert_eq!(calculate_total_points(test_input), 13);
    }
}
