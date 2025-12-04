mod day1;
mod day2;
mod day3;
mod day4;
mod grid;

use std::fs;
use std::path::Path;

fn main() {
    let day1_input = read_input(Path::new("inputs/day1.txt"));
    let solution1_1 = day1::solve_part1(&day1_input);
    println!("Day 1, part 1 solution: {solution1_1}");
    let solution1_2 = day1::solve_part2(&day1_input);
    println!("Day 1, part 2 solution: {solution1_2}");

    let day2_input = read_input(Path::new("inputs/day2.txt"));
    let solution2_1 = day2::solve_part1(&day2_input);
    println!("Day 2, part 1 solution: {solution2_1}");
    let solution2_2 = day2::solve_part2(&day2_input);
    println!("Day 2, part 2 solution: {solution2_2}");

    let day3_input = read_input(Path::new("inputs/day3.txt"));
    let solution3_1 = day3::solve_part1(&day3_input);
    println!("Day 3, part 1 solution: {solution3_1}");
    let solution3_2 = day3::solve_part2(&day3_input);
    println!("Day 3, part 2 solution: {solution3_2}");

    let day4_input = read_input(Path::new("inputs/day4.txt"));
    let solution4_1 = day4::solve_part1(&day4_input);
    println!("Day 4, part 1 solution: {solution4_1}");
    let solution4_2 = day4::solve_part2(&day4_input);
    println!("Day 4, part 2 solution: {solution4_2}");
}

fn read_input(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("couldn't read input file {path:?}"))
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

    #[test]
    fn day2_part1_returns_correct_solution() {
        let day2_input = read_input(Path::new("inputs/day2.txt"));
        let solution = day2::solve_part1(&day2_input);
        assert_eq!(solution, 31000881061);
    }

    #[test]
    fn day2_part2_returns_correct_solution() {
        let day2_input = read_input(Path::new("inputs/day2.txt"));
        let solution = day2::solve_part2(&day2_input);
        assert_eq!(solution, 46769308485);
    }

    #[test]
    fn day3_part1_returns_correct_solution() {
        let day3_input = read_input(Path::new("inputs/day3.txt"));
        let solution = day3::solve_part1(&day3_input);
        assert_eq!(solution, 17301);
    }

    #[test]
    fn day3_part2_returns_correct_solution() {
        let day3_input = read_input(Path::new("inputs/day3.txt"));
        let solution = day3::solve_part2(&day3_input);
        assert_eq!(solution, 172162399742349);
    }

    #[test]
    fn day4_part1_returns_correct_solution() {
        let day4_input = read_input(Path::new("inputs/day4.txt"));
        let solution = day4::solve_part1(&day4_input);
        assert_eq!(solution, 1397);
    }

    #[test]
    fn day4_part2_returns_correct_solution() {
        let day4_input = read_input(Path::new("inputs/day4.txt"));
        let solution = day4::solve_part2(&day4_input);
        assert_eq!(solution, 8758);
    }
}
