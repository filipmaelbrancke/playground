use crate::get_input_as_string;
use itertools::Itertools;

pub fn solve() {
    let input = get_input_as_string("day02", "input");

    part_one(input.clone());
    part_two(input.clone());
}

fn part_one(input: String) {
    let number_of_safe_reports = find_number_of_safe_reports(input);
    println!("Part one: {}", number_of_safe_reports);
}

fn part_two(input: String) {
    let number_of_safe_dampened_reports = find_number_of_safe_dampened_reports(input);
    println!("Part two: {}", number_of_safe_dampened_reports);
}

fn find_number_of_safe_reports(input: String) -> u32 {
    parse_input(input)
        .iter()
        .filter(|report| is_safe_report(report))
        .count() as u32
}

fn find_number_of_safe_dampened_reports(input: String) -> u32 {
    parse_input(input)
        .iter()
        .filter(|report| is_safe_dampened_report(report))
        .count() as u32
}

fn parse_input(input: String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

// Clippy suggested a change from is_safe_report(report: &Vec<i32>)
fn is_safe_report(report: &[i32]) -> bool {
    let differences: Vec<i32> = report.iter().tuple_windows().map(|(a, b)| b - a).collect();
    // the differences either need to have
    // - all the same sign
    //     all increasing or all decreasing
    // AND
    // - differ by at least one and at most three
    let is_either_increasing_or_decreasing =
        differences.iter().all(|&x| x > 0) || differences.iter().all(|&x| x < 0);
    let differs_by_at_least_one_and_at_most_three =
        differences.iter().all(|&x| x.abs() >= 1 && x.abs() <= 3);
    is_either_increasing_or_decreasing && differs_by_at_least_one_and_at_most_three
}

// Same here: Clippy suggestion from report: &Vec<i32>
fn is_safe_dampened_report(report: &[i32]) -> bool {
    for i in 0..report.len() {
        let mut amended = report.to_owned();
        amended.remove(i);
        if is_safe_report(&amended) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {

    fn get_example_input() -> String {
        String::from(
            "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        )
    }

    #[test]
    fn test_parse_input() {
        let input = get_example_input();
        let expected = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(super::parse_input(input), expected);
    }

    #[test]
    fn test_is_safe_report() {
        assert_eq!(super::is_safe_report(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(super::is_safe_report(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(super::is_safe_report(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(super::is_safe_report(&vec![1, 3, 2, 4, 5]), false);
        assert_eq!(super::is_safe_report(&vec![8, 6, 4, 4, 1]), false);
        assert_eq!(super::is_safe_report(&vec![1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn test_part1_example() {
        let input = get_example_input();
        assert_eq!(super::find_number_of_safe_reports(input), 2);
    }

    #[test]
    fn test_is_safe_dampened_report() {
        assert_eq!(super::is_safe_dampened_report(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(super::is_safe_dampened_report(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(super::is_safe_dampened_report(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(super::is_safe_dampened_report(&vec![1, 3, 2, 4, 5]), true);
        assert_eq!(super::is_safe_dampened_report(&vec![8, 6, 4, 4, 1]), true);
        assert_eq!(super::is_safe_dampened_report(&vec![1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn test_part2_example() {
        let input = get_example_input();
        assert_eq!(super::find_number_of_safe_dampened_reports(input), 4);
    }
}
