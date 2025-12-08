use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: &str) -> usize {
    let mut junctions = parse_input(input);
    let distances = calculate_all_distances(&junctions);
    let mut formed_connections = distances
        .iter()
        .take(1000)
        .map(|distance| (distance.junction_a.clone(), distance.junction_b.clone()))
        .collect();

    let mut circuits = vec![];
    while let Some(junction) = junctions.pop() {
        circuits.push(extract_circuit(&junction, &mut formed_connections));
        junctions.retain(|junction| circuits.iter().all(|circuit| !circuit.contains(junction)));
    }

    let mut circuit_lengths: Vec<usize> = circuits.iter().map(HashSet::len).collect();
    circuit_lengths.sort_unstable();
    circuit_lengths
        .iter()
        .rev()
        .take(3)
        .copied()
        .reduce(|a, b| a * b)
        .unwrap()
}

pub fn solve_part2(input: &str) -> i64 {
    let junctions = parse_input(input);
    let mut circuits = HashMap::new();
    junctions.iter().enumerate().for_each(|(i, junction)| {
        circuits.insert(junction.clone(), i); // junction to circuit number
    });
    let mut distances = calculate_all_distances(&junctions);
    let from_wall: i64;
    loop {
        let shortest = distances.remove(0);
        let circuit_a = *circuits.get(&shortest.junction_a).unwrap();
        let circuit_b = *circuits.get(&shortest.junction_b).unwrap();
        if circuit_a != circuit_b {
            for (junction, circuit_number) in circuits.clone() {
                if circuit_number == circuit_a {
                    circuits.insert(junction, circuit_b);
                }
            }
        }
        let mut circuit_ids = circuits.values().collect::<Vec<_>>();
        circuit_ids.dedup();
        if circuit_ids.len() == 1 {
            from_wall = shortest.junction_a.x * shortest.junction_b.x;
            break;
        }
    }
    from_wall
}

fn parse_input(input: &str) -> Vec<Junction> {
    input
        .lines()
        .map(|line| line.splitn(3, ',').collect::<Vec<&str>>())
        .map(|coordinates| {
            Junction::new(
                coordinates[0].parse().unwrap(),
                coordinates[1].parse().unwrap(),
                coordinates[2].parse().unwrap(),
            )
        })
        .collect()
}

fn calculate_all_distances(junctions: &[Junction]) -> Vec<Distance> {
    let mut distances = vec![];
    for i in 0..junctions.len() {
        for j in (i + 1)..junctions.len() {
            distances.push(Distance::calculate(&junctions[i], &junctions[j]));
        }
    }
    distances.sort_by_key(|distance| distance.distance);
    distances
}

fn extract_circuit(
    start: &Junction,
    connections: &mut Vec<(Junction, Junction)>,
) -> HashSet<Junction> {
    let mut circuit = HashSet::new();
    circuit.insert(start.clone());
    loop {
        let mut connected = vec![];
        for junction in &circuit {
            let index = connections
                .iter()
                .position(|(from, to)| junction.is_in_connection((from, to)));
            if let Some(index) = index {
                let connection = connections.swap_remove(index);
                connected.push(connection.0);
                connected.push(connection.1);
            }
        }
        if connected.is_empty() {
            break;
        }
        circuit.extend(connected);
    }
    circuit
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Junction {
    x: i64,
    y: i64,
    z: i64,
}

impl Junction {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn distance_to(&self, junction: &Self) -> i64 {
        let x_distance = (junction.x - self.x).pow(2);
        let y_distance = (junction.y - self.y).pow(2);
        let z_distance = (junction.z - self.z).pow(2);
        (x_distance + y_distance + z_distance).isqrt()
    }

    fn is_in_connection(&self, connection: (&Self, &Self)) -> bool {
        self == connection.0 || self == connection.1
    }
}

struct Distance {
    junction_a: Junction,
    junction_b: Junction,
    distance: i64,
}

impl Distance {
    fn calculate(junction_a: &Junction, junction_b: &Junction) -> Self {
        let distance = junction_a.distance_to(junction_b);
        Distance {
            junction_a: junction_a.clone(),
            junction_b: junction_b.clone(),
            distance,
        }
    }
}
