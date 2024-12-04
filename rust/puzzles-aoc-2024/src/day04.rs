use crate::get_input_as_string;
use gridly::prelude::{Grid, Left, Right, Up};
use gridly::vector::TOUCHING_ADJACENCIES;
use gridly_grids::VecGrid;

pub fn solve() {
    let input = get_input_as_string("day04", "input");

    part_one(input.clone());
    part_two(input.clone());
}

fn part_one(input: String) {
    let xmas_occurences = find_number_of_xmas_occurrences(input);
    println!("Part one: {}", xmas_occurences);
}

fn part_two(input: String) {
    let x_mas_occurences = find_number_of_x_mas_occurences(input);
    println!("Part two: {}", x_mas_occurences);
}

fn find_number_of_xmas_occurrences(input: String) -> u32 {
    let grid = convert_input_to_xmas_grid(input).unwrap().grid;
    grid.rows()
        .iter()
        .flat_map(|row| row.iter_with_locations())
        .flat_map(|(location, _current_position)| {
            TOUCHING_ADJACENCIES
                .iter()
                .map(move |&direction| (location, direction))
        })
        .filter(|&(location, direction)| {
            "XMAS".chars().enumerate().all(|(index, character)| {
                grid.get(location + direction * index as isize)
                    .map(|&c| c == character)
                    .unwrap_or(false)
            })
        })
        .count() as u32
}

fn find_number_of_x_mas_occurences(input: String) -> u32 {
    let grid = convert_input_to_xmas_grid(input).unwrap().grid;
    grid
        .rows()
        .iter()
        .flat_map(|row| row.iter_with_locations())
        .filter(|&(location, _character)| {

            // location is an 'A'
            (grid.get(location).copied().ok() == Some('A'))
                // neighboring locations form two MAS in the shape of an X
                // = the locations above and below it contain M and S in any order
                && [Left, Right].iter().all(|&direction| {
                    let location_shift = Up + direction;
                    let left_or_right_neighbor_above = grid.get(location + location_shift).copied();
                    let left_or_right_neighbor_below = grid.get(location - location_shift).copied();
                    matches!((left_or_right_neighbor_above, left_or_right_neighbor_below), (Ok('M'), Ok('S')) | (Ok('S'), Ok('M')))
                })
        })
        .count() as u32
}

fn convert_input_to_xmas_grid(input: String) -> Option<XmasGrid> {
    VecGrid::new_from_rows(input.lines().map(|line| line.chars())).map(|grid| XmasGrid { grid })
}

#[derive(Debug)]
struct XmasGrid {
    grid: VecGrid<char>,
}

#[cfg(test)]
mod tests {
    use crate::day04::{convert_input_to_xmas_grid, XmasGrid};
    use gridly::grid::GridBounds;

    fn get_example_input() -> String {
        String::from(
            "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        )
    }

    #[test]
    fn test_convert_input_to_xmas_grdi() {
        if let Some(XmasGrid { grid }) = convert_input_to_xmas_grid(get_example_input()) {
            assert_eq!(grid.dimensions(), (10, 10));
        }
    }

    #[test]
    fn test_find_number_of_xmas_occurrences_in_example_should_be_18() {
        let input = get_example_input();
        assert_eq!(super::find_number_of_xmas_occurrences(input), 18);
    }

    #[test]
    fn test_find_number_of_x_mas_occurrences_in_example_should_be_18() {
        let input = get_example_input();
        assert_eq!(super::find_number_of_x_mas_occurences(input), 9);
    }
}
