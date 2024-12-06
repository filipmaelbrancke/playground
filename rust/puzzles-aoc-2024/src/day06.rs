use crate::day06::SimulationResult::KeepLooping;
use crate::get_input_as_string;
use std::collections::HashSet;

pub fn solve() {
    let input = get_input_as_string("day06", "input");

    part_one(input.clone());
    part_two(input.clone());
}

#[derive(Debug, PartialEq, Hash, Eq, Ord, PartialOrd, Clone)]
struct Position {
    column: u32,
    row: u32,
}

impl Position {
    fn apply_movement(&self, movement: Movement) -> Position {
        Position {
            column: self.column.wrapping_add(movement.0 as u32),
            row: self.row.wrapping_add(movement.1 as u32),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Cell {
    Obstructed,
    Free,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Obstructed,
            '.' | '^' => Cell::Free,
            _ => panic!("Unexpected character in input"),
        }
    }
}

type Movement = (i32, i32);

#[derive(Debug, Clone)]
struct Map {
    cells: Vec<Vec<Cell>>,
}

impl Map {
    fn size(&self) -> usize {
        let rows = self.height();
        let columns = self.width();
        columns * rows
    }

    fn width(&self) -> usize {
        self.cells.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.cells.len()
    }

    fn is_within_bounds(&self, position: &Position) -> bool {
        position.column < self.width() as u32 && position.row < self.height() as u32
    }

    fn get_by_position(&self, position: &Position) -> Cell {
        self.cells[position.row as usize][position.column as usize].clone()
    }

    fn is_obstacle(&self, position: &Position) -> bool {
        if !self.is_within_bounds(position) {
            return false;
        }
        self.get_by_position(position) == Cell::Obstructed
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
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn movement(&self) -> Movement {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, Clone)]
struct Simulation {
    map: Map,
    position: Position,
    direction: Direction,
}

impl Simulation {
    fn new(input: String) -> Self {
        let mut start_position = Position { column: 0, row: 0 };

        let cells = input
            .lines()
            .filter(|line| !line.is_empty())
            .enumerate()
            .map(|(row_number, line)| {
                line.chars()
                    .enumerate()
                    .map(|(column_number, column_char)| {
                        if column_char == '^' {
                            start_position = Position {
                                column: column_number as u32,
                                row: row_number as u32,
                            };
                        }
                        column_char.into()
                    })
                    .collect()
            })
            .collect();
        let map = Map { cells };
        let direction = Direction::Up;
        Simulation {
            map,
            position: start_position,
            direction,
        }
    }

    fn run(&mut self, added_obstacle: Option<Position>) -> SimulationResult {
        let mut path = Vec::with_capacity(self.map.size());
        let mut paths_and_directions = HashSet::with_capacity(self.map.size());
        loop {
            if !self.map.is_within_bounds(&self.position) {
                path.sort();
                path.dedup();
                return SimulationResult::Done(path.len() as u32);
            }
            let movement = self.direction.movement();
            let potential_obstacle_x = self.position.column as i32 + movement.0;
            let potential_obstacle_y = self.position.row as i32 + movement.1;
            let potential_obstacle_position = Position {
                column: potential_obstacle_x as u32,
                row: potential_obstacle_y as u32,
            };
            if self.map.is_obstacle(&potential_obstacle_position)
                || added_obstacle == Some(potential_obstacle_position)
            {
                self.direction = self.direction.turn_right();
            } else {
                let new_position = (self.position.clone(), movement);
                if paths_and_directions.contains(&new_position) {
                    return KeepLooping;
                }
                paths_and_directions.insert(new_position);

                path.push(self.position.clone());
                let movement = self.direction.movement();
                self.position = self.position.apply_movement(movement);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum SimulationResult {
    Done(u32),
    KeepLooping,
}

fn part_one(input: String) {
    let distinct_guard_positions = calculate_distinct_guard_positions(input);
    println!("Part one: {}", distinct_guard_positions);
}

fn part_two(input: String) {
    let obstructed_positions = calculate_obstructed_positions(input);
    println!("Part two: {}", obstructed_positions);
}

fn calculate_distinct_guard_positions(input: String) -> u32 {
    let mut simulation = Simulation::new(input);
    let result = simulation.run(None);

    match result {
        SimulationResult::Done(distinct_positions) => distinct_positions,
        KeepLooping => panic!("Simulation did not finish"),
    }
}

// slowest brute-force solution
fn calculate_obstructed_positions(input: String) -> u32 {
    let simulation = Simulation::new(input);

    let positions: Vec<Position> = (0..simulation.map.size())
        .map(|i| Position {
            column: (i % simulation.map.width()) as u32,
            row: (i / simulation.map.width()) as u32,
        })
        .filter(|position| !simulation.map.is_obstacle(position))
        .filter(|position| (simulation.position != *position))
        .collect();

    positions
        .into_iter()
        .filter(|position| {
            let simulation_result = simulation.clone().run(Some(position.clone()));
            matches!(simulation_result, KeepLooping)
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use crate::day06::{
        calculate_distinct_guard_positions, calculate_obstructed_positions, Position,
    };
    use crate::get_input_as_string;

    fn get_example_input() -> String {
        String::from(
            "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        )
    }

    #[test]
    fn test_simulation_creation() {
        let simulation = super::Simulation::new(get_example_input());
        assert_eq!(simulation.map.cells.len(), 10);
        assert_eq!(simulation.map.cells.first().unwrap().len(), 10);
        assert_eq!(simulation.position, Position { column: 4, row: 6 });
        assert_eq!(simulation.direction, super::Direction::Up);
    }

    #[test]
    fn test_get_by_position() {
        let simulation = super::Simulation::new(get_example_input());
        assert_eq!(
            simulation
                .map
                .get_by_position(&Position { column: 0, row: 0 }),
            super::Cell::Free
        );
        assert_eq!(
            simulation
                .map
                .get_by_position(&Position { column: 0, row: 9 }),
            super::Cell::Free
        );
        assert_eq!(
            simulation
                .map
                .get_by_position(&Position { column: 4, row: 0 }),
            super::Cell::Obstructed
        );
        assert_eq!(
            simulation
                .map
                .get_by_position(&Position { column: 0, row: 8 }),
            super::Cell::Obstructed
        );
    }

    #[test]
    fn test_movement() {
        assert_eq!(
            Position { column: 5, row: 5 }.apply_movement(super::Direction::Up.movement()),
            Position { column: 5, row: 4 }
        );
        assert_eq!(
            Position { column: 5, row: 4 }.apply_movement(super::Direction::Down.movement()),
            Position { column: 5, row: 5 }
        );
        assert_eq!(
            Position { column: 5, row: 5 }
                .apply_movement(super::Direction::Down.movement())
                .apply_movement(super::Direction::Left.movement()),
            Position { column: 4, row: 6 }
        );
    }

    #[test]
    fn test_guards_route_from_example_should_visit_41_distinct_cells() {
        assert_eq!(calculate_distinct_guard_positions(get_example_input()), 41);
    }

    #[test]
    fn test_guards_route_solution() {
        assert_eq!(
            calculate_distinct_guard_positions(get_input_as_string("day06", "input")),
            4647
        );
    }

    #[test]
    fn test_calculate_loops_from_example_should_be_6() {
        assert_eq!(calculate_obstructed_positions(get_example_input()), 6);
    }
}
