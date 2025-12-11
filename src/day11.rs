use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: &str) -> usize {
    let devices = parse_input(input);
    let paths = get_paths(&devices, "you", "out", "");
    paths.iter().count()
}

pub fn solve_part2(input: &str) -> usize {
    let devices = parse_input(input);
    // let dac_to_fft = get_paths(&devices, "dac", "fft", "");
    // println!("{}", dac_to_fft.len());
    let dtf = count_paths(&devices, "dac", "fft", "out", &mut HashMap::new());
    println!("dtf: {}", dtf);
    let ftd = count_paths(&devices, "fft", "dac", "out", &mut HashMap::new());
    println!("dtf: {}", dtf);
    // let fft_to_dac = get_paths(&devices, "fft", "dac");
    // println!("{}", fft_to_dac.len());

    0
    // paths
    //     .iter()
    //     .filter(|path| path.last().unwrap() == &"out")
    //     .filter(|path| path.contains(&"fft") && path.contains(&"dac"))
    //     .count()
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
    ignoring: &str,
    counts: &mut HashMap<String, usize>,
) -> usize {
    if from == to {
        return 1;
    }
    if from == ignoring {
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
