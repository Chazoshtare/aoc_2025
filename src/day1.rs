pub fn solve_part1(input: &str) -> i32 {
    let rotations = parse_input(input);
    let mut dial = Dial::new();
    rotations
        .iter()
        .for_each(|rotation| dial.rotate(rotation.0, rotation.1));
    dial.exact_zeroes
}

pub fn solve_part2(input: &str) -> i32 {
    let rotations = parse_input(input);
    let mut dial = Dial::new();
    rotations
        .iter()
        .for_each(|rotation| dial.rotate(rotation.0, rotation.1));
    dial.exact_zeroes + dial.crossed_zeroes
}

fn parse_input(input: &str) -> Vec<(char, i32)> {
    input
        .lines()
        .map(|line| {
            let rotation = line.split_at(1);
            let direction = rotation.0.chars().next().unwrap();
            let value = rotation.1.parse::<i32>().unwrap();
            (direction, value)
        })
        .collect()
}

struct Dial {
    value: i32,
    exact_zeroes: i32,
    crossed_zeroes: i32,
}

impl Dial {
    fn new() -> Self {
        Self {
            value: 50,
            exact_zeroes: 0,
            crossed_zeroes: 0,
        }
    }

    fn rotate(&mut self, direction: char, value: i32) {
        match direction {
            'L' => self.rotate_left(value),
            'R' => self.rotate_right(value),
            _ => panic!("wrong direction: {}", direction),
        }
        if self.value == 0 {
            self.exact_zeroes += 1;
        }
    }

    fn rotate_left(&mut self, value: i32) {
        self.crossed_zeroes += value / 100;
        let rotate_by = value % 100;
        if self.value > 0 && rotate_by > self.value {
            self.crossed_zeroes += 1;
        }
        self.value -= rotate_by;
        if self.value < 0 {
            self.value += 100;
        }
    }

    fn rotate_right(&mut self, value: i32) {
        self.crossed_zeroes += value / 100;
        let rotate_by = value % 100;
        if rotate_by > (100 - self.value) {
            self.crossed_zeroes += 1;
        }
        self.value += value % 100;
        if self.value > 99 {
            self.value -= 100;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dial_rotation_after_99_fixes_overflow() {
        let mut dial = Dial::new();
        dial.rotate('R', 100);
        assert_eq!(dial.value, 50);
    }

    #[test]
    fn dial_rotation_before_0_fixes_overflow() {
        let mut dial = Dial::new();
        dial.rotate('L', 100);
        assert_eq!(dial.value, 50);
    }

    #[test]
    fn dial_rotation_from_0_to_left_should_not_count_anything() {
        let mut dial = Dial::new();
        dial.value = 0;
        dial.rotate('L', 10);
        assert_eq!(dial.value, 90);
        assert_eq!(dial.exact_zeroes, 0);
        assert_eq!(dial.crossed_zeroes, 0);
    }

    #[test]
    fn dial_rotation_over_99_right_counts_all_zeroes() {
        let mut dial = Dial::new();
        dial.rotate('R', 450);
        assert_eq!(dial.value, 0);
        assert_eq!(dial.exact_zeroes, 1);
        assert_eq!(dial.crossed_zeroes, 4);
    }

    #[test]
    fn dial_rotation_over_99_left_counts_all_zeroes() {
        let mut dial = Dial::new();
        dial.rotate('L', 250);
        assert_eq!(dial.value, 0);
        assert_eq!(dial.exact_zeroes, 1);
        assert_eq!(dial.crossed_zeroes, 2);
    }
}
