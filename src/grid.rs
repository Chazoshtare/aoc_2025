const DIRECTIONS: [Direction; 8] = [
    Direction::NorthWest,
    Direction::North,
    Direction::NorthEast,
    Direction::West,
    Direction::East,
    Direction::SouthWest,
    Direction::South,
    Direction::SouthEast,
];

pub struct Grid {
    grid: Vec<Vec<char>>,
    rows: i32,
    columns: i32,
}

impl Grid {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let rows = grid[0].len() as i32;
        let columns = grid.len() as i32;
        Self {
            grid,
            rows,
            columns,
        }
    }

    pub fn replace_at(&mut self, coordinates: &Coordinates, c: char) {
        self.grid[coordinates.row as usize][coordinates.column as usize] = c;
    }

    pub fn all_occurrences_of(&self, c: char) -> Vec<Coordinates> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(row, chars)| {
                chars
                    .iter()
                    .enumerate()
                    .filter(|(_, gc)| **gc == c)
                    .map(|(column, _)| Coordinates::new(row as i32, column as i32))
                    .collect::<Vec<Coordinates>>()
            })
            .collect()
    }

    pub fn all_adjacent_to(&self, coordinates: &Coordinates) -> Vec<(Coordinates, char)> {
        DIRECTIONS
            .iter()
            .map(|direction| {
                let mut coordinates = coordinates.clone();
                coordinates.move_towards(direction);
                coordinates
            })
            .filter(|coordinates| self.is_in_bounds(coordinates))
            .map(|coordinates| {
                let row = coordinates.row;
                let column = coordinates.column;
                (coordinates, self.grid[row as usize][column as usize])
            })
            .collect()
    }

    fn is_in_bounds(&self, coordinates: &Coordinates) -> bool {
        (0..self.rows).contains(&coordinates.row) && (0..self.columns).contains(&coordinates.column)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Coordinates {
    row: i32,
    column: i32,
}

impl Coordinates {
    fn new(row: i32, column: i32) -> Self {
        Self { row, column }
    }

    fn move_towards(&mut self, direction: &Direction) {
        match direction {
            Direction::NorthWest => {
                self.row -= 1;
                self.column -= 1;
            }
            Direction::North => self.row -= 1,
            Direction::NorthEast => {
                self.row -= 1;
                self.column += 1;
            }
            Direction::West => self.column -= 1,
            Direction::East => self.column += 1,
            Direction::SouthWest => {
                self.row += 1;
                self.column -= 1;
            }
            Direction::South => self.row += 1,
            Direction::SouthEast => {
                self.row += 1;
                self.column += 1;
            }
        }
    }
}

enum Direction {
    NorthWest,
    North,
    NorthEast,
    West,
    East,
    SouthWest,
    South,
    SouthEast,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_all_occurrences_of_char_in_grid() {
        let grid = Grid::new(vec![
            vec!['+', '-', '+'],
            vec!['-', '+', '-'],
            vec!['-', '-', '+'],
        ]);
        let occurrences = grid.all_occurrences_of('+');
        let expected = vec![
            Coordinates::new(0, 0),
            Coordinates::new(0, 2),
            Coordinates::new(1, 1),
            Coordinates::new(2, 2),
        ];
        assert_eq!(occurrences, expected);
    }

    #[test]
    fn returns_all_coordinates_adjacent_to_middle() {
        let grid = Grid::new(vec![
            vec!['+', '-', '+'],
            vec!['-', '+', '-'],
            vec!['-', '-', '+'],
        ]);
        let middle = Coordinates::new(1, 1);
        let adjacent = grid.all_adjacent_to(&middle);
        let expected = vec![
            (Coordinates::new(0, 0), '+'),
            (Coordinates::new(0, 1), '-'),
            (Coordinates::new(0, 2), '+'),
            (Coordinates::new(1, 0), '-'),
            (Coordinates::new(1, 2), '-'),
            (Coordinates::new(2, 0), '-'),
            (Coordinates::new(2, 1), '-'),
            (Coordinates::new(2, 2), '+'),
        ];
        assert_eq!(adjacent, expected);
    }

    #[test]
    fn returns_all_coordinates_adjacent_to_upper_wall() {
        let grid = Grid::new(vec![
            vec!['+', '-', '+'],
            vec!['-', '+', '-'],
            vec!['-', '-', '+'],
        ]);
        let middle = Coordinates::new(0, 1);
        let adjacent = grid.all_adjacent_to(&middle);
        let expected = vec![
            (Coordinates::new(0, 0), '+'),
            (Coordinates::new(0, 2), '+'),
            (Coordinates::new(1, 0), '-'),
            (Coordinates::new(1, 1), '+'),
            (Coordinates::new(1, 2), '-'),
        ];
        assert_eq!(adjacent, expected);
    }

    #[test]
    fn returns_all_adjacent_to_right_wall() {
        let grid = Grid::new(vec![
            vec!['+', '-', '+'],
            vec!['-', '+', '-'],
            vec!['-', '-', '+'],
        ]);
        let middle = Coordinates::new(1, 2);
        let adjacent = grid.all_adjacent_to(&middle);
        let expected = vec![
            (Coordinates::new(0, 1), '-'),
            (Coordinates::new(0, 2), '+'),
            (Coordinates::new(1, 1), '+'),
            (Coordinates::new(2, 1), '-'),
            (Coordinates::new(2, 2), '+'),
        ];
        assert_eq!(adjacent, expected);
    }

    #[test]
    fn returns_all_adjacent_to_lower_left_corner() {
        let grid = Grid::new(vec![
            vec!['+', '-', '+'],
            vec!['-', '+', '-'],
            vec!['-', '-', '+'],
        ]);
        let middle = Coordinates::new(2, 0);
        let adjacent = grid.all_adjacent_to(&middle);
        let expected = vec![
            (Coordinates::new(1, 0), '-'),
            (Coordinates::new(1, 1), '+'),
            (Coordinates::new(2, 1), '-'),
        ];
        assert_eq!(adjacent, expected);
    }
}
