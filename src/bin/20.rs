use std::{
    collections::BTreeMap,
    fmt,
    io::{self, BufRead},
    str::FromStr,
};

fn main() {
    let mut tiles: BTreeMap<u16, Tile> = BTreeMap::new();

    let mut tile_id: Option<u16> = None;
    let mut tile: Option<Tile> = None;
    let mut i = 0;
    for line in io::stdin().lock().lines().filter_map(Result::ok) {
        if line.is_empty() {
            i = 0;
            tile_id = None;
            continue;
        }

        if tile_id.is_none() {
            tile = Some(Tile::new());
            tile_id = line
                .split_whitespace()
                .skip(1)
                .next()
                .unwrap()
                .replace(':', "")
                .parse()
                .ok();
        } else {
            tile.as_mut().unwrap().set_row(&line, i).unwrap();
            i += 1;
        }

        if i == TILE_WIDTH {
            tiles.insert(tile_id.unwrap(), tile.unwrap());
            tile = None;
        }
    }

    let mut align_map: BTreeMap<u16, Vec<u16>> = BTreeMap::new();
    for (id, tile) in tiles.iter() {
        let entry = align_map.entry(*id).or_default();
        for (other_id, other_tile) in tiles.iter() {
            if tile != other_tile && tile.aligns_with(other_tile) {
                entry.push(*other_id);
            }
        }
    }

    let corners: Vec<u16> = align_map
        .iter()
        .filter_map(|(id, touching_tiles)| {
            if touching_tiles.len() == 2 {
                Some(*id)
            } else {
                None
            }
        })
        .collect();
    let center = align_map
        .iter()
        .find_map(|(id, touching_tiles)| {
            if touching_tiles.len() == 4 {
                Some(*id)
            } else {
                None
            }
        })
        .unwrap();

    println!(
        "Part 1: {}",
        corners.iter().fold(1u128, |acc, id| acc * *id as u128)
    );

    let mut image = [[0; 3]; 3];

    image[0][0] = corners[0];
    image[0][1] = align_map.get(&image[0][0]).unwrap()[0];
    image[0][2] = align_map
        .get(&image[0][1])
        .unwrap()
        .iter()
        .copied()
        .find(|&id| id != image[0][0] && id != center)
        .unwrap();

    image[1][0] = align_map.get(&image[0][0]).unwrap()[1];
    image[1][1] = center;
    image[1][2] = align_map
        .get(&image[0][2])
        .unwrap()
        .iter()
        .copied()
        .find(|&id| id != image[0][2])
        .unwrap();

    image[2][0] = align_map
        .get(&image[1][2])
        .unwrap()
        .iter()
        .copied()
        .find(|&id| id != image[0][0] && id != image[1][1])
        .unwrap();
    image[2][1] = align_map
        .get(&image[2][0])
        .unwrap()
        .iter()
        .copied()
        .find(|&id| id != image[1][2])
        .unwrap();
    image[2][2] = align_map
        .get(&image[2][1])
        .unwrap()
        .iter()
        .copied()
        .find(|id| align_map.get(&image[1][2]).unwrap().contains(id))
        .unwrap();

    dbg!(&image);
}

const TILE_WIDTH: usize = 10;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Tile {
    pixels: [[bool; TILE_WIDTH]; TILE_WIDTH],
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..TILE_WIDTH {
            writeln!(f)?;
            for x in 0..TILE_WIDTH {
                write!(f, "{}", if self.pixels[x][y] { '#' } else { '.' })?;
            }
        }

        Ok(())
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tile = Tile::new();

        for (y, line) in s.lines().enumerate() {
            tile.set_row(&line, y)?;
        }

        Ok(tile)
    }
}

impl Tile {
    fn new() -> Tile {
        Tile {
            pixels: [[false; TILE_WIDTH]; TILE_WIDTH],
        }
    }

    fn set_row(&mut self, row: &str, y: usize) -> Result<(), String> {
        for (x, c) in row.char_indices() {
            self.pixels[x][y] = match c {
                '#' => true,
                '.' => false,
                c => {
                    return Err(format!("invalid pixel data: {}", c));
                }
            };
        }

        Ok(())
    }

