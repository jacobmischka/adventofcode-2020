use std::{
    collections::HashSet,
    io::{self, BufRead},
    ops::{Deref, DerefMut},
};

fn main() {
    let mut grid = Grid::new();
    let mut hypergrid = HyperGrid::new();
    for (x, line) in io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .enumerate()
    {
        for (y, c) in line.char_indices() {
            if c == '#' {
                grid.insert(Cube {
                    x: x as isize,
                    y: y as isize,
                    z: 0,
                });
                hypergrid.insert(HyperCube {
                    x: x as isize,
                    y: y as isize,
                    z: 0,
                    w: 0,
                });
            }
        }
    }

    for _ in 0..6 {
        grid = grid.run_step();
        hypergrid = hypergrid.run_step();
    }

    println!("Part 1: {}", grid.len());
    println!("Part 2: {}", hypergrid.len());
}

#[derive(Debug, Clone, Default)]
struct Grid {
    map: HashSet<Cube>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
}

impl Grid {
    fn new() -> Grid {
        Default::default()
    }

    fn insert(&mut self, cube: Cube) {
        self.min_x = self.min_x.min(cube.x);
        self.max_x = self.max_x.max(cube.x);
        self.min_y = self.min_y.min(cube.y);
        self.max_y = self.max_y.max(cube.y);
        self.min_z = self.min_z.min(cube.z);
        self.max_z = self.max_z.max(cube.z);
        self.map.insert(cube);
    }

    fn run_step(&self) -> Grid {
        let mut new = Grid::new();

        for x in (self.min_x - 1)..=(self.max_x + 1) {
            for y in (self.min_y - 1)..=(self.max_y + 1) {
                for z in (self.min_z - 1)..=(self.max_z + 1) {
                    let cube = Cube { x, y, z };
                    let was_on = self.contains(&cube);

                    let num_on = cube.get_neighbors().iter().fold(0, |acc, neighbor| {
                        if self.contains(neighbor) {
                            acc + 1
                        } else {
                            acc
                        }
                    });

                    if (was_on && (num_on == 2 || num_on == 3)) || (!was_on && num_on == 3) {
                        new.insert(cube);
                    }
                }
            }
        }

        new
    }
}

