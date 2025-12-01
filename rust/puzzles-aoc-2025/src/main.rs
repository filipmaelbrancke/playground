use puzzles_aoc_2025::day01;

fn main() {
    println!("Advent of Code 2025");

    let solutions: Vec<fn()> = vec![day01::solve];

    for (i, solve) in solutions.iter().enumerate() {
        println!("Solutions for day {}::", i + 1);
        solve();
    }
}
