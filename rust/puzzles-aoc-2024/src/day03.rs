use std::sync::LazyLock;
use regex::Regex;
use crate::get_input_as_string;

// instruction = like mul(X,Y), where X and Y are each 1-3 digit numbers.
static MUL_INSTRUCTION_REGEX: LazyLock<Regex> = LazyLock::new(||
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Unable to compile regex"));

static DO_OR_DONT_MUL_INSTRUCTION_REGEX: LazyLock<Regex> = LazyLock::new(||
    Regex::new(r"(?s)don't\(\).*?do\(\)").expect("Unable to compile regex"));

pub fn solve() {
    let input = get_input_as_string("day03", "input");

    part_one(input.clone());
    part_two(input.clone());
}

fn part_one(input: String) {
    let multiplications_sum = find_multiplications_sum(input);
    println!("Part one: {}", multiplications_sum);
}

fn part_two(input: String) {
    let filtered_multiplications_sum = find_filtered_multiplications_sum(input);
    println!("Part two: {}", filtered_multiplications_sum);
}

fn find_multiplications_sum(input: String) -> u32 {
    find_mul_instructions(&input)
        .iter()
        .map(|instruction| instruction.run())
        .sum()
}

fn find_filtered_multiplications_sum(input: String) -> u32 {
    // filter out the don't do mul instructions parts from the input, and run the mul instructions
    let filtered_input = remove_disabled_mul_instructions(&input);
    find_multiplications_sum(filtered_input.to_string())
}

fn remove_disabled_mul_instructions(input: &str) -> String {
    // Only the most recent do() or don't() instruction applies. At the beginning of the program, mul instructions are enabled.
    DO_OR_DONT_MUL_INSTRUCTION_REGEX.replace_all(input, "").into_owned()
}

fn find_mul_instructions(input: &str) -> Vec<MulInstruction> {
    MUL_INSTRUCTION_REGEX.captures_iter(input)
        .map(|capture| {
            MulInstruction {
                x: capture[1].parse::<u32>().expect("Unable to parse x int as u32"),
                y: capture[2].parse::<u32>().expect("Unable to parse y int as u32"),
            }
        })
        .collect()
}

#[derive(Debug, PartialEq)]
struct MulInstruction {
    x: u32,
    y: u32,
}

impl MulInstruction {
    fn run(&self) -> u32 {
        self.x * self.y
    }
}

#[cfg(test)]
mod tests {
    use crate::day03::MUL_INSTRUCTION_REGEX;

    fn get_example_input() -> String {
        String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
    }

    fn get_example_input_2() -> String {
        String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
    }

    #[test]
    fn test_part_one_regex() {
        assert_eq!(MUL_INSTRUCTION_REGEX.is_match("mul(2,4)"), true);
        assert_eq!(MUL_INSTRUCTION_REGEX.is_match("mul(5,5)"), true);
        assert_eq!(MUL_INSTRUCTION_REGEX.is_match("mul(11,8)"), true);
        assert_eq!(MUL_INSTRUCTION_REGEX.is_match("mul(8,5)"), true);
        assert_eq!(MUL_INSTRUCTION_REGEX.is_match("mul(8,1234)"), false);
        assert_eq!(MUL_INSTRUCTION_REGEX.is_match("mul[3,7]"), false);
        assert_eq!(MUL_INSTRUCTION_REGEX.is_match("mul(32,64]"), false);
    }
    
    #[test]
    fn test_find_mul_instructions() {
        let input = get_example_input();
        let expected = vec![
            super::MulInstruction { x: 2, y: 4 },
            super::MulInstruction { x: 5, y: 5 },
            super::MulInstruction { x: 11, y: 8 },
            super::MulInstruction { x: 8, y: 5 },
        ];
        assert_eq!(super::find_mul_instructions(&input), expected);
    }
    
    #[test]
    fn test_mul_instruction_run() {
        let instruction = super::MulInstruction { x: 2, y: 4 };
        assert_eq!(instruction.run(), 8);
        let instruction = super::MulInstruction { x: 5, y: 5 };
        assert_eq!(instruction.run(), 25);
        let instruction = super::MulInstruction { x: 11, y: 8 };
        assert_eq!(instruction.run(), 88);
        let instruction = super::MulInstruction { x: 8, y: 5 };
        assert_eq!(instruction.run(), 40);
    }
    
    #[test]
    fn test_find_multiplications_sum() {
        let input = get_example_input();
        assert_eq!(super::find_multiplications_sum(input), 161);
    }

    #[test]
    fn test_filter_out_disabled_sections() {
        let input = get_example_input_2();
        let expected = "xmul(2,4)&mul[3,7]!^?mul(8,5))";
        assert_eq!(super::remove_disabled_mul_instructions(&input), expected);
    }

    #[test]
    fn test_find_filtered_multiplications_sum() {
        let input = get_example_input_2();
        assert_eq!(super::find_filtered_multiplications_sum(input), 48);
    }
}