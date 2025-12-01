use crate::get_input_as_string;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Left(i32),
    Right(i32),
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('L') => {
                let value = s[1..].parse::<i32>()?;
                Ok(Move::Left(value))
            }
            Some('R') => {
                let value = s[1..].parse::<i32>()?;
                Ok(Move::Right(value))
            }
            _ => Err(anyhow::anyhow!("Invalid move string: {s}")),
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Left(steps) => write!(f, "L{}", steps),
            Move::Right(steps) => write!(f, "R{}", steps),
        }
    }
}

#[derive(Debug, Clone)]
struct Dial {
    current_position: i32,
    min: i32,
    max: i32,
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            current_position: 50,
            min: 0,
            max: 99,
        }
    }
}

pub fn solve() {
    let input = get_input_as_string("day01", "input");

    part_one(input.clone());
}

fn part_one(input: String) {
    let password: u32 = find_number_of_times_passing_by_zero(input);
    println!("Part one: {}", password);
}

fn find_number_of_times_passing_by_zero(input: String) -> u32 {
    let moves: Vec<Move> = input
        .lines()
        .map(|line| line.parse::<Move>().unwrap())
        .collect();

    let mut dial = Dial::default();
    let mut pass_by_zero_count = 0;

    for mv in moves {
        match mv {
            Move::Left(steps) => {
                dial.current_position -= steps % (dial.max + 1);
                if dial.current_position < dial.min {
                    dial.current_position += dial.max + 1;
                }
            }
            Move::Right(steps) => {
                dial.current_position = (dial.current_position + steps) % (dial.max + 1);
            }
        }

        if dial.current_position == dial.min {
            pass_by_zero_count += 1;
        }
    }

    pass_by_zero_count
}

#[cfg(test)]
mod tests {
    use crate::day01::Move;

    fn get_example_input() -> String {
        String::from(
            "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        )
    }

    #[test]
    fn test_move_parsing() {
        assert_eq!("L68".parse::<Move>().unwrap(), Move::Left(68));
        assert_eq!("R30".parse::<Move>().unwrap(), Move::Right(30));
    }

    #[test]
    fn test_find_number_of_times_passing_by_zero() {
        assert_eq!(
            super::find_number_of_times_passing_by_zero(get_example_input()),
            3
        );
    }
}
