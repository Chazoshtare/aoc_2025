use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: &str) -> usize {
    let devices = parse_input(input);
    count_paths(&devices, "you", "out", &[""], &mut HashMap::new())
}

pub fn solve_part2(input: &str) -> usize {
    let devices = parse_input(input);
    let svr_to_fft = count_paths(&devices, "svr", "fft", &["out", "dac"], &mut HashMap::new());
    let fft_to_dac = count_paths(&devices, "fft", "dac", &["out"], &mut HashMap::new());
    let dac_to_out = count_paths(&devices, "dac", "out", &["fft"], &mut HashMap::new());
    svr_to_fft * fft_to_dac * dac_to_out
}

fn count_paths(
    devices: &HashMap<&str, HashSet<&str>>,
    from: &str,
    to: &str,
    ignoring: &[&str],
    counts: &mut HashMap<String, usize>,
) -> usize {
    if from == to {
        return 1;
    }
    if ignoring.contains(&from) {
        return 0;
    }
    if let Some(count) = counts.get(from) {
        return *count;
    }

    let directions = devices.get(from).unwrap();
    let result: usize = directions
        .iter()
        .map(|&direction| count_paths(devices, direction, to, ignoring, counts))
        .sum();
    counts.insert(from.to_string(), result);
    result
}

fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    input
        .lines()
        .map(|line| line.split_at(3))
        .map(|(device, outputs)| {
            let outputs: HashSet<&str> =
                outputs[1..outputs.len()].split_ascii_whitespace().collect();
            (device, outputs)
        })
        .collect()
}
