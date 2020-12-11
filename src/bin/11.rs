use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let grid = Grid(
        stdin
            .lock()
            .lines()
            .map(|s| {
                s.unwrap()
                    .chars()
                    .filter_map(Tile::from_char)
                    .collect::<Vec<_>>()
            })
            .collect(),
    );

    let mut p1_grid = grid.clone();
    loop {
        let new_grid = p1_grid.apply_round_adjacent();

        if new_grid == p1_grid {
            break;
        } else {
            p1_grid = new_grid;
        }
    }

    let part_1 = p1_grid
        .iter()
        .fold(0, |acc, t| if *t == Tile::Occupied { acc + 1 } else { acc });

    println!("Part 1: {}", part_1);

    let mut p2_grid = grid.clone();
    loop {
        let new_grid = p2_grid.apply_round_in_sight();

        if new_grid == p2_grid {
            break;
        } else {
            p2_grid = new_grid;
        }
    }

    let part_2 = p2_grid
        .iter()
        .fold(0, |acc, t| if *t == Tile::Occupied { acc + 1 } else { acc });

    println!("Part 2: {}", part_2);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

impl Tile {
    fn from_char(c: char) -> Option<Tile> {
        match c {
            '.' => Some(Tile::Floor),
            'L' => Some(Tile::Empty),
            '#' => Some(Tile::Occupied),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.0.get(y).and_then(|row| row.get(x))
    }

    fn set(&mut self, x: usize, y: usize, val: Tile) {
        self.0[y][x] = val;
    }

    fn iter(&self) -> impl Iterator<Item = &Tile> {
        self.0.iter().flat_map(|row| row.iter())
    }

    fn apply_round_adjacent(&self) -> Grid {
        let mut new = self.clone();

        let get_adjacent = |x: usize, y: usize| {
            DIRECTION_VECTORS
                .iter()
                .map(move |(x_vec, y_vec)| {
                    (
                        add_direction_vector(x, *x_vec),
                        add_direction_vector(y, *y_vec),
                    )
                })
                .filter_map(|(x, y)| x.and_then(|x| y.map(|y| (x, y))))
        };

        for (y, row) in self.0.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Floor => {}
                    Tile::Empty => {
                        if get_adjacent(x, y)
                            .filter_map(|(x, y)| self.get(x, y))
                            .all(|t| *t != Tile::Occupied)
                        {
                            new.set(x, y, Tile::Occupied);
                        }
                    }
                    Tile::Occupied => {
                        if get_adjacent(x, y)
                            .filter_map(|(x, y)| self.get(x, y))
                            .filter(|t| **t == Tile::Occupied)
                            .count()
                            >= 4
                        {
                            new.set(x, y, Tile::Empty);
                        }
                    }
                }
            }
        }

        new
    }

    fn apply_round_in_sight(&self) -> Grid {
        let mut new = self.clone();

        let get_in_sight = |x: usize, y: usize| {
            DIRECTION_VECTORS.iter().filter_map(move |(x_vec, y_vec)| {
                let mut radius = 1;
                while let (Some(new_x), Some(new_y)) = (
                    add_direction_vector(x, x_vec * radius),
                    add_direction_vector(y, y_vec * radius),
                ) {
                    if let Some(new_tile) = self.get(new_x, new_y) {
                        match new_tile {
                            Tile::Occupied | Tile::Empty => {
                                return Some(new_tile);
                            }
                            _ => {}
                        }
                        radius += 1;
                    } else {
                        break;
                    }
                }

                None
            })
        };

        for (y, row) in self.0.iter().enumerate() {
            for (x, tile) in row
                .iter()
                .enumerate()
                .filter(|(_, tile)| **tile != Tile::Floor)
            {
                match tile {
                    Tile::Floor => {}
                    Tile::Empty => {
                        if get_in_sight(x, y).all(|t| *t != Tile::Occupied) {
                            new.set(x, y, Tile::Occupied);
                        }
                    }
                    Tile::Occupied => {
                        if get_in_sight(x, y).filter(|t| **t == Tile::Occupied).count() >= 5 {
                            new.set(x, y, Tile::Empty);
                        }
                    }
                }
            }
        }

        new
    }
}

const DIRECTION_VECTORS: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn add_direction_vector(lhs: usize, rhs: isize) -> Option<usize> {
    if rhs.is_positive() {
        Some(lhs + rhs as usize)
    } else {
        lhs.checked_sub(rhs.abs() as usize)
    }
}
