use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let coords = Grid(
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

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut trees = [0u128; 5];

    for (i, (right, down)) in slopes.iter().enumerate() {
        let mut x = 0;
        let mut y = 0;

        while y < coords.height() {
            match coords.get(x, y) {
                Tile::Open => {}
                Tile::Tree => {
                    trees[i] += 1;
                }
            }
            x += right;
            y += down;
        }
    }

    let part_1 = trees[1];
    let part_2 = trees.iter().fold(1u128, |acc, x| acc * x);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Open,
    Tree,
}

impl Tile {
    fn from_char(c: char) -> Option<Tile> {
        match c {
            '.' => Some(Tile::Open),
            '#' => Some(Tile::Tree),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn height(&self) -> usize {
        self.0.len()
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        let row = &self.0[y];
        row[x % row.len()]
    }
}
