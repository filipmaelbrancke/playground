use crate::get_input_as_string;
use std::collections::{HashSet, VecDeque};
use Direction::{Down, Left, Right, Up};

pub fn solve() {
    let input = get_input_as_string("day12", "input");

    part_one(input.clone());
    part_two(input.clone());
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Map {
    map: Vec<Vec<char>>,
}

impl From<String> for Map {
    fn from(input: String) -> Self {
        Map {
            map: input
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| line.chars().collect())
                .collect(),
        }
    }
}

impl Map {
    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn get_by_position(&self, column: i32, row: i32) -> char {
        self.map[row as usize][column as usize]
    }

    fn is_within_bounds(&self, column: i32, row: i32) -> bool {
        column >= 0 && row >= 0 && column < self.width() as i32 && row < self.height() as i32
    }

    fn solve(&self) -> Vec<CalculationResult> {
        let mut visited_plots = HashSet::new();
        let mut results = Vec::new();

        for row in 0..self.height() {
            for column in 0..self.width() {
                // not yet processed plots
                if !visited_plots.contains(&(row, column)) {
                    let mut region = HashSet::new();
                    let mut queue = VecDeque::new();
                    let target = self.get_by_position(column as i32, row as i32);

                    queue.push_back((row, column));
                    region.insert((row, column));
                    visited_plots.insert((row, column));

                    while let Some((current_row, current_column)) = queue.pop_front() {
                        for direction in [Up, Down, Left, Right] {
                            let (delta_row, delta_column) = direction.delta();
                            let new_row = current_row.wrapping_add(delta_row as usize);
                            let new_column = current_column.wrapping_add(delta_column as usize);

                            if new_row < self.height()
                                && new_column < self.width()
                                && !visited_plots.contains(&(new_row, new_column))
                                && self.get_by_position(new_column as i32, new_row as i32) == target
                            {
                                queue.push_back((new_row, new_column));
                                region.insert((new_row, new_column));
                                visited_plots.insert((new_row, new_column));
                            }
                        }
                    }

                    let area = region.len();
                    let mut perimeter = 0;
                    let mut sides = 0;

                    for &(row, column) in &region {
                        // perimeter
                        for direction in [Up, Down, Left, Right] {
                            let (delta_row, delta_column) = direction.delta();
                            if !self.equals_target(
                                row as i32 + delta_row,
                                column as i32 + delta_column,
                                target,
                            ) {
                                perimeter += 1;
                            }
                        }
                        let test_position = Position {
                            row: row as i32,
                            column: column as i32,
                        };
                        // outer sides
                        if !self.equals_target_position(test_position.move_in_direction(Up), target)
                            && !self.equals_target_position(
                                test_position.move_in_direction(Left),
                                target,
                            )
                        {
                            sides += 1;
                        }
                        if !self.equals_target_position(test_position.move_in_direction(Up), target)
                            && !self.equals_target_position(
                                test_position.move_in_direction(Right),
                                target,
                            )
                        {
                            sides += 1;
                        }
                        if !self
                            .equals_target_position(test_position.move_in_direction(Down), target)
                            && !self.equals_target_position(
                                test_position.move_in_direction(Left),
                                target,
                            )
                        {
                            sides += 1;
                        }
                        if !self
                            .equals_target_position(test_position.move_in_direction(Down), target)
                            && !self.equals_target_position(
                                test_position.move_in_direction(Right),
                                target,
                            )
                        {
                            sides += 1;
                        }
                        // inner sides
                        if self.equals_target_position(test_position.move_in_direction(Up), target)
                            && self.equals_target_position(
                                test_position.move_in_direction(Left),
                                target,
                            )
                            && !self.equals_target_position(
                                test_position.move_in_direction(Up).move_in_direction(Left),
                                target,
                            )
                        {
                            sides += 1;
                        }
                        if self.equals_target_position(test_position.move_in_direction(Up), target)
                            && self.equals_target_position(
                                test_position.move_in_direction(Right),
                                target,
                            )
                            && !self.equals_target_position(
                                test_position.move_in_direction(Up).move_in_direction(Right),
                                target,
                            )
                        {
                            sides += 1;
                        }
                        if self
                            .equals_target_position(test_position.move_in_direction(Down), target)
                            && self.equals_target_position(
                                test_position.move_in_direction(Left),
                                target,
                            )
                            && !self.equals_target_position(
                                test_position
                                    .move_in_direction(Down)
                                    .move_in_direction(Left),
                                target,
                            )
                        {
                            sides += 1;
                        }
                        if self
                            .equals_target_position(test_position.move_in_direction(Down), target)
                            && self.equals_target_position(
                                test_position.move_in_direction(Right),
                                target,
                            )
                            && !self.equals_target_position(
                                test_position
                                    .move_in_direction(Down)
                                    .move_in_direction(Right),
                                target,
                            )
                        {
                            sides += 1;
                        }
                    }

                    results.push(CalculationResult {
                        area,
                        perimeter,
                        sides,
                    });
                }
            }
        }

        results
    }