impl Deref for Grid {
    type Target = HashSet<Cube>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube {
    fn get_neighbors(&self) -> [Cube; 26] {
        [
            Cube {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
            Cube {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
            Cube {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y + 1,
                z: self.z + 1,
            },
            Cube {
                x: self.x,
                y: self.y + 1,
                z: self.z - 1,
            },
            Cube {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y - 1,
                z: self.z + 1,
            },
            Cube {
                x: self.x,
                y: self.y - 1,
                z: self.z - 1,
            },
            Cube {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Cube {
                x: self.x + 1,
                y: self.y,
                z: self.z + 1,
            },
            Cube {
                x: self.x + 1,
                y: self.y,
                z: self.z - 1,
            },
            Cube {
                x: self.x + 1,
                y: self.y + 1,
                z: self.z,
            },
            Cube {
                x: self.x + 1,
                y: self.y + 1,
                z: self.z + 1,
            },
            Cube {
                x: self.x + 1,
                y: self.y + 1,
                z: self.z - 1,
            },
            Cube {
                x: self.x + 1,
                y: self.y - 1,
                z: self.z,
            },
            Cube {
                x: self.x + 1,
                y: self.y - 1,
                z: self.z + 1,
            },
            Cube {
                x: self.x + 1,
                y: self.y - 1,
                z: self.z - 1,
            },
            Cube {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Cube {
                x: self.x - 1,
                y: self.y,
                z: self.z + 1,
            },
            Cube {
                x: self.x - 1,
                y: self.y,
                z: self.z - 1,
            },
            Cube {
                x: self.x - 1,
                y: self.y + 1,
                z: self.z,
            },
            Cube {
                x: self.x - 1,
                y: self.y + 1,
                z: self.z + 1,
            },
            Cube {
                x: self.x - 1,
                y: self.y + 1,
                z: self.z - 1,
            },
            Cube {
                x: self.x - 1,
                y: self.y - 1,
                z: self.z,
            },
            Cube {
                x: self.x - 1,
                y: self.y - 1,
                z: self.z + 1,
            },
            Cube {
                x: self.x - 1,
                y: self.y - 1,
                z: self.z - 1,
            },
        ]
    }
}

#[derive(Debug, Clone, Default)]
struct HyperGrid {
    map: HashSet<HyperCube>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
    min_w: isize,
    max_w: isize,
}

impl HyperGrid {
    fn new() -> HyperGrid {
        Default::default()
    }

    fn insert(&mut self, cube: HyperCube) {
        self.min_x = self.min_x.min(cube.x);
        self.max_x = self.max_x.max(cube.x);
        self.min_y = self.min_y.min(cube.y);
        self.max_y = self.max_y.max(cube.y);
        self.min_z = self.min_z.min(cube.z);
        self.max_z = self.max_z.max(cube.z);
        self.min_w = self.min_w.min(cube.w);
        self.max_w = self.max_w.max(cube.w);
        self.map.insert(cube);
    }

    fn run_step(&self) -> HyperGrid {
        let mut new = HyperGrid::new();

        for x in (self.min_x - 1)..=(self.max_x + 1) {
            for y in (self.min_y - 1)..=(self.max_y + 1) {
                for z in (self.min_z - 1)..=(self.max_z + 1) {
                    for w in (self.min_w - 1)..=(self.max_w + 1) {
                        let cube = HyperCube { x, y, z, w };
                        let was_on = self.contains(&cube);

                        let num_on = cube.get_neighbors().iter().fold(0, |acc, neighbor| {
                            if self.contains(neighbor) {
                                acc + 1
                            } else {
                                acc
                            }
                        });

                        if (was_on && (num_on == 2 || num_on == 3)) || (!was_on && num_on == 3) {
                            new.insert(cube);
                        }
                    }
                }
            }
        }

        new
    }
}

impl Deref for HyperGrid {
    type Target = HashSet<HyperCube>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for HyperGrid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct HyperCube {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl HyperCube {
    fn get_neighbors(&self) -> [HyperCube; 80] {
        [
            HyperCube {
                x: self.x,
                y: self.y,
                z: self.z,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x,
                y: self.y,
                z: self.z,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x,
                y: self.y,
                z: self.z + 1,
                w: self.w,
            },
            HyperCube {
                x: self.x,
                y: self.y,
                z: self.z + 1,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x,
                y: self.y,
                z: self.z + 1,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x,
                y: self.y,
                z: self.z - 1,
                w: self.w,
            },
            HyperCube {
                x: self.x,
                y: self.y,
                z: self.z - 1,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x,
                y: self.y,
                z: self.z - 1,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x,
                y: self.y + 1,
                z: self.z,
                w: self.w,
            },
            HyperCube {
                x: self.x,
                y: self.y + 1,
                z: self.z,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x,
                y: self.y + 1,
                z: self.z,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x,
                y: self.y + 1,
                z: self.z + 1,
                w: self.w,
            },
            HyperCube {
                x: self.x,
                y: self.y + 1,
                z: self.z + 1,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x,
                y: self.y + 1,
                z: self.z + 1,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x,
                y: self.y + 1,
                z: self.z - 1,
                w: self.w,
            },
            HyperCube {
                x: self.x,
                y: self.y + 1,
                z: self.z - 1,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x,
                y: self.y + 1,
                z: self.z - 1,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x,
                y: self.y - 1,
                z: self.z,
                w: self.w,
            },
            HyperCube {
                x: self.x,
                y: self.y - 1,
                z: self.z,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x,
                y: self.y - 1,
                z: self.z,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x,
                y: self.y - 1,
                z: self.z + 1,
                w: self.w,
            },
            HyperCube {
                x: self.x,
                y: self.y - 1,
                z: self.z + 1,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x,
                y: self.y - 1,
                z: self.z + 1,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x,
                y: self.y - 1,
                z: self.z - 1,
                w: self.w,
            },
            HyperCube {
                x: self.x,
                y: self.y - 1,
                z: self.z - 1,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x,
                y: self.y - 1,
                z: self.z - 1,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y,
                z: self.z,
                w: self.w,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y,
                z: self.z,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y,
                z: self.z,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y,
                z: self.z + 1,
                w: self.w,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y,
                z: self.z + 1,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y,
                z: self.z + 1,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y,
                z: self.z - 1,
                w: self.w,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y,
                z: self.z - 1,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y,
                z: self.z - 1,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y + 1,
                z: self.z,
                w: self.w,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y + 1,
                z: self.z,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y + 1,
                z: self.z,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y + 1,
                z: self.z + 1,
                w: self.w,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y + 1,
                z: self.z + 1,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y + 1,
                z: self.z + 1,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y + 1,
                z: self.z - 1,
                w: self.w,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y + 1,
                z: self.z - 1,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y + 1,
                z: self.z - 1,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y - 1,
                z: self.z,
                w: self.w,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y - 1,
                z: self.z,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y - 1,
                z: self.z,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y - 1,
                z: self.z + 1,
                w: self.w,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y - 1,
                z: self.z + 1,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y - 1,
                z: self.z + 1,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y - 1,
                z: self.z - 1,
                w: self.w,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y - 1,
                z: self.z - 1,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x + 1,
                y: self.y - 1,
                z: self.z - 1,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y,
                z: self.z,
                w: self.w,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y,
                z: self.z,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y,
                z: self.z,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y,
                z: self.z + 1,
                w: self.w,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y,
                z: self.z + 1,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y,
                z: self.z + 1,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y,
                z: self.z - 1,
                w: self.w,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y,
                z: self.z - 1,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y,
                z: self.z - 1,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y + 1,
                z: self.z,
                w: self.w,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y + 1,
                z: self.z,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y + 1,
                z: self.z,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y + 1,
                z: self.z + 1,
                w: self.w,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y + 1,
                z: self.z + 1,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y + 1,
                z: self.z + 1,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y + 1,
                z: self.z - 1,
                w: self.w,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y + 1,
                z: self.z - 1,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y + 1,
                z: self.z - 1,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y - 1,
                z: self.z,
                w: self.w,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y - 1,
                z: self.z,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y - 1,
                z: self.z,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y - 1,
                z: self.z + 1,
                w: self.w,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y - 1,
                z: self.z + 1,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y - 1,
                z: self.z + 1,
                w: self.w - 1,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y - 1,
                z: self.z - 1,
                w: self.w,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y - 1,
                z: self.z - 1,
                w: self.w + 1,
            },
            HyperCube {
                x: self.x - 1,
                y: self.y - 1,
                z: self.z - 1,
                w: self.w - 1,
            },
        ]
    }
}
