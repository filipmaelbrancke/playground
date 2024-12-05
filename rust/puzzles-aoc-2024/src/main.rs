fn main() {
    println!("Advent of Code 2024");

    let solutions: Vec<fn()> = vec![
        puzzles_aoc_2024::day01::solve,
        puzzles_aoc_2024::day02::solve,
        puzzles_aoc_2024::day03::solve,
        puzzles_aoc_2024::day04::solve,
        puzzles_aoc_2024::day05::solve,
    ];

    for (i, solve) in solutions.iter().enumerate() {
        println!("Solutions for day {}::", i + 1);
        solve();
    }
}
