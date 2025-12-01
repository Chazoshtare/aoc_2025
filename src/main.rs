mod day1;

use std::fs;
use std::path::Path;

fn main() {
    let day1_input = read_input(Path::new("inputs/day1.txt"));
    let solution1_1 = day1::solve_part1(&day1_input);
    println!("Day 1, part 1 solution: {}", solution1_1);
    let solution1_2 = day1::solve_part2(&day1_input);
    println!("Day 1, part 2 solution: {}", solution1_2);
}

fn read_input(path: &Path) -> String {
    fs::read_to_string(path).expect(format!("couldn't read input file {:?}", path).as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part1_returns_correct_solution() {
        let day1_input = read_input(Path::new("inputs/day1.txt"));
        let solution = day1::solve_part1(&day1_input);
        assert_eq!(solution, 984);
    }

    #[test]
    fn day1_part2_returns_correct_solution() {
        let day1_input = read_input(Path::new("inputs/day1.txt"));
        let solution = day1::solve_part2(&day1_input);
        assert_eq!(solution, 5657);
    }
}
