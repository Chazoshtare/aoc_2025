pub fn solve_part1(input: &str) -> usize {
    let (presents, regions) = parse_input(input);
    let presents_wherever = regions
        .iter()
        .filter(|region| region.fits_all_square_presents())
        .count();
    let presents_impossible = regions
        .iter()
        .filter(|region| region.never_fits_presents(&presents))
        .count();
    if presents_wherever + presents_impossible != regions.len() {
        println!("harder input, requires packing check");
    }
    presents_wherever
}

fn parse_input(input: &str) -> (Vec<Present>, Vec<Region>) {
    let mut segments = input.split("\n\n");
    let presents = segments
        .by_ref()
        .take(6)
        .map(Present::parse)
        .collect::<Vec<_>>();
    let regions = segments
        .next()
        .unwrap()
        .lines()
        .map(Region::parse)
        .collect::<Vec<_>>();
    (presents, regions)
}

#[derive(Debug)]
struct Present {
    index: u32,
    shape: Vec<Vec<bool>>,
}

impl Present {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let index: u32 = lines
            .next()
            .unwrap()
            .chars()
            .nth(0)
            .unwrap()
            .to_digit(10)
            .unwrap();
        let shape = lines
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        Self { index, shape }
    }

    fn size(&self) -> usize {
        self.shape.iter().fold(0, |acc, row| {
            acc + row.iter().filter(|&&value| value).count()
        })
    }
}

#[derive(Debug)]
struct Region {
    x: usize,
    y: usize,
    target_presents: Vec<usize>,
}

impl Region {
    fn parse(line: &str) -> Self {
        let (size, presents) = line.split_once(": ").unwrap();
        let (x_str, y_str) = size.split_once('x').unwrap();
        let x = x_str.parse().unwrap();
        let y= y_str.parse().unwrap();
        let target_presents = presents
            .split_ascii_whitespace()
            .map(|number| number.parse::<usize>().unwrap())
            .collect();
        Self {
            x,
            y,
            target_presents,
        }
    }

    fn fits_all_square_presents(&self) -> bool {
        let x = self.x - (self.x % 3);
        let y = self.y - (self.y % 3);
        let area = x * y;
        let total_presents: usize = self.target_presents.iter().sum();
        total_presents * 9 <= area
    }

    fn never_fits_presents(&self, available_presents: &[Present]) -> bool {
        let area = self.x * self.y;
        let spaces_taken = available_presents.iter()
            .fold(0, |acc, present| {
                let count = self.target_presents[present.index as usize];
                acc + count * present.size()
            });
        area < spaces_taken
    }
}
