use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: &str) -> u32 {
    let mut splitter_lines = parse_input(input).into_iter();
    let first_splitter = splitter_lines.next().unwrap()[0];
    let mut tachyons = HashSet::from_iter(vec![first_splitter - 1, first_splitter + 1]);
    let mut hit_split = 1;
    splitter_lines.for_each(|splitters| {
        let mut new_tachyons = HashSet::new();
        for tachyon in tachyons.clone() {
            if splitters.contains(&tachyon) {
                hit_split += 1;
                new_tachyons.insert(tachyon + 1);
                new_tachyons.insert(tachyon - 1);
            } else {
                new_tachyons.insert(tachyon);
            }
        }
        tachyons = new_tachyons;
    });
    hit_split
}

pub fn solve_part2(input: &str) -> usize {
    let mut splitter_lines = parse_input(input).into_iter();
    let first_splitter = splitter_lines.next().unwrap()[0];

    let mut tachyons = HashMap::new();
    tachyons.insert(first_splitter - 1, 1);
    tachyons.insert(first_splitter + 1, 1);

    splitter_lines.for_each(|splitters| {
        let mut new_tachyons = HashMap::new();
        for (tachyon_pos, count) in tachyons.clone() {
            if splitters.contains(&tachyon_pos) {
                new_tachyons
                    .entry(tachyon_pos - 1)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
                new_tachyons
                    .entry(tachyon_pos + 1)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            } else {
                new_tachyons
                    .entry(tachyon_pos)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            }
        }
        tachyons = new_tachyons;
    });
    tachyons.values().sum()
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .skip(2)
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '^')
                .map(|(i, _)| i)
                .collect()
        })
        .filter(|splitters: &Vec<usize>| !splitters.is_empty())
        .collect()
}
