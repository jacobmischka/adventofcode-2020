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

    let corners: BTreeSet<u16> = align_map
        .iter()
        .filter_map(|(id, touching_tiles)| {
            if touching_tiles.len() == 2 {
                Some(*id)
            } else {
                None
            }
        })
        .collect();

    let edges: BTreeSet<u16> = align_map
        .iter()
        .filter_map(|(id, touching_tiles)| {
            if touching_tiles.len() == 3 {
                Some(*id)
            } else {
                None
            }
        })
        .collect();

    let centers: BTreeSet<u16> = align_map
        .iter()
        .filter_map(|(id, touching_tiles)| {
            if touching_tiles.len() == 4 {
                Some(*id)
            } else {
                None
            }
        })
        .collect();

    println!(
        "Part 1: {}",
        corners.iter().fold(1u128, |acc, id| acc * *id as u128)
    );

    let mut tiles_remaining: BTreeSet<u16> = tiles.keys().copied().collect();

    let width = (tiles.len() as f64).sqrt() as usize;
    let mut image: Vec<Vec<u16>> = vec![vec![0; width]; width];

    image[0][0] = *corners.iter().next().unwrap();
    tiles_remaining.remove(&image[0][0]);

    for x in 0..width {
        for y in 0..width {
            if image[x][y] != 0 {
                continue;
            }

            let mut possible_tiles: BTreeSet<u16> = tiles_remaining
                .intersection(
                    if (x == 0 && y == width - 1)
                        || (x == width - 1 && y == 0)
                        || (x == width - 1 && y == width - 1)
                    {
                        &corners
                    } else if x == 0 || x == width - 1 || y == 0 || y == width - 1 {
                        &edges
                    } else {
                        &centers
                    },
                )
                .copied()
                .collect();

            if x > 0 {
                possible_tiles = possible_tiles
                    .intersection(
                        &align_map
                            .get(&image[x - 1][y])
                            .unwrap()
                            .keys()
                            .copied()
                            .collect(),
                    )
                    .copied()
                    .collect();
            }

            if y > 0 {
                possible_tiles = possible_tiles
                    .intersection(
                        &align_map
                            .get(&image[x][y - 1])
                            .unwrap()
                            .keys()
                            .copied()
                            .collect(),
                    )
                    .copied()
                    .collect();
            }

            let tile_id = possible_tiles.iter().next().copied().unwrap();
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

    'outer1: loop {
        for i in 0..8 {
            for j in 0..8 {
                for k in 0..8 {
                    if (Some(Side::Bottom), Some(Side::Right))
                        == (
                            image[0][0].aligned_side(&image[0][1]),
                            image[0][0].aligned_side(&image[1][0]),
                        )
                    {
                        break 'outer1;
                    }

                    if k == 3 {
                        image[1][0] = image[1][0].flip_horizontally();
                    } else {
                        image[1][0] = image[1][0].rotate_right();
                    }
                }
                if j == 3 {
                    image[0][1] = image[0][1].flip_horizontally();
                } else {
                    image[0][1] = image[0][1].rotate_right();
                }
            }
            if i == 3 {
                image[0][0] = image[0][0].flip_horizontally();
            } else {
                image[0][0] = image[0][0].rotate_right();
            }
        }
    }

    for x in 0..width {
        for y in 0..width {
            if x == 0 && y == 0 || x == 1 && y == 0 || x == 0 && y == 1 {
                continue;
            }

            for i in 0..8 {
                if (x == 0 || image[x][y].aligned_side(&image[x - 1][y]) == Some(Side::Left))
                    && (y == 0 || image[x][y].aligned_side(&image[x][y - 1]) == Some(Side::Top))
                {
                    break;
                }

                if i == 3 {
                    image[x][y] = image[x][y].flip_horizontally();
                } else {
                    image[x][y] = image[x][y].rotate_right();
                }
            }
        }
    }

    let mut image = Image::from_tiles(&image);
    let mut found = false;

    for i in 0..12 {
        for x in 0..(image.width - SEAMONSTER_WIDTH) {
            for y in 0..(image.width - SEAMONSTER_HEIGHT) {
                let seamonster_pixels: Vec<_> = SEAMONSTER_PIXELS
                    .iter()
                    .map(|&(dx, dy)| (x + dx, y + dy))
                    .collect();
                if seamonster_pixels.iter().all(|&(x, y)| image.pixels[x][y]) {
                    found = true;
                    for &(x, y) in &seamonster_pixels {
                        image.pixels[x][y] = false;
                    }
                }
            }
        }

        if found {
            break;
        }

        if i == 3 {
            image = image.flip_horizontally();
        } else {
            image = image.rotate_right();
        }
    }

    let part_2 = image.pixels.iter().fold(0, |acc, col| {
        acc + col
            .iter()
            .fold(0, |acc, &cell| if cell { acc + 1 } else { acc })
    });

    println!("Part 2: {}", part_2);
}

const TILE_WIDTH: usize = 10;

const SEAMONSTER_WIDTH: usize = 20;
const SEAMONSTER_HEIGHT: usize = 3;
const SEAMONSTER_PIXELS: [(usize, usize); 15] = [
    (0, 1),
    (1, 2),
    (4, 2),
    (5, 1),
    (6, 1),
    (7, 2),
    (10, 2),
    (11, 1),
    (12, 1),
    (13, 2),
    (16, 2),
    (17, 1),
    (18, 0),
    (18, 1),
    (19, 1),
];

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, PartialEq, Eq)]
struct Image {
    width: usize,
    pixels: Vec<Vec<bool>>,
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.width {
            writeln!(f)?;
            for x in 0..self.width {
                write!(f, "{}", if self.pixels[x][y] { '#' } else { '.' })?;
            }
        }

        Ok(())
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Image {
    fn from_tiles(tiles: &Vec<Vec<Tile>>) -> Image {
        let width = (TILE_WIDTH - 2) * tiles.len();
        let mut pixels = vec![vec![false; width]; width];

        for tile_x in 0..tiles.len() {
            for tile_y in 0..tiles[tile_x].len() {
                let tile = &tiles[tile_x][tile_y];

                for x in 1..(TILE_WIDTH - 1) {
                    for y in 1..(TILE_WIDTH - 1) {
                        pixels[tile_x * (TILE_WIDTH - 2) + (x - 1)]
                            [tile_y * (TILE_WIDTH - 2) + (y - 1)] = tile.pixels[x][y];
                    }
                }
            }
        }

        Image { width, pixels }
    }

    fn rotate_right(&self) -> Image {
        let mut new = self.clone();

        for x in 0..self.width {
            for y in 0..self.width {
                new.pixels[self.width - 1 - y][x] = self.pixels[x][y];
            }
        }

        new
    }

    fn flip_horizontally(&self) -> Image {
        let mut new = self.clone();

        for x in 0..self.width {
            for y in 0..self.width {
                new.pixels[self.width - 1 - x][y] = self.pixels[x][y];
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
