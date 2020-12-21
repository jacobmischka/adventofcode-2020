use std::{
    collections::{BTreeMap, BTreeSet},
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

    let mut align_map: BTreeMap<u16, BTreeMap<u16, Edge>> = BTreeMap::new();
    for (id, tile) in tiles.iter() {
        let entry = align_map.entry(*id).or_default();
        for (other_id, other_tile) in tiles.iter() {
            if tile != other_tile {
                if let Some(edge) = tile.aligns_with(other_tile) {
                    entry.insert(*other_id, edge);
                }
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

    let mut tiles_remaining: BTreeSet<u16> = tiles.keys().copied().collect();

    let mut image = [[0; 3]; 3];

    image[0][0] = corners[0];
    tiles_remaining.remove(&corners[0]);
    image[1][1] = center;
    tiles_remaining.remove(&center);

    for x in 0..3 {
        for y in 0..3 {
            if image[x][y] != 0 {
                continue;
            }

            let mut possible_tiles: Option<BTreeSet<u16>> = None;

            if x > 0 {
                possible_tiles = Some(
                    align_map
                        .get(&image[x - 1][y])
                        .unwrap()
                        .keys()
                        .copied()
                        .collect(),
                );
            }

            if y > 0 {
                match possible_tiles {
                    Some(tiles) => {
                        possible_tiles = Some(
                            tiles
                                .intersection(
                                    &align_map
                                        .get(&image[x][y - 1])
                                        .unwrap()
                                        .keys()
                                        .copied()
                                        .collect(),
                                )
                                .copied()
                                .collect(),
                        )
                    }
                    None => {
                        possible_tiles = Some(
                            align_map
                                .get(&image[x][y - 1])
                                .unwrap()
                                .keys()
                                .copied()
                                .collect(),
                        );
                    }
                }
            }

            let tile_id = possible_tiles
                .unwrap()
                .intersection(&tiles_remaining)
                .copied()
                .next()
                .unwrap();
            tiles_remaining.remove(&tile_id);
            image[x][y] = tile_id;
        }
    }

    let mut image: Vec<Vec<Tile>> = image
        .iter()
        .map(|row| {
            row.iter()
                .map(|id| tiles.get(&id).unwrap())
                .cloned()
                .collect()
        })
        .collect();

    for c in [0, 2].iter().copied() {
        'outer: for i in 0..8 {
            for j in 0..8 {
                for k in 0..8 {
                    if image[c][c].aligned_side(&image[c][1]).is_some()
                        && image[c][c].aligned_side(&image[1][c]).is_some()
                    {
                        break 'outer;
                    }

                    if k == 4 {
                        image[1][c] = image[1][c].flip_horizontally();
                    } else {
                        image[1][c] = image[1][c].rotate_right();
                    }
                }
                if j == 4 {
                    image[c][1] = image[c][1].flip_horizontally();
                } else {
                    image[c][1] = image[c][1].rotate_right();
                }
            }
            if i == 4 {
                image[c][c] = image[c][c].flip_horizontally();
            } else {
                image[c][c] = image[c][c].rotate_right();
            }
        }
    }

    for i in 0..8 {
        if image[2][0].aligned_side(&image[1][0]).is_some()
            && image[2][0].aligned_side(&image[2][1]).is_some()
        {
            break;
        }

        if i == 4 {
            image[2][0] = image[2][0].rotate_right();
        } else {
            image[2][0] = image[2][0].flip_horizontally();
        }
    }

    for i in 0..8 {
        if image[0][2].aligned_side(&image[1][2]).is_some()
            && image[0][2].aligned_side(&image[0][1]).is_some()
        {
            break;
        }

        if i == 4 {
            image[0][2] = image[0][2].rotate_right();
        } else {
            image[0][2] = image[0][2].flip_horizontally();
        }
    }

    for i in 0..8 {
        if image[1][1].aligned_side(&image[0][1]).is_some()
            && image[1][1].aligned_side(&image[1][0]).is_some()
            && image[1][1].aligned_side(&image[1][2]).is_some()
            && image[1][1].aligned_side(&image[2][1]).is_some()
        {
            break;
        }

        if i == 4 {
            image[1][1] = image[1][1].rotate_right();
        } else {
            image[1][1] = image[1][1].flip_horizontally();
        }
    }

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

    fn aligns_with(&self, other: &Tile) -> Option<Edge> {
        for this_edge in &self.edges() {
            for that_edge in &other.edges() {
                if Tile::edges_match(this_edge, that_edge) {
                    return Some(*this_edge);
                }
            }
        }

        None
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
