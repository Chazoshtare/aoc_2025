use crate::grid::{Coordinates, Grid};

const PAPER_ROLL: char = '@';

pub fn solve_part1(input: &str) -> usize {
    let grid = parse_input(input);
    grid.all_occurrences_of(PAPER_ROLL)
        .iter()
        .map(|roll| grid.all_adjacent_to(roll))
        .map(|adjacent| adjacent.iter().filter(|(_, c)| *c == PAPER_ROLL).count())
        .filter(|count| *count < 4)
        .count()
}

pub fn solve_part2(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut removed = 0;
    loop {
        let removable: Vec<Coordinates> = grid
            .all_occurrences_of(PAPER_ROLL)
            .into_iter()
            .filter(|roll| {
                let adjacent = grid.all_adjacent_to(roll);
                adjacent.iter().filter(|(_, c)| *c == PAPER_ROLL).count() < 4
            })
            .collect();
        if removable.is_empty() {
            break;
        }
        removed += removable.len();
        removable
            .iter()
            .for_each(|coordinates| grid.replace_at(coordinates, '.'));
    }
    removed
}

fn parse_input(input: &str) -> Grid {
    let grid = input.lines().map(|line| line.chars().collect()).collect();
    Grid::new(grid)
}
