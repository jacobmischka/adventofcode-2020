use std::{
    io::{self, BufRead},
    str::FromStr,
};

fn main() {
    let actions: Vec<Action> = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|s| Action::from_str(&s).ok())
        .collect();

    let mut ship = Ship::default();
    for action in actions.iter() {
        ship.take_action(action);
    }

    let part_1 = ship.x.abs() + ship.y.abs();
    println!("Part 1: {}", part_1);

    let mut ship = Ship::default();
    let mut waypoint = Waypoint::new();
    for action in actions.iter() {
        match action {
            Action::Forward(val) => {
                ship.move_to_waypoint(&waypoint, *val);
            }
            x => {
                waypoint.take_action(x);
            }
        }
    }

    let part_2 = ship.x.abs() + ship.y.abs();
    println!("Part 2: {}", part_2);
}

trait Actionable {
    fn take_action(&mut self, action: &Action);
}

#[derive(Debug, Clone)]
struct Waypoint {
    x: i64,
    y: i64,
}

impl Waypoint {
    fn new() -> Waypoint {
        Waypoint { x: 10, y: 1 }
    }
}

impl Actionable for Waypoint {
    fn take_action(&mut self, action: &Action) {
        use Action::*;

        match action {
            North(val) => {
                self.y += *val as i64;
            }
            South(val) => {
                self.y -= *val as i64;
            }
            East(val) => {
                self.x += *val as i64;
            }
            West(val) => {
                self.x -= *val as i64;
            }
            Left(val) => {
                let (new_x, new_y) = match val {
                    0 => (self.x, self.y),
                    90 => (self.y * -1, self.x),
                    180 => (self.x * -1, self.y * -1),
                    270 => (self.y, self.x * -1),
                    x => panic!("Unknown rotation: {}", x),
                };

                self.x = new_x;
                self.y = new_y;
            }
            Right(val) => {
                let (new_x, new_y) = match val {
                    0 => (self.x, self.y),
                    90 => (self.y, self.x * -1),
                    180 => (self.x * -1, self.y * -1),
                    270 => (self.y * -1, self.x),
                    x => panic!("Unknown rotation: {}", x),
                };

                self.x = new_x;
                self.y = new_y;
            }
            x => panic!("invalid waypoint action: {:?}", x),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Ship {
    x: i64,
    y: i64,
    direction: u16,
}

impl Ship {
    fn move_to_waypoint(&mut self, waypoint: &Waypoint, multiplier: u16) {
        self.x += waypoint.x * multiplier as i64;
        self.y += waypoint.y * multiplier as i64;
    }
}

impl Actionable for Ship {
    fn take_action(&mut self, action: &Action) {
        use Action::*;

        match action {
            North(val) => {
                self.y += *val as i64;
            }
            South(val) => {
                self.y -= *val as i64;
            }
            East(val) => {
                self.x += *val as i64;
            }
            West(val) => {
                self.x -= *val as i64;
            }
            Left(val) => {
                self.direction = (self.direction + 360 - val) % 360;
            }
            Right(val) => {
                self.direction = (self.direction + val) % 360;
            }
            Forward(val) => {
                let translated_action = match self.direction {
                    0 => East(*val),
                    90 => South(*val),
                    180 => West(*val),
                    270 => North(*val),
                    x => panic!("Unknown direction: {}", x),
                };
                self.take_action(&translated_action);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    North(u16),
    South(u16),
    East(u16),
    West(u16),
    Left(u16),
    Right(u16),
    Forward(u16),
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Action::*;

        let action = s
            .chars()
            .next()
            .ok_or_else(|| format!("empty action: {}", s))?;
        let val: u16 = s[1..]
            .parse()
            .map_err(|e| format!("invalid action value: {} ({:?})", s, e))?;

        match action {
            'N' => Ok(North(val)),
            'S' => Ok(South(val)),
            'E' => Ok(East(val)),
            'W' => Ok(West(val)),
            'L' => Ok(Left(val)),
            'R' => Ok(Right(val)),
            'F' => Ok(Forward(val)),
            x => Err(format!("invalid action: {}", x)),
        }
    }
}
