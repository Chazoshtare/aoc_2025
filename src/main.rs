mod day1;
mod day10a;
mod day10b;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
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

    let day5_input = read_input(Path::new("inputs/day5.txt"));
    let solution5_1 = day5::solve_part1(&day5_input);
    println!("Day 5, part 1 solution: {solution5_1}");
    let solution5_2 = day5::solve_part2(&day5_input);
    println!("Day 5, part 2 solution: {solution5_2}");

    let day6_input = read_input(Path::new("inputs/day6.txt"));
    let solution6_1 = day6::solve_part1(&day6_input);
    println!("Day 6, part 1 solution: {solution6_1}");
    let solution6_2 = day6::solve_part2(&day6_input);
    println!("Day 6, part 2 solution: {solution6_2}");

    let day7_input = read_input(Path::new("inputs/day7.txt"));
    let solution7_1 = day7::solve_part1(&day7_input);
    println!("Day 7, part 1 solution: {solution7_1}");
    let solution7_2 = day7::solve_part2(&day7_input);
    println!("Day 7, part 2 solution: {solution7_2}");

    let day8_input = read_input(Path::new("inputs/day8.txt"));
    let solution8_1 = day8::solve_part1(&day8_input);
    println!("Day 8, part 1 solution: {solution8_1}");
    let solution8_2 = day8::solve_part2(&day8_input);
    println!("Day 8, part 2 solution: {solution8_2}");

    let day9_input = read_input(Path::new("inputs/day9.txt"));
    let solution9_1 = day9::solve_part1(&day9_input);
    println!("Day 9, part 1 solution: {solution9_1}");
    let solution9_2 = day9::solve_part2(&day9_input);
    println!("Day 9, part 2 solution: {solution9_2}");

    let day10_input = read_input(Path::new("inputs/day10.txt"));
    let solution10_1 = day10a::solve_part1(&day10_input);
    println!("Day 10, part 1 solution: {solution10_1}");
    let solution10_2 = day10b::solve_part2(&day10_input);
    println!("Day 10, part 2 solution: {solution10_2}");
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

    #[test]
    fn day5_part1_returns_correct_solution() {
        let day5_input = read_input(Path::new("inputs/day5.txt"));
        let solution = day5::solve_part1(&day5_input);
        assert_eq!(solution, 505);
    }

    #[test]
    fn day5_part2_returns_correct_solution() {
        let day5_input = read_input(Path::new("inputs/day5.txt"));
        let solution = day5::solve_part2(&day5_input);
        assert_eq!(solution, 344423158480189);
    }

    #[test]
    fn day6_part1_returns_correct_solution() {
        let day6_input = read_input(Path::new("inputs/day6.txt"));
        let solution = day6::solve_part1(&day6_input);
        assert_eq!(solution, 6171290547579);
    }

    #[test]
    fn day6_part2_returns_correct_solution() {
        let day6_input = read_input(Path::new("inputs/day6.txt"));
        let solution = day6::solve_part2(&day6_input);
        assert_eq!(solution, 8811937976367);
    }

    #[test]
    fn day7_part1_returns_correct_solution() {
        let day7_input = read_input(Path::new("inputs/day7.txt"));
        let solution = day7::solve_part1(&day7_input);
        assert_eq!(solution, 1539);
    }

    #[test]
    fn day7_part2_returns_correct_solution() {
        let day7_input = read_input(Path::new("inputs/day7.txt"));
        let solution = day7::solve_part2(&day7_input);
        assert_eq!(solution, 6479180385864);
    }

    #[test]
    fn day8_part1_returns_correct_solution() {
        let day8_input = read_input(Path::new("inputs/day8.txt"));
        let solution = day8::solve_part1(&day8_input);
        assert_eq!(solution, 42840);
    }

    #[test]
    fn day8_part2_returns_correct_solution() {
        let day8_input = read_input(Path::new("inputs/day8.txt"));
        let solution = day8::solve_part2(&day8_input);
        assert_eq!(solution, 170629052);
    }

    #[test]
    fn day9_part1_returns_correct_solution() {
        let day9_input = read_input(Path::new("inputs/day9.txt"));
        let solution = day9::solve_part1(&day9_input);
        assert_eq!(solution, 4769758290);
    }

    #[test]
    fn day9_part2_returns_correct_solution() {
        let day9_input = read_input(Path::new("inputs/day9.txt"));
        let solution = day9::solve_part2(&day9_input);
        assert_eq!(solution, 1588990708);
    }

    #[test]
    fn day10_part1_returns_correct_solution() {
        let day10_input = read_input(Path::new("inputs/day10.txt"));
        let solution = day10a::solve_part1(&day10_input);
        assert_eq!(solution, 500);
    }

    #[test]
    fn day10_part2_returns_correct_solution() {
        let day10_input = read_input(Path::new("inputs/day10.txt"));
        let solution = day10b::solve_part2(&day10_input);
        assert_eq!(solution, 19763f64);
    }
}
