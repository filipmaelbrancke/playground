use crate::get_input_as_string;

pub fn solve() {
    let input = get_input_as_string("day01", "input");

    part_one(input.clone());
    part_two(input.clone());
}

fn part_one(input: String) {
    let total_distance = find_total_distance(input);
    println!("Part one: {}", total_distance);
}

fn part_two(input: String) {
    let total_similarity = find_total_similarity(input);
    println!("Part two: {}", total_similarity);
}

fn find_total_similarity(input: String) -> u32 {
    let (left_numbers, right_numbers) = parse_input(input);
    let mut total_similarity: u32 = 0;
    for left_number in left_numbers {
        //total_similarity += left_number * right_numbers.iter().filter(|x| x == &&left_number).count() as u32;
        total_similarity += left_number * right_numbers.iter().filter(|x| **x == left_number).count() as u32;
    }
    total_similarity
}

fn find_total_distance(input: String) -> u32 {
    let (mut left_numbers, mut right_numbers) = parse_input(input);
    left_numbers.sort();
    right_numbers.sort();

    left_numbers.into_iter().zip(right_numbers).map(|(left, right)| left.abs_diff(right)).sum()
}

fn parse_input(input: String) -> (Vec<u32>, Vec<u32>) {
    let mut left_numbers: Vec<u32> = Vec::new();
    let mut right_numbers: Vec<u32> = Vec::new();

    for line in input.lines() {
        let NumberPair { left, right } = parse_line_to_pair(line);
        left_numbers.push(left);
        right_numbers.push(right);
    }

    (left_numbers, right_numbers)
}

fn parse_line_to_pair(line: &str) -> NumberPair {
    let mut split_on_whitespace = line.split_whitespace();
    NumberPair {
        left: split_on_whitespace.next().unwrap().parse::<u32>().unwrap(),
        right: split_on_whitespace.next().unwrap().parse::<u32>().unwrap(),
    }
}

#[derive(Debug, PartialEq)]
struct NumberPair {
    left: u32,
    right: u32,
}

#[cfg(test)]
mod tests {
    use crate::day01::{parse_line_to_pair, NumberPair};

    fn get_example_input() -> String {
        String::from("\
3   4
4   3
2   5
1   3
3   9
3   3")
    }

    #[test]
    fn test_get_numbers_from_line() {
        assert_eq!(parse_line_to_pair("3   4"), NumberPair { left: 3, right: 4 });
        assert_eq!(parse_line_to_pair("3   9"), NumberPair { left: 3, right: 9 });
        assert_ne!(parse_line_to_pair("3   9"), NumberPair { left: 3, right: 3 });
    }

    #[test]
    fn test_get_numbers_from_input() {
        let (left_numbers, right_numbers) = super::parse_input(get_example_input());
        assert_eq!(left_numbers, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right_numbers, vec![4, 3, 5, 3, 9, 3]);
    }
    
    #[test]
    fn test_find_total_distance() {
        assert_eq!(super::find_total_distance(get_example_input()), 11);
    }

    #[test]
    fn test_find_total_similarity() {
        assert_eq!(super::find_total_similarity(get_example_input()), 31);
    }
}