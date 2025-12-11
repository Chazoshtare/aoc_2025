use std::ops::RangeInclusive;

pub fn solve_part1(input: &str) -> usize {
    let (ranges, ids) = parse_input(input);
    ids.iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count()
}

pub fn solve_part2(input: &str) -> usize {
    let (mut ranges, _) = parse_input(input);
    let mut results = vec![];
    while !ranges.is_empty() {
        let mut current = ranges.swap_remove(0);
        loop {
            let initial = current.clone();
            for range in &mut ranges {
                if current.contains(range.start()) && current.contains(range.end()) {
                    *range = 0..=0;
                } else if current.contains(range.start()) {
                    current = *current.start()..=*range.end();
                    *range = 0..=0;
                } else if current.contains(range.end()) {
                    current = *range.start()..=*current.end();
                    *range = 0..=0;
                } else if range.contains(current.start()) && range.contains(current.end()) {
                    current = range.clone();
                    *range = 0..=0;
                }
            }
            ranges.retain(|range| *range != (0..=0));
            if current == initial {
                break;
            }
        }
        results.push(current);
    }
    results.into_iter().map(Iterator::count).sum()
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut lines = input.lines();
    let ranges = lines
        .by_ref()
        .take_while(|line| line.contains('-'))
        .map(|line| line.split_once('-').unwrap())
        .map(|(from, to)| from.parse::<u64>().unwrap()..=to.parse::<u64>().unwrap())
        .collect();
    let ids = lines.map(|line| line.parse::<u64>().unwrap()).collect();
    (ranges, ids)
}
