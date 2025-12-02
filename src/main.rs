mod day1;
mod day2;

use std::fs;
use std::path::Path;

fn main() {
    let day1_input = read_input(Path::new("inputs/day1.txt"));
    let solution1_1 = day1::solve_part1(&day1_input);
    println!("Day 1, part 1 solution: {}", solution1_1);
    let solution1_2 = day1::solve_part2(&day1_input);
    println!("Day 1, part 2 solution: {}", solution1_2);

    let day2_input = read_input(Path::new("inputs/day2.txt"));
    let solution2_1 = day2::solve_part1(&day2_input);
    println!("Day 2, part 1 solution: {}", solution2_1);
    let solution2_2 = day2::solve_part2(&day2_input);
    println!("Day 2, part 2 solution: {}", solution2_2);
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
}
