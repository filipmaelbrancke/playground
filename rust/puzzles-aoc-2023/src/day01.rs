use crate::get_input_as_string;

pub fn solve() {
    let input = get_input_as_string("day01", "input1");
    let part1_result: u64 = calculate_calibrations_sum_part1(input.clone());
    let part2_result: u64 = calculate_calibrations_sum_part2(input.clone());

    println!("Result part 1: {}", part1_result);
    println!("Result part 2: {}", part2_result);
}

pub fn calculate_calibrations_sum_part1(input: String) -> u64 {
    input.lines().map(calculate_calibration_value_part1()).sum()
}

pub fn calculate_calibration_value_part1() -> fn(&str) -> u64 {
    |line| {
        let first_digit = (0..line.len())
            .find_map(|i| get_first_numeric_digit(&line[i..]))
            .unwrap() as u64;
        let last_digit = (0..line.len())
            .rev()
            .find_map(|i| get_first_numeric_digit(&line[i..]))
            .unwrap() as u64;
        first_digit * 10 + last_digit
    }
}

pub fn get_first_numeric_digit(s: &str) -> Option<u32> {
    let numbers: Vec<char> = s.chars().filter(|c| c.is_numeric()).collect();
    if numbers.is_empty() {
        return None;
    }
    let first_number = numbers.first().unwrap().to_owned();
    first_number.to_digit(10)
}

const DIGITS: [(&str, u32); 10] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn get_digit_at_front(s: &str) -> Option<u32> {
    DIGITS.iter().find_map(
        |(numeric_string, number)| match s.starts_with(numeric_string) {
            true => Some(*number),
            false => s.chars().next().and_then(|c| c.to_digit(10)),
        },
    )
}

pub fn calculate_calibrations_sum_part2(input: String) -> u64 {
    input.lines().map(calculate_calibration_value_part2()).sum()
}

fn calculate_calibration_value_part2() -> fn(&str) -> u64 {
    |line| {
        let first_digit = (0..line.len())
            .find_map(|i| get_digit_at_front(&line[i..]))
            .unwrap() as u64;
        let last_digit = (0..line.len())
            .rev()
            .find_map(|i| get_digit_at_front(&line[i..]))
            .unwrap() as u64;
        first_digit * 10 + last_digit
    }
}

#[cfg(test)]
mod tests {
    use crate::day01::get_digit_at_front;
    use crate::day01::get_first_numeric_digit;

    #[test]
    fn test_get_first_real_number() {
        assert_eq!(get_first_numeric_digit("abc123").unwrap(), 1);

        assert_eq!(get_first_numeric_digit("pqr3stu8vwx").unwrap(), 3);
        assert_eq!(get_first_numeric_digit("a1b2c3d4e5f").unwrap(), 1);
        assert_eq!(get_first_numeric_digit("treb7uchet").unwrap(), 7);

        assert_eq!(
            get_first_numeric_digit("pqr3stu8vwx".chars().rev().collect::<String>().as_str())
                .unwrap(),
            8
        );
        assert_eq!(
            get_first_numeric_digit("a1b2c3d4e5f".chars().rev().collect::<String>().as_str())
                .unwrap(),
            5
        );
        assert_eq!(
            get_first_numeric_digit("treb7uchet".chars().rev().collect::<String>().as_str())
                .unwrap(),
            7
        );
    }

    #[test]
    fn test_get_first_number() {
        assert_eq!(get_digit_at_front("1abc123").unwrap(), 1);
        assert_eq!(get_digit_at_front("two1nine").unwrap(), 2);
        assert_eq!(get_digit_at_front("eightwothree").unwrap(), 8);
        assert_eq!(get_digit_at_front("4nineeightseven2").unwrap(), 4);
        assert_eq!(get_digit_at_front("7pqrstsixteen").unwrap(), 7);
        assert!(get_digit_at_front("abcone2threexyz").is_none());
    }

    #[test]
    fn test_calculate_calibration_value() {
        let calculation_function: fn(&str) -> u64 = super::calculate_calibration_value_part1();
        assert_eq!(calculation_function("1abc2"), 12);
        assert_eq!(calculation_function("pqr3stu8vwx"), 38);
        assert_eq!(calculation_function("a1b2c3d4e5f"), 15);
        assert_eq!(calculation_function("treb7uchet"), 77);
    }

    #[test]
    fn test_calculate_calibration_value_part2() {
        let calculation_function: fn(&str) -> u64 = super::calculate_calibration_value_part2();
        assert_eq!(calculation_function("two1nine"), 29);
        assert_eq!(calculation_function("eightwothree"), 83);
        assert_eq!(calculation_function("abcone2threexyz"), 13);
        assert_eq!(calculation_function("xtwone3four"), 24);
        assert_eq!(calculation_function("4nineeightseven2"), 42);
        assert_eq!(calculation_function("zoneight234"), 14);
        assert_eq!(calculation_function("7pqrstsixteen"), 76);
    }

    #[test]
    fn test_aoc_example1() {
        let v = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        assert_eq!(super::calculate_calibrations_sum_part1(v.to_string()), 142);
    }

    #[test]
    fn test_aoc_example2() {
        let v = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        assert_eq!(super::calculate_calibrations_sum_part2(v.to_string()), 281);
    }
}
