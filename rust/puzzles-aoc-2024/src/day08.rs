use crate::get_input_as_string;
use itertools::Itertools;

pub fn solve() {
    let input = get_input_as_string("day08", "input");

    part_one(input.clone());
    part_two(input.clone());
}

#[derive(Debug, PartialEq, Hash, Eq, Ord, PartialOrd, Clone)]
struct Grid {
    pub antennas: Vec<Antenna>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut grid = Grid {
            antennas: Vec::new(),
            width: 0,
            height: 0,
        };
        for (i, line) in input.lines().filter(|line| !line.is_empty()).enumerate() {
            grid.height = i + 1;
            for (j, c) in line.chars().enumerate() {
                grid.width = j + 1;
                if c != '.' {
                    grid.antennas.push(Antenna {
                        position: Position { column: j, row: i },
                        frequency: c,
                    });
                }
            }
        }
        grid
    }

    fn antinodes(&self, model_resonance: bool) -> Vec<Position> {
        self.antennas
            .iter()
            .permutations(2)
            .filter(|antenna_pair| antenna_pair[0].frequency == antenna_pair[1].frequency)
            .flat_map(|antenna_pair| {
                antenna_pair[0].get_antinodes_for_antenna(antenna_pair[1], self, model_resonance)
            })
            .filter(|position| self.is_within_bounds(position))
            .unique()
            .collect()
    }

    fn is_within_bounds(&self, position: &Position) -> bool {
        position.column < self.width && position.row < self.height
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Ord, PartialOrd, Clone)]
pub struct Antenna {
    pub position: Position,
    pub frequency: char,
}

impl Antenna {
    fn get_antinodes_for_antenna(
        &self,
        other_antenna: &Antenna,
        grid: &Grid,
        model_resonance: bool,
    ) -> Vec<Position> {
        let delta = self.position.delta(&other_antenna.position);
        let mut antinodes = vec![self.position.translate(&delta)];
        if model_resonance {
            antinodes.extend([self.position.clone(), other_antenna.position.clone()]);
            let mut position = antinodes[0].clone();
            while grid.is_within_bounds(&position) {
                position = position.translate(&delta);
                antinodes.push(position.clone());
            }
        }
        antinodes
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Ord, PartialOrd, Clone)]
pub struct Position {
    column: usize,
    row: usize,
}

impl Position {
    fn delta(&self, other: &Position) -> Position {
        Position {
            column: other.column.wrapping_sub(self.column),
            row: other.row.wrapping_sub(self.row),
        }
    }

    fn translate(&self, other: &Position) -> Position {
        Position {
            column: self.column.wrapping_sub(other.column),
            row: self.row.wrapping_sub(other.row),
        }
    }
}

fn part_one(input: String) {
    let unique_locations_antinode = unique_locations_containing_antinode(input);
    println!("Part one: {}", unique_locations_antinode);
}

fn part_two(input: String) {
    let antinodes = unique_locations_containing_antinode_updated_model(input);
    println!("Part two: {}", antinodes);
}

fn unique_locations_containing_antinode(input: String) -> usize {
    let grid = Grid::new(&input);
    grid.antinodes(false).len()
}

fn unique_locations_containing_antinode_updated_model(input: String) -> usize {
    let grid = Grid::new(&input);
    grid.antinodes(true).len()
}

#[cfg(test)]
mod tests {
    use crate::day08::{
        unique_locations_containing_antinode, unique_locations_containing_antinode_updated_model,
        Grid,
    };
    use crate::get_input_as_string;

    fn get_example_input() -> String {
        String::from(
            "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        )
    }

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(&get_example_input());
        assert_eq!(grid.width, 12);
        assert_eq!(grid.height, 12);
        assert_eq!(grid.antennas.len(), 7);
    }

    #[test]
    fn test_unique_locations_containing_antinode_from_example_be_14() {
        assert_eq!(
            unique_locations_containing_antinode(get_example_input()),
            14
        );
    }

    #[test]
    fn test_unique_locations_containing_antinode_updated_model_from_example_be_34() {
        assert_eq!(
            unique_locations_containing_antinode_updated_model(get_example_input()),
            34
        );
    }

    #[test]
    fn test_solution() {
        let input = get_input_as_string("day08", "input");
        assert_eq!(unique_locations_containing_antinode(input.clone()), 320);
        assert_eq!(
            unique_locations_containing_antinode_updated_model(input.clone()),
            1157
        );
    }
}
