use std::ops::RangeInclusive;

pub fn solve_part1(input: &str) -> u64 {
    let ranges = parse_input(input);
    ranges.map(|range| add_invalid_ids(range)).sum()
}

pub fn solve_part2(input: &str) -> u64 {
    let ranges = parse_input(input);
    ranges
        .map(|range| add_invalid_ids_with_repeats(range))
        .sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = RangeInclusive<u64>> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|range| range.split_once('-').unwrap())
        .map(|(start, end)| start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap())
}

fn add_invalid_ids(range: RangeInclusive<u64>) -> u64 {
    let mut invalid_ids_value = 0u64;
    range.for_each(|number| {
        let number_str = number.to_string();
        let length = number_str.len();
        if length % 2 == 0 {
            let mid_index = length / 2;
            let (first, second) = number_str.split_at(mid_index);
            if first == second {
                invalid_ids_value += number_str.parse::<u64>().unwrap();
            }
        }
    });
    invalid_ids_value
}

fn add_invalid_ids_with_repeats(range: RangeInclusive<u64>) -> u64 {
    range
        .filter(|number| is_invalid_with_repeats(*number))
        .sum()
}

fn is_invalid_with_repeats(number: u64) -> bool {
    let digits = number_to_digits(number);
    let max_split = digits.len() / 2;
    for split in (1..=max_split) {
        if digits.len() % split == 0 {
            let mut chunks = digits.chunks_exact(split);
            let pattern = chunks.next().unwrap();
            if chunks.all(|chunk| chunk == pattern) {
                return true;
            };
        }
    }
    false
}

fn number_to_digits(number: u64) -> Vec<u64> {
    let mut number = number;
    let mut digits = Vec::with_capacity(10);
    while number > 0 {
        digits.insert(0, number % 10);
        number = number / 10;
    }
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_number_to_digits() {
        let number = 123456789u64;
        let digits = number_to_digits(number);
        assert_eq!(digits, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
