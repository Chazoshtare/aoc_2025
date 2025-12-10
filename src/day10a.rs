use itertools::Itertools;

pub fn solve_part1(input: &str) -> usize {
    parse_input(input)
        .map(|(target, available_presses)| {
            generate_all_press_combinations(&available_presses)
                .filter(|combination| presses_reach_target(combination, target))
                .map(|combination| combination.len())
                .min()
                .unwrap()
        })
        .sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = (u16, Vec<u16>)> {
    input.lines().map(|line| {
        let mut parts = line.split_ascii_whitespace();
        let target = parse_target(parts.next().unwrap());
        let mut parts = parts.rev();
        parts.next();
        let buttons: Vec<_> = parts.collect();
        let available_presses = parse_available_presses(&buttons);

        (target, available_presses)
    })
}

fn parse_target(input: &str) -> u16 {
    let lights = &input[1..input.len() - 1];
    let mut target = 0;
    for (index, char) in lights.chars().enumerate() {
        if char == '#' {
            target += 1 << index;
        }
    }
    target
}

fn parse_available_presses(input: &[&str]) -> Vec<u16> {
    input
        .iter()
        .map(|buttons| parse_available_press(buttons))
        .collect()
}

fn parse_available_press(input: &str) -> u16 {
    let buttons = &input[1..input.len() - 1];
    let mut press = 0;
    for button in buttons.split(',') {
        let index = button.parse::<u16>().unwrap();
        press += 1 << index;
    }
    press
}

fn generate_all_press_combinations(available_presses: &[u16]) -> impl Iterator<Item = Vec<u16>> {
    available_presses.iter().copied().powerset()
}

fn presses_reach_target(presses: &[u16], target: u16) -> bool {
    let mut initial = 0;
    for press in presses {
        initial ^= press;
    }
    initial == target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presses_reach_target_returns_true_for_valid_presses() {
        let presses = [0b11111, 0b01000, 0b00111, 0b00100, 0b00001];
        let target = 0b10101;
        let result = presses_reach_target(&presses, target);
        assert_eq!(result, true);
    }

    #[test]
    fn presses_reach_target_returns_false_for_invalid_presses() {
        let presses = [0b11111, 0b01000, 0b10101, 0b00100, 0b00001];
        let target = 0b10101;
        let result = presses_reach_target(&presses, target);
        assert_eq!(result, false);
    }
}
