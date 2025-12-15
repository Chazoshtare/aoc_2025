pub fn solve_part1(input: &str) -> u64 {
    let (operations, numbers) = parse_input(input);
    numbers
        .into_iter()
        .reduce(|a, b| calculate_two_rows(&a, &b, &operations))
        .unwrap()
        .iter()
        .sum()
}

pub fn solve_part2(input: &str) -> u64 {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let max_position = lines[0].len() - 1;
    let operations_index = lines.len() - 1;
    let mut total = 0;
    let mut cephal_numbers: Vec<u64> = vec![];
    for cursor in (0..=max_position).rev() {
        let cephal_number_str = (0..operations_index)
            .map(|number_line| lines[number_line][cursor])
            .collect::<String>();
        let cephal_number = cephal_number_str.trim();
        if !cephal_number.is_empty() {
            cephal_numbers.push(cephal_number.parse().unwrap());
        }

        let operation = lines[operations_index][cursor];
        if operation != ' ' {
            let result = cephal_numbers
                .iter()
                .copied()
                .reduce(|a, b| perform_operation(a, b, operation))
                .unwrap();
            total += result;
            cephal_numbers.clear();
        }
    }
    total
}

fn parse_input(input: &str) -> (Vec<char>, Vec<Vec<u64>>) {
    let mut lines = input.lines().rev();
    let operations: Vec<char> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();
    let numbers: Vec<Vec<u64>> = lines
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();
    (operations, numbers)
}

fn calculate_two_rows(row_a: &[u64], row_b: &[u64], operations: &[char]) -> Vec<u64> {
    row_a
        .iter()
        .enumerate()
        .map(|(index, a)| perform_operation(*a, row_b[index], operations[index]))
        .collect()
}

fn perform_operation(a: u64, b: u64, operation: char) -> u64 {
    match operation {
        '+' => a + b,
        '*' => a * b,
        _ => panic!("unsupported operation {operation}"),
    }
}
