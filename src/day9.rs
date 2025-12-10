use std::cmp::{max, min};

pub fn solve_part1(input: &str) -> i64 {
    let tiles = parse_input(input);
    let furthest_upper_left = tiles.iter().min_by_key(|Tile { x, y }| x + y).unwrap();
    let furthest_upper_right = tiles.iter().max_by_key(|Tile { x, y }| x - y).unwrap();
    let furthest_lower_left = tiles.iter().max_by_key(|Tile { x, y }| y - x).unwrap();
    let furthest_lower_right = tiles.iter().max_by_key(|Tile { x, y }| x + y).unwrap();

    let left_diagonal_rectangle = Rectangle::new(furthest_upper_left, furthest_lower_right);
    let right_diagonal_rectangle = Rectangle::new(furthest_upper_right, furthest_lower_left);

    if left_diagonal_rectangle.area > right_diagonal_rectangle.area {
        left_diagonal_rectangle.area
    } else {
        right_diagonal_rectangle.area
    }
}

pub fn solve_part2(input: &str) -> i64 {
    let mut tiles = parse_input(input);
    tiles.push(tiles[0].clone());
    let lines: Vec<(&Tile, &Tile)> = tiles
        .windows(2)
        .map(|window| (&window[0], &window[1]))
        .collect();
    let mut rectangles = vec![];
    for tile_a in &tiles {
        for tile_b in &tiles {
            rectangles.push(Rectangle::new(tile_a, tile_b));
        }
    }
    rectangles.sort_unstable_by(|a, b| b.area.cmp(&a.area));

    let biggest = rectangles
        .iter()
        .find(|rectangle| lines.iter().all(|line| !rectangle.intersects_with(*line)))
        .unwrap();
    biggest.area
}

fn parse_input(input: &str) -> Vec<Tile> {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| Tile::new(x.parse().unwrap(), y.parse().unwrap()))
        .collect()
}

#[derive(Debug, Clone)]
struct Tile {
    x: i64,
    y: i64,
}

impl Tile {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Rectangle<'a> {
    tile_a: &'a Tile,
    tile_b: &'a Tile,
    area: i64,
}

impl<'a> Rectangle<'a> {
    fn new(tile_a: &'a Tile, tile_b: &'a Tile) -> Self {
        let side_a = (tile_a.x - tile_b.x).abs() + 1;
        let side_b = (tile_a.y - tile_b.y).abs() + 1;
        let area = side_a * side_b;
        Self {
            tile_a,
            tile_b,
            area,
        }
    }

    fn intersects_with(&self, line: (&Tile, &Tile)) -> bool {
        let min_x = min(self.tile_a.x, self.tile_b.x);
        let max_x = max(self.tile_a.x, self.tile_b.x);
        let min_y = min(self.tile_a.y, self.tile_b.y);
        let max_y = max(self.tile_a.y, self.tile_b.y);

        let horizontal = line.0.y == line.1.y;
        if horizontal {
            let y_in_rectangle = min_y < line.0.y && line.0.y < max_y;
            let line_from = min(line.0.x, line.1.x);
            let line_to = max(line.0.x, line.1.x);
            let x_range = (min_x + 1)..max_x;
            let x_through_rectangle = x_range.contains(&line.0.x)
                || x_range.contains(&line.1.x)
                || (line_from <= min_x && line_to >= max_x);
            y_in_rectangle && x_through_rectangle
        } else {
            let x_in_rectangle = min_x < line.0.x && line.0.x < max_x;
            let line_from = min(line.0.y, line.1.y);
            let line_to = max(line.0.y, line.1.y);
            let y_range = (min_y + 1)..max_y;
            let y_through_rectangle = y_range.contains(&line.0.y)
                || y_range.contains(&line.1.y)
                || (line_from <= min_y && line_to >= max_y);
            x_in_rectangle && y_through_rectangle
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_rectangle_intersecting_with_horizontal_line() {
        let tile_a = Tile::new(1, 1);
        let tile_b = Tile::new(4, 3);
        let rectangle = Rectangle::new(&tile_a, &tile_b);
        let line_a = Tile::new(2, 2);
        let line_b = Tile::new(5, 2);
        let line = (&line_a, &line_b);
        assert_eq!(rectangle.intersects_with(line), true);
    }

    #[test]
    fn detects_rectangle_intersecting_with_vertical_line() {
        let tile_a = Tile::new(1, 1);
        let tile_b = Tile::new(4, 3);
        let rectangle = Rectangle::new(&tile_a, &tile_b);
        let line_a = Tile::new(3, 0);
        let line_b = Tile::new(3, 4);
        let line = (&line_a, &line_b);
        assert_eq!(rectangle.intersects_with(line), true);
    }

    #[test]
    fn detects_rectangle_intersecting_with_vertical_line_inside() {
        let tile_a = Tile::new(11, 1);
        let tile_b = Tile::new(2, 3);
        let rectangle = Rectangle::new(&tile_a, &tile_b);
        let line_a = Tile::new(7, 1);
        let line_b = Tile::new(7, 3);
        let line = (&line_a, &line_b);
        assert_eq!(rectangle.intersects_with(line), true);
    }

    #[test]
    fn detects_rectangle_not_intersecting_with_line() {
        let tile_a = Tile::new(1, 1);
        let tile_b = Tile::new(4, 3);
        let rectangle = Rectangle::new(&tile_a, &tile_b);
        let line_a = Tile::new(2, 3);
        let line_b = Tile::new(6, 3);
        let line = (&line_a, &line_b);
        assert_eq!(rectangle.intersects_with(line), false);
    }

    #[test]
    fn detects_rectangle_not_intersecting_with_touching_horizontal_line() {
        let tile_a = Tile::new(1, 1);
        let tile_b = Tile::new(4, 3);
        let rectangle = Rectangle::new(&tile_a, &tile_b);
        let line_a = Tile::new(4, 2);
        let line_b = Tile::new(6, 2);
        let line = (&line_a, &line_b);
        assert_eq!(rectangle.intersects_with(line), false);
    }
}
