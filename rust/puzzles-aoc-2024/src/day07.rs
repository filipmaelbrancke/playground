use crate::get_input_as_string;
use itertools::Itertools;
use rayon::prelude::*;

pub fn solve() {
    let input = get_input_as_string("day07", "input");

    part_one(input.clone());
    part_two(input.clone());
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct CalibrationEquation {
    test_result: u64,
    test_numbers: Vec<u64>,
}

impl CalibrationEquation {
    fn is_valid(&self, ops: Vec<Operator>) -> bool {
        for operators in std::iter::repeat(ops)
            .take(self.test_numbers.len() - 1)
            .multi_cartesian_product()
        {
            if self.evaluate(&operators) == self.test_result {
                return true;
            }
        }

        false
    }

    //fn evaluate(&self, operators: &Vec<Operator>) -> u64 {
    fn evaluate(&self, operators: &[Operator]) -> u64 {
        let mut value = self.test_numbers[0];
        for (number_index, operator) in operators.iter().enumerate() {
            value = operator.apply(value, self.test_numbers[number_index + 1]);
        }
        value
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concatenate => format!("{}{}", a, b).parse::<u64>().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Equations {
    equations: Vec<CalibrationEquation>,
}

impl Equations {
    fn from_input(input: &str) -> Equations {
        let equations = input
            .lines()
            .map(|line| {
                if let Some((result, numbers)) = line.split_once(':') {
                    CalibrationEquation {
                        test_result: result.parse::<u64>().unwrap(),
                        test_numbers: numbers
                            .split_whitespace()
                            .map(|number| number.parse::<u64>().unwrap())
                            .collect(),
                    }
                } else {
                    panic!("Unable to parse line: {}", line);
                }
            })
            .collect();
        Equations { equations }
    }
}

fn part_one(input: String) {
    let total_calibration_result = calculate_total_calibration_result(input);
    println!("Part one: {}", total_calibration_result);
}

fn part_two(input: String) {
    let total_calibration_result =
        calculate_total_calibration_result_including_concatenation(input);
    println!("Part two: {}", total_calibration_result);
}

fn calculate_total_calibration_result(input: String) -> u64 {
    calculate_total_sum_from_calibration_results(
        input,
        [Operator::Add, Operator::Multiply].to_vec(),
    )
}

fn calculate_total_calibration_result_including_concatenation(input: String) -> u64 {
    calculate_total_sum_from_calibration_results(
        input,
        vec![Operator::Add, Operator::Multiply, Operator::Concatenate],
    )
}

fn calculate_total_sum_from_calibration_results(input: String, operators: Vec<Operator>) -> u64 {
    Equations::from_input(&input)
        .equations
        .par_iter()
        .filter(|equation| equation.is_valid(operators.clone()))
        .map(|equation| equation.test_result)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::Operator::{Add, Concatenate, Multiply};

    fn get_example_input() -> String {
        String::from(
            "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        )
    }

    #[test]
    fn test_calculate_total_calibration_result_from_example_should_be_3749() {
        let input = get_example_input();
        assert_eq!(super::calculate_total_calibration_result(input), 3749);
    }

    #[test]
    fn test_calculate_total_calibration_result_including_concatenation_from_example_should_be_11387(
    ) {
        let input = get_example_input();
        assert_eq!(
            super::calculate_total_calibration_result_including_concatenation(input),
            11387
        );
    }

    #[test]
    fn test_equations_input_parsing() {
        let input = get_example_input();
        let equations = super::Equations::from_input(&input);
        assert_eq!(equations.equations.len(), 9);
        assert_eq!(equations.equations[0].test_result, 190);
        assert_eq!(equations.equations[0].test_numbers, vec![10, 19]);
        assert_eq!(equations.equations[1].test_result, 3267);
        assert_eq!(equations.equations[1].test_numbers, vec![81, 40, 27]);
        assert_eq!(equations.equations[2].test_result, 83);
        assert_eq!(equations.equations[2].test_numbers, vec![17, 5]);
        assert_eq!(equations.equations[3].test_result, 156);
        assert_eq!(equations.equations[3].test_numbers, vec![15, 6]);
        assert_eq!(equations.equations[4].test_result, 7290);
        assert_eq!(equations.equations[4].test_numbers, vec![6, 8, 6, 15]);
        assert_eq!(equations.equations[5].test_result, 161011);
        assert_eq!(equations.equations[5].test_numbers, vec![16, 10, 13]);
        assert_eq!(equations.equations[6].test_result, 192);
        assert_eq!(equations.equations[6].test_numbers, vec![17, 8, 14]);
        assert_eq!(equations.equations[7].test_result, 21037);
        assert_eq!(equations.equations[7].test_numbers, vec![9, 7, 18, 13]);
        assert_eq!(equations.equations[8].test_result, 292);
        assert_eq!(equations.equations[8].test_numbers, vec![11, 6, 16, 20]);
    }

    #[test]
    fn test_operators() {
        assert_eq!(Add.apply(1, 2), 3);
        assert_eq!(Multiply.apply(2, 3), 6);
        assert_eq!(super::Operator::Concatenate.apply(15, 6), 156);
    }

    #[test]
    fn test_equations_evaluation() {
        let equation = super::CalibrationEquation {
            test_result: 190,
            test_numbers: vec![10, 19],
        };
        assert_eq!(equation.evaluate(&vec![Add]), 29);
        assert_eq!(equation.evaluate(&vec![Multiply]), 190);
        assert_eq!(equation.evaluate(&vec![super::Operator::Concatenate]), 1019);
    }

    #[test]
    fn test_equations_validation() {
        assert_eq!(
            super::CalibrationEquation {
                test_result: 190,
                test_numbers: vec![10, 19],
            }
            .is_valid(vec![Multiply]),
            true
        );
        assert_eq!(
            super::CalibrationEquation {
                test_result: 3267,
                test_numbers: vec![81, 40, 27],
            }
            .is_valid(vec![Multiply, Add]),
            true
        );
        assert_eq!(
            super::CalibrationEquation {
                test_result: 3267,
                test_numbers: vec![81, 40, 27],
            }
            .is_valid(vec![Add, Multiply]),
            true
        );
        assert_eq!(
            super::CalibrationEquation {
                test_result: 292,
                test_numbers: vec![11, 6, 16, 20],
            }
            .is_valid(vec![Add, Multiply, Add]),
            true
        );
        assert_eq!(
            super::CalibrationEquation {
                test_result: 156,
                test_numbers: vec![15, 6],
            }
            .is_valid(vec![Concatenate]),
            true
        );
        assert_eq!(
            super::CalibrationEquation {
                test_result: 7290,
                test_numbers: vec![6, 8, 6, 15],
            }
            .is_valid(vec![Multiply, Concatenate, Multiply]),
            true
        );
        assert_eq!(
            super::CalibrationEquation {
                test_result: 192,
                test_numbers: vec![17, 8, 14],
            }
            .is_valid(vec![Concatenate, Add]),
            true
        );
    }
}
