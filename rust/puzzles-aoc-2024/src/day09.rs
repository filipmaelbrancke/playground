use crate::day09::Block::{Empty, File};
use crate::get_input_as_string;

pub fn solve() {
    let input = get_input_as_string("day09", "input");

    part_one(input.clone());
    part_two(input.clone());
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
enum Block {
    File { file_id: usize },
    Empty,
}

impl Block {
    fn is_file(&self) -> bool {
        match self {
            Block::File { .. } => true,
            Block::Empty => false,
        }
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct DiskMap {
    blocks: Vec<Block>,
}

impl DiskMap {
    fn from_input(input: &str) -> Self {
        let mut disk_map: Vec<Block> = Vec::new();
        let mut is_file = true;
        let mut file_id = 0;

        input.chars().for_each(|c| {
            let next = c.to_digit(10).unwrap() as usize;
            match is_file {
                true => {
                    disk_map.extend(vec![File { file_id }; next]);
                    file_id += 1;
                }
                false => {
                    disk_map.extend(vec![Block::Empty; next]);
                }
            }
            is_file = !is_file;
        });

        DiskMap { blocks: disk_map }
    }

    fn compact(&mut self) -> Self {
        let mut loop_index = 0;
        let mut end_block_to_move = self.blocks.len() - 1;

        loop {
            loop {
                if self.blocks[loop_index] != Empty && loop_index < self.blocks.len() {
                    loop_index += 1;
                } else {
                    break;
                }
            }
            // loop_index is first currently free position
            loop {
                if self.blocks[end_block_to_move] == Empty {
                    end_block_to_move -= 1;
                } else {
                    break;
                }
            }
            // end_block_to_move is first non-empty position from the end
            if loop_index >= end_block_to_move {
                break;
            }
            self.blocks.swap(loop_index, end_block_to_move);
        }
        DiskMap {
            blocks: self.blocks.clone(),
        }
    }

    fn compact_full_files(&mut self) -> Self {
        let mut loop_index;
        let mut from_end_index = self.blocks.len() - 1;
        let mut file_size;
        let mut empty_blocks_available;

        loop {
            loop_index = 0;

            // check where last block sits
            loop {
                if self.blocks[from_end_index] == Empty {
                    from_end_index -= 1;
                } else {
                    break;
                }
            }
            file_size = 1;
            loop {
                if file_size <= from_end_index
                    && self.blocks[from_end_index - file_size] == self.blocks[from_end_index]
                {
                    file_size += 1;
                } else {
                    break;
                }
            }
            // where to insert the file?
            loop {
                loop {
                    if loop_index < self.blocks.len() && self.blocks[loop_index] != Empty {
                        loop_index += 1;
                    } else {
                        break;
                    }
                }
                empty_blocks_available = 1;
                loop {
                    if loop_index + empty_blocks_available < self.blocks.len()
                        && self.blocks[loop_index + empty_blocks_available] == Empty
                    {
                        empty_blocks_available += 1;
                    } else {
                        break;
                    }
                }
                if empty_blocks_available >= file_size {
                    break;
                } else if loop_index >= self.blocks.len() {
                    from_end_index -= file_size;
                    file_size = 0;
                    break;
                } else {
                    loop_index += empty_blocks_available;
                }
            }
            if from_end_index < loop_index {
                if file_size > from_end_index {
                    break;
                }
                from_end_index -= file_size;
                continue;
            }
            // swap blocks for file
            for i in 0..file_size {
                self.blocks.swap(loop_index + i, from_end_index - i);
            }
        }
        DiskMap {
            blocks: self.blocks.clone(),
        }
    }

    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .filter(|(_, block)| block.is_file())
            .map(|(i, block)| match block {
                Block::File { file_id } => (i, file_id),
                Block::Empty => panic!("Empty block found past filtering"),
            })
            .fold(0, |acc, (index, file)| acc + index * file)
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
    DiskMap::from_input(&input).compact().checksum()
}

fn run_part_two_logic(input: String) -> usize {
    DiskMap::from_input(&input).compact_full_files().checksum()
}

#[cfg(test)]
mod tests {
    use crate::day09::{run_part_one_logic, run_part_two_logic, DiskMap};

    fn get_example_input() -> String {
        String::from("2333133121414131402")
    }

    #[test]
    fn test_input_parsing() {
        let input = get_example_input();
        let disk_map = DiskMap::from_input(&input);
        assert_eq!(disk_map.blocks.len(), 42);
    }

    #[test]
    fn test_part_one_from_example_should_be_1928() {
        assert_eq!(run_part_one_logic(get_example_input()), 1928);
    }

    #[test]
    fn test_part_two_from_example_should_be_2858() {
        assert_eq!(run_part_two_logic(get_example_input()), 2858);
    }

    #[test]
    fn test_part_one_solution() {
        let input = super::get_input_as_string("day09", "input");
        assert_eq!(run_part_one_logic(input), 6370402949053);
    }

    /*#[test]
    fn test_part_two_solution() {
        let input = super::get_input_as_string("day09", "input");
        assert_eq!(run_part_two_logic(input), 6398096697992);
    }*/
}
