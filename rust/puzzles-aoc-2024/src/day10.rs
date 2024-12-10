use crate::get_input_as_string;
use std::collections::{HashSet, VecDeque};

pub fn solve() {
    let input = get_input_as_string("day10", "input");

    part_one(input.clone());
    part_two(input.clone());
}

type Movement = (i32, i32);

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Map {
    map: Vec<Vec<usize>>,
}

impl Map {
    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn get_by_position(&self, x: i32, y: i32) -> usize {
        self.map[y as usize][x as usize]
    }

    fn is_within_bounds(&self, position: &Position) -> bool {
        position.column >= 0
            && position.row >= 0
            && position.column < self.width() as i32
            && position.row < self.height() as i32
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct HikingTrailMap {
    map: Map,
    trailhead_positions: Vec<Position>,
}

impl HikingTrailMap {
    fn parse(input: &str) -> Self {
        let hiking_trails: Vec<Vec<usize>> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect();
        let map = Map { map: hiking_trails };
        let mut trailheads = Vec::new();
        for y in 0..map.height() {
            for x in 0..map.width() {
                if map.get_by_position(x as i32, y as i32) == 0 {
                    trailheads.push(Position {
                        column: x as i32,
                        row: y as i32,
                        value: 0,
                    });
                }
            }
        }
        Self {
            map,
            trailhead_positions: trailheads,
        }
    }

    fn calculate_trailhead_scores(&self) -> CalculationResult {
        let mut total_trailheads_score = 0;
        let mut total_ratings = 0;
        for trailhead in &self.trailhead_positions {
            let mut trailhead_positions = VecDeque::new();
            trailhead_positions.push_back(trailhead.clone());
            let mut hiking_trail_paths = HashSet::new();
            let mut rating = 0;
            while let Some(position) = trailhead_positions.pop_front() {
                if position.value == 9 {
                    hiking_trail_paths.insert(position);
                    rating += 1;
                    continue;
                }
                for direction in [
                    Direction::Right,
                    Direction::Left,
                    Direction::Down,
                    Direction::Up,
                ] {
                    let new_position = position.move_in_direction(direction);
                    if self.map.is_within_bounds(&new_position) {
                        let neighboring_value = self
                            .map
                            .get_by_position(new_position.column, new_position.row);
                        if neighboring_value == position.value + 1 {
                            trailhead_positions.push_back(Position {
                                column: new_position.column,
                                row: new_position.row,
                                value: neighboring_value,
                            });
                        }
                    }
                }
            }
            total_trailheads_score += hiking_trail_paths.len();
            total_ratings += rating;
        }

        CalculationResult {
            total_trailheads_score,
            total_ratings,
        }
    }
}

struct CalculationResult {
    total_trailheads_score: usize,
    total_ratings: usize,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn movement(&self) -> Movement {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Ord, PartialOrd, Clone)]
pub struct Position {
    column: i32,
    row: i32,
    value: usize,
}

impl Position {
    fn move_in_direction(&self, direction: Direction) -> Position {
        let (dx, dy) = direction.movement();
        Position {
            column: self.column + dx,
            row: self.row + dy,
            value: self.value,
        }
    }
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
    HikingTrailMap::parse(&input)
        .calculate_trailhead_scores()
        .total_trailheads_score
}

fn run_part_two_logic(input: String) -> usize {
    HikingTrailMap::parse(&input)
        .calculate_trailhead_scores()
        .total_ratings
}

#[cfg(test)]
mod tests {
    use crate::day10::{run_part_one_logic, run_part_two_logic};

    fn get_example_input() -> String {
        String::from(
            "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        )
    }

    #[test]
    fn test_input_parsing() {
        let input = get_example_input();
        let grid = super::HikingTrailMap::parse(&input);
        assert_eq!(grid.map.width(), 8);
        assert_eq!(grid.map.height(), 8);
        assert_eq!(grid.trailhead_positions.len(), 9);
    }

    #[test]
    fn test_part_one_from_example() {
        assert_eq!(run_part_one_logic(get_example_input()), 36);
    }

    #[test]
    fn test_part_two_from_example() {
        assert_eq!(run_part_two_logic(get_example_input()), 81);
    }

    #[test]
    fn test_part_one_solution() {
        let input = super::get_input_as_string("day10", "input");
        assert_eq!(run_part_one_logic(input), 566);
    }

    #[test]
    fn test_part_two_solution() {
        let input = super::get_input_as_string("day10", "input");
        assert_eq!(run_part_two_logic(input), 1324);
    }
}