    fn get_row(&self, y: usize) -> [bool; TILE_WIDTH] {
        let mut a = [false; TILE_WIDTH];

        for x in 0..TILE_WIDTH {
            a[x] = self.pixels[x][y];
        }

        a
    }

    fn get_col(&self, x: usize) -> &[bool; TILE_WIDTH] {
        &self.pixels[x]
    }

    fn rotate_right(&self) -> Tile {
        let mut new = Tile::new();

        for x in 0..TILE_WIDTH {
            for y in 0..TILE_WIDTH {
                new.pixels[TILE_WIDTH - 1 - y][x] = self.pixels[x][y];
            }
        }

        new
    }

    fn flip_row(row: &Edge) -> Edge {
        let mut new = [false; TILE_WIDTH];

        for i in 0..TILE_WIDTH {
            new[TILE_WIDTH - 1 - i] = row[i];
        }

        new
    }

    fn flip_horizontally(&self) -> Tile {
        let mut new = Tile::new();

        for x in 0..TILE_WIDTH {
            for y in 0..TILE_WIDTH {
                new.pixels[TILE_WIDTH - 1 - x][y] = self.pixels[x][y];
            }
        }

        new
    }

    fn edges(&self) -> [Edge; 4] {
        [
            self.get_edge(&Side::Top),
            self.get_edge(&Side::Bottom),
            self.get_edge(&Side::Left),
            self.get_edge(&Side::Right),
        ]
    }

    fn edges_match(e1: &Edge, e2: &Edge) -> bool {
        e1 == e2 || *e1 == Tile::flip_row(e2)
    }

    fn aligns_with(&self, other: &Tile) -> bool {
        for this_edge in &self.edges() {
            for that_edge in &other.edges() {
                if Tile::edges_match(this_edge, that_edge) {
                    return true;
                }
            }
        }

        false
    }

    fn get_edge(&self, side: &Side) -> Edge {
        match side {
            Side::Top => self.get_row(0),
            Side::Bottom => self.get_row(TILE_WIDTH - 1),
            Side::Left => self.get_col(0).clone(),
            Side::Right => self.get_col(TILE_WIDTH - 1).clone(),
        }
    }

    fn aligned_side(&self, other: &Tile) -> Option<Side> {
        if self.get_edge(&Side::Top) == other.get_edge(&Side::Bottom) {
            Some(Side::Top)
        } else if self.get_edge(&Side::Bottom) == other.get_edge(&Side::Top) {
            Some(Side::Bottom)
        } else if self.get_edge(&Side::Right) == other.get_edge(&Side::Left) {
            Some(Side::Right)
        } else if self.get_edge(&Side::Left) == other.get_edge(&Side::Right) {
            Some(Side::Left)
        } else {
            None
        }
    }
}

type Edge = [bool; TILE_WIDTH];

#[derive(Debug, Clone, Copy)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rotate_works() {
        assert_eq!(
            Tile::from_str(
                "..##.#..#.\n\
				##..#.....\n\
				#...##..#.\n\
				####.#...#\n\
				##.##.###.\n\
				##...#.###\n\
				.#.#.#..##\n\
				..#....#..\n\
				###...#.#.\n\
				..###..###"
            )
            .unwrap()
            .rotate_right(),
            Tile::from_str(
                ".#..#####.\n\
				.#.####.#.\n\
				###...#..#\n\
				#..#.##..#\n\
				#....#.##.\n\
				...##.##.#\n\
				.#...#....\n\
				#.#.##....\n\
				##.###.#.#\n\
				#..##.#..."
            )
            .unwrap()
        );
    }

    #[test]
    fn flip_works() {
        assert_eq!(
            Tile::from_str(
                "..##.#..#.\n\
				##..#.....\n\
				#...##..#.\n\
				####.#...#\n\
				##.##.###.\n\
				##...#.###\n\
				.#.#.#..##\n\
				..#....#..\n\
				###...#.#.\n\
				..###..###"
            )
            .unwrap()
            .flip_horizontally(),
            Tile::from_str(
                ".#..#.##..\n\
				.....#..##\n\
				.#..##...#\n\
				#...#.####\n\
				.###.##.##\n\
				###.#...##\n\
				##..#.#.#.\n\
				..#....#..\n\
				.#.#...###\n\
				###..###.."
            )
            .unwrap()
        );
    }
}
