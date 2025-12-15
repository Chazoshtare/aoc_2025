pub fn solve_part1(input: &str) -> u64 {
    parse_input(input)
        .map(|batteries| highest_two_battery_joltage(&batteries))
        .sum()
}

pub fn solve_part2(input: &str) -> u64 {
    parse_input(input)
        .map(|batteries| highest_twelve_battery_joltage(&batteries))
        .sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = Vec<u64>> {
    input.lines().map(|line| {
        line.chars()
            .map(|char| u64::from(char.to_digit(10).unwrap()))
            .collect()
    })
}

fn highest_two_battery_joltage(batteries: &[u64]) -> u64 {
    let mut first = 0;
    let mut second = 0;
    let length = batteries.len();
    batteries.iter().take(length - 1).for_each(|&battery| {
        if battery > first {
            first = battery;
            second = 0;
        } else if battery > second {
            second = battery;
        }
    });
    if batteries[length - 1] > second {
        second = batteries[length - 1];
    }
    first * 10 + second
}

fn highest_twelve_battery_joltage(batteries: &[u64]) -> u64 {
    let mut selected = Vec::with_capacity(12);
    let mut from = 0;
    for i in 0..=11 {
        let to = batteries.len() - 11 + i;
        let digit = find_first_highest_digit_with_index(&batteries[from..to]);
        from += digit.0 + 1;
        selected.insert(0, digit.1);
    }
    let mut result = 0;
    for (power, digit) in selected.iter().enumerate() {
        result += digit * 10u64.pow(power as u32);
    }
    result
}

fn find_first_highest_digit_with_index(batteries: &[u64]) -> (usize, u64) {
    let highest = batteries
        .iter()
        .enumerate()
        .rev()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();
    (highest.0, *highest.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_highest_two_battery_joltage_for_batteries_next_to_each_other() {
        let batteries = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let joltage = highest_two_battery_joltage(&batteries);
        assert_eq!(joltage, 98);
    }

    #[test]
    fn finds_highest_two_battery_joltage_for_batteries_with_largest_digit_in_middle() {
        let batteries = vec![2, 8, 2, 9, 2, 3, 2];
        let joltage = highest_two_battery_joltage(&batteries);
        assert_eq!(joltage, 93);
    }

    #[test]
    fn finds_highest_two_battery_joltage_with_largest_value_ending() {
        let batteries = vec![1, 1, 5, 9];
        let joltage = highest_two_battery_joltage(&batteries);
        assert_eq!(joltage, 59);
    }

    #[test]
    fn finds_highest_eleven_battery_joltage() {
        let batteries = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        let joltage = highest_twelve_battery_joltage(&batteries);
        assert_eq!(joltage, 434234234278);
    }

    #[test]
    fn finds_highest_eleven_battery_joltage_with_leading_digit_early() {
        let batteries = vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];
        let joltage = highest_twelve_battery_joltage(&batteries);
        assert_eq!(joltage, 888911112111);
    }
}
