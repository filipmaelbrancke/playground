use crate::get_input_as_string;
use scan_fmt::scan_fmt;

#[derive(Clone)]
struct Game {
    number_of_red: u32,
    number_of_green: u32,
    number_of_blue: u32,
}

impl Game {
    fn new(number_of_red: u32, number_of_green: u32, number_of_blue: u32) -> Game {
        Game {
            number_of_red,
            number_of_green,
            number_of_blue,
        }
    }

    fn add_number_of_cubes(self, red: u32, green: u32, blue: u32) -> Game {
        Game {
            number_of_red: self.number_of_red + red,
            number_of_green: self.number_of_green + green,
            number_of_blue: self.number_of_blue + blue,
        }
    }

    fn multiply_all_cubes(&self) -> u32 {
        self.number_of_red * self.number_of_green * self.number_of_blue
    }
}

pub fn solve() {
    let input = get_input_as_string("day02", "input");
    let assignment: Game = Game::new(12, 13, 14);
    let result_part1: u32 = input
        .lines()
        .map(parse_line())
        .map(|(game_id, game)| (game_id, parse_game()(game.as_str())))
        .filter(|(_game_id, sub_games)| {
            sub_games.iter().all(|sub_game| {
                sub_game.number_of_red <= assignment.number_of_red
                    && sub_game.number_of_green <= assignment.number_of_green
                    && sub_game.number_of_blue <= assignment.number_of_blue
            })
        })
        .map(|(game_id, _)| game_id)
        .sum();

    let result_part2: u32 = input
        .lines()
        .map(parse_line())
        .map(|(_, game)| parse_game()(game.as_str()))
        .map(|sub_games| {
            sub_games
                .iter()
                .cloned()
                .reduce(|sub_game1, sub_game2| {
                    Game {
                        number_of_red: sub_game1.number_of_red.max(sub_game2.number_of_red),
                        number_of_green: sub_game1.number_of_green.max(sub_game2.number_of_green),
                        number_of_blue: sub_game1.number_of_blue.max(sub_game2.number_of_blue),
                    }
                })
        })
        .map(|game| game.unwrap().multiply_all_cubes())
        .sum();

    println!("Result part 1: {result_part1}");
    println!("Result part 2: {result_part2}");
}

fn parse_line() -> fn(&str) -> (u32, String) {
    |line| {
        let (game_id, game) = scan_fmt!(line, "Game {d}: {/.*$/}", u32, String).unwrap();
        (game_id, game)
    }
}

fn parse_game() -> fn(&str) -> Vec<Game> {
    |game_input| {
        game_input
            .split("; ")
            .map(|sub_game| {
                sub_game
                    .split(", ")
                    .fold(Game::new(0, 0, 0), |sub_game, sub_game_input| {
                        let (amount, color) =
                            scan_fmt!(sub_game_input, "{} {}", u32, String).unwrap();

                        match color.as_str() {
                            "red" => sub_game.add_number_of_cubes(amount, 0, 0),
                            "green" => sub_game.add_number_of_cubes(0, amount, 0),
                            "blue" => sub_game.add_number_of_cubes(0, 0, amount),
                            _ => sub_game.add_number_of_cubes(0, 0, 0),
                        }
                    })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::day02::Game;
    use scan_fmt::scan_fmt;

    #[test]
    fn test_parse_line() {
        let line = "Game 1: 7 red, 8 blue; 6 blue, 6 red, 2 green; 2 red, 6 green, 8 blue; 9 green, 2 red, 4 blue; 6 blue, 4 green";
        let (game_id, game) = scan_fmt!(line, "Game {d}: {/.*$/}", u32, String).unwrap();
        assert_eq!(game_id, 1);
        assert_eq!(game, "7 red, 8 blue; 6 blue, 6 red, 2 green; 2 red, 6 green, 8 blue; 9 green, 2 red, 4 blue; 6 blue, 4 green");
    }

    #[test]
    fn test_parse_game_line() {
        let line = "7 red, 8 blue; 6 blue, 6 red, 2 green; 2 red, 6 green, 8 blue; 9 green, 2 red, 4 blue; 6 blue, 4 green";
        let parse_game_function: fn(&str) -> Vec<Game> = super::parse_game();
        assert_eq!(parse_game_function(line).len(), 5)
    }
}
