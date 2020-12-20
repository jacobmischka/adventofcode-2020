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

    dbg!(&tiles);
}

const TILE_WIDTH: usize = 10;

#[derive(Clone, PartialEq, Eq)]
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

    fn flip_horizontally(&self) -> Tile {
        let mut new = Tile::new();

        for x in 0..TILE_WIDTH {
            for y in 0..TILE_WIDTH {
                new.pixels[TILE_WIDTH - 1 - x][y] = self.pixels[x][y];
            }
        }

        new
    }
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
