use good_lp::{
    microlp, variable, variables, Expression, IntoAffineExpression, Solution, SolverModel,
};

pub fn solve_part2(input: &str) -> f64 {
    let instructions = parse_input(input);
    instructions
        .map(|(available_presses, target)| calculate_shortest(&available_presses, &target))
        .sum()
}

fn calculate_shortest(available_presses: &[Vec<usize>], joltages: &Vec<u16>) -> f64 {
    let mut variables = variables!();
    let mut press_variables = vec![];
    for _ in 0..available_presses.len() {
        let press = variables.add(variable().min(0).integer());
        press_variables.push(press);
    }

    let mut problem = microlp(variables.minimise(press_variables.iter().sum::<Expression>()));
    let mut expressions = vec![0.into_expression(); joltages.len()];
    for (index, press) in available_presses.iter().enumerate() {
        for light in press {
            expressions[*light] += press_variables[index];
        }
    }
    expressions
        .into_iter()
        .zip(joltages)
        .for_each(|(expression, joltage)| {
            problem.add_constraint(expression.eq(f64::from(*joltage)));
        });
    let solution = problem.solve().unwrap();
    press_variables
        .iter()
        .map(|&v| solution.value(v))
        .sum::<f64>()
}

fn parse_input(input: &str) -> impl Iterator<Item = (Vec<Vec<usize>>, Vec<u16>)> {
    input.lines().map(|line| {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        let buttons = &parts[1..parts.len() - 1]
            .iter()
            .map(|press| parse_press(press))
            .collect::<Vec<_>>();
        let joltage = parse_joltage(parts[parts.len() - 1]);
        (buttons.clone(), joltage)
    })
}

fn parse_press(input: &str) -> Vec<usize> {
    let buttons = &input[1..input.len() - 1];
    buttons
        .split(',')
        .map(|index| index.parse().unwrap())
        .collect()
}

fn parse_joltage(input: &str) -> Vec<u16> {
    let values = &input[1..input.len() - 1];
    values
        .split(',')
        .map(|value| value.parse().unwrap())
        .collect()
}