    fn equals_target(&self, row: i32, column: i32, target: char) -> bool {
        self.is_within_bounds(column, row) && self.get_by_position(column, row) == target
    }

    fn equals_target_position(&self, position: Position, target: char) -> bool {
        self.equals_target(position.row, position.column, target)
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            // row - column
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
        }
    }

    fn apply_to(&self, row: i32, column: i32) -> (i32, i32) {
        let (delta_row, delta_column) = self.delta();
        (
            row.wrapping_add(delta_row),
            column.wrapping_add(delta_column),
        )
    }
}

struct Position {
    column: i32,
    row: i32,
}

impl Position {
    fn move_in_direction(&self, direction: Direction) -> Position {
        let (row, column) = direction.apply_to(self.row, self.column);
        Position { column, row }
    }
}

struct CalculationResult {
    area: usize,
    perimeter: usize,
    sides: usize,
}

fn part_one(input: String) {
    let part_one = run_part_one_logic(input);
    println!("Part one: {}", part_one);
}

fn part_two(input: String) {
    let part_two = run_part_two_logic(input);
    println!("Part two: {}", part_two);
}

fn run_part_one_logic(input: String) -> usize {
    Map::from(input)
        .solve()
        .iter()
        .map(|result| result.area * result.perimeter)
        .sum()
}

fn run_part_two_logic(input: String) -> usize {
    Map::from(input)
        .solve()
        .iter()
        .map(|result| result.area * result.sides)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day12::{run_part_one_logic, run_part_two_logic, Direction, Position};
    use Direction::{Down, Right, Up};

    fn get_example_input_1() -> String {
        String::from(
            "\
AAAA
BBCD
BBCC
EEEC",
        )
    }

    fn get_example_input_2() -> String {
        String::from(
            "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
        )
    }

    fn get_example_input_3() -> String {
        String::from(
            "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
        )
    }

    fn get_example_input_4() -> String {
        String::from(
            "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
        )
    }

    fn get_example_input_5() -> String {
        String::from(
            "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
        )
    }

    #[test]
    fn test_position() {
        let new_position = Position { row: 5, column: 5 }.move_in_direction(Right);
        assert_eq!((new_position.row, new_position.column), (5, 6));
        let new_position = Position { row: 5, column: 5 }.move_in_direction(Down);
        assert_eq!((new_position.row, new_position.column), (6, 5));
        let new_position = new_position.move_in_direction(Up);
        assert_eq!((new_position.row, new_position.column), (5, 5));
    }

    #[test]
    fn test_part_one_from_example() {
        assert_eq!(run_part_one_logic(get_example_input_1()), 140);
        assert_eq!(run_part_one_logic(get_example_input_2()), 772);
        assert_eq!(run_part_one_logic(get_example_input_3()), 1930);
    }

    #[test]
    fn test_part_two_from_example() {
        assert_eq!(run_part_two_logic(get_example_input_1()), 80);
        assert_eq!(run_part_two_logic(get_example_input_2()), 436);
        assert_eq!(run_part_two_logic(get_example_input_3()), 1206);
        assert_eq!(run_part_two_logic(get_example_input_4()), 236);
        assert_eq!(run_part_two_logic(get_example_input_5()), 368);
    }

    #[test]
    fn test_part_one_solution() {
        let input = super::get_input_as_string("day12", "input");
        assert_eq!(run_part_one_logic(input), 1546338);
    }

    #[test]
    fn test_part_two_solution() {
        let input = super::get_input_as_string("day12", "input");
        assert_eq!(run_part_two_logic(input), 978590);
    }
}
