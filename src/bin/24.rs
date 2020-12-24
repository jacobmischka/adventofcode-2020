use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
    str::FromStr,
};

fn main() {
    let mut tiles: HashMap<(isize, isize), isize> = HashMap::new();
    for line in io::stdin().lock().lines().filter_map(Result::ok) {
        let mut total_east = 0;
        let mut total_north = 0;

        let mut i = 0;
        while i < line.len() {
            if i + 2 <= line.len() {
                if let Ok(direction) = Direction::from_str(&line[i..i + 2]) {
                    let (east, north) = direction.canonical_vector();
                    total_east += east;
                    total_north += north;
                    i += 2;
                    continue;
                }
            }

            let (east, north) = Direction::from_str(&line[i..i + 1])
                .unwrap()
                .canonical_vector();
            total_east += east;
            total_north += north;
            i += 1;
        }

        *tiles.entry((total_east, total_north)).or_default() += 1;
    }

    let mut tiles: HashSet<(isize, isize)> = tiles
        .into_iter()
        .filter_map(|(c, count)| if count % 2 == 1 { Some(c) } else { None })
        .collect();

    println!("Part 1: {}", tiles.len());

    for _ in 0..100 {
        let mut new_tiles = tiles.clone();

        let mut min_x = tiles.iter().fold(isize::MAX, |acc, (x, _)| acc.min(*x)) - 1;
        let mut max_x = tiles.iter().fold(isize::MIN, |acc, (x, _)| acc.max(*x)) + 1;
        let min_y = tiles.iter().fold(isize::MAX, |acc, (_, y)| acc.min(*y)) - 1;
        let max_y = tiles.iter().fold(isize::MIN, |acc, (_, y)| acc.max(*y)) + 1;

        if min_x % 2 != 0 {
            min_x -= 1;
        }

        if max_x % 2 != 0 {
            max_x += 1;
        }

        for base_x in min_x..=max_x {
            for y in min_y..=max_y {
                let x = if y % 2 == 1 { base_x + 1 } else { base_x };

                let num_black = [
                    Direction::East.canonical_vector(),
                    Direction::Southeast.canonical_vector(),
                    Direction::Southwest.canonical_vector(),
                    Direction::West.canonical_vector(),
                    Direction::Northwest.canonical_vector(),
                    Direction::Northeast.canonical_vector(),
                ]
                .iter()
                .map(|&(dx, dy)| (x + dx, y + dy))
                .fold(0, |acc, coords| {
                    if tiles.contains(&coords) {
                        acc + 1
                    } else {
                        acc
                    }
                });

                let tile = (x, y);
                if tiles.contains(&tile) {
                    if num_black == 0 || num_black > 2 {
                        new_tiles.remove(&tile);
                    }
                } else if num_black == 2 {
                    new_tiles.insert(tile);
                }
            }
        }

        tiles = new_tiles;
    }

    println!("Part 2: {}", tiles.len());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

impl Direction {
    fn canonical_vector(&self) -> (isize, isize) {
        match self {
            Direction::East => (2, 0),
            Direction::Southeast => (1, -1),
            Direction::Southwest => (-1, -1),
            Direction::West => (-2, 0),
            Direction::Northwest => (-1, 1),
            Direction::Northeast => (1, 1),
        }
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "e" => Ok(Direction::East),
            "se" => Ok(Direction::Southeast),
            "sw" => Ok(Direction::Southwest),
            "w" => Ok(Direction::West),
            "nw" => Ok(Direction::Northwest),
            "ne" => Ok(Direction::Northeast),
            s => Err(format!("invalid direction: {}", s)),
        }
    }
}
