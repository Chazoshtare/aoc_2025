use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: &str) -> usize {
    let devices = parse_input(input);
    let paths = get_paths(&devices, "you", "out", "");
    paths.iter().count()
}

pub fn solve_part2(input: &str) -> usize {
    let devices = parse_input(input);
    let svr_to_dac = count_paths(&devices, "svr", "dac", &["out", "fft"], &mut HashMap::new());
    let dac_to_fft = count_paths(&devices, "dac", "fft", &["out"], &mut HashMap::new());
    let fft_to_out = count_paths(&devices, "fft", "out", &["dac"], &mut HashMap::new());

    println!("std {}", svr_to_dac);
    println!("dtf {}", dac_to_fft);
    println!("fto {}", fft_to_out);

    let svr_to_fft = count_paths(&devices, "svr", "fft", &["out", "dac"], &mut HashMap::new());
    let fft_to_dac = count_paths(&devices, "fft", "dac", &["out"], &mut HashMap::new());
    let dac_to_out = count_paths(&devices, "dac", "out", &["fft"], &mut HashMap::new());

    println!("stf {}", svr_to_fft);
    println!("ftd {}", fft_to_dac);
    println!("dto {}", dac_to_out);

    svr_to_fft * fft_to_dac * dac_to_out
}

fn get_paths<'a>(
    devices: &HashMap<&str, HashSet<&'a str>>,
    from: &str,
    to: &str,
    ignoring: &str,
) -> Vec<Vec<&'a str>> {
    let mut paths = devices
        .get(from)
        .unwrap()
        .iter()
        .map(|&to| vec![to])
        .collect::<Vec<_>>();

    loop {
        let mut new_paths = vec![];
        for path in &paths {
            let current = path[path.len() - 1];
            if current == to || current == "out" {
                new_paths.push(path.clone());
            } else {
                let directions = devices.get(current).unwrap();
                directions
                    .iter()
                    .filter(|&&direction| direction != ignoring)
                    .for_each(|to| {
                        if !path.contains(to) {
                            let mut new_path = path.clone();
                            new_path.push(to);
                            new_paths.push(new_path);
                        }
                    })
            }
        }
        if new_paths == paths {
            break;
        }
        paths = new_paths;
    }
    paths
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
    counts.insert(from.to_string(), result.clone());
    result
}

fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    input
        .lines()
        .map(|line| line.split_at(3))
        .map(|(device, outputs)| {
            let outputs: HashSet<&str> = (&outputs[1..outputs.len()])
                .split_ascii_whitespace()
                .collect();
            (device, outputs)
        })
        .collect()
}
