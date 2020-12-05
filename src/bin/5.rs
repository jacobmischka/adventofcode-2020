use std::{
    io::{self, BufRead},
    str::FromStr,
};

fn main() {
    let stdin = io::stdin();

    let seats: Vec<_> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|s| Seat::from_str(&s).ok())
        .collect();

    let part_1 = seats.iter().fold(0, |acc, s| acc.max(s.id()));

    let mut filled_seats = [[false; MAX_COL as usize + 1]; MAX_ROW as usize + 1];
    for seat in seats.iter() {
        let filled_row: &mut [bool; MAX_COL as usize + 1] = &mut filled_seats[seat.row() as usize];
        filled_row[seat.col() as usize] = true;
    }

    let mut my_seat: Option<Seat> = None;

    for i in 0..filled_seats.len() {
        let unfilled = filled_seats[i].iter().position(|x| !x);

        if unfilled.is_some()
            && i > 0
            && filled_seats[i - 1].iter().all(|x| *x)
            && i < filled_seats.len()
            && filled_seats[i + 1].iter().all(|x| *x)
        {
            my_seat = Some(Seat(i as u8, unfilled.unwrap() as u8));
        }
    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", my_seat.unwrap().id());
}

const MAX_ROW: u8 = 127;
const MAX_COL: u8 = 7;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Seat(u8, u8);

impl Seat {
    fn row(&self) -> u8 {
        self.0
    }
    fn col(&self) -> u8 {
        self.1
    }

    fn id(&self) -> u32 {
        self.0 as u32 * 8u32 + self.1 as u32
    }
}

impl FromStr for Seat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row_iter = s.chars().take(7);
        let col_iter = s.chars().skip(7);

        let mut row_bounds = Bounds(0, MAX_ROW);
        for inst in row_iter {
            match inst {
                'F' => row_bounds = row_bounds.lower(),
                'B' => row_bounds = row_bounds.upper(),
                _ => {}
            }
        }

        let mut col_bounds = Bounds(0, MAX_COL);
        for inst in col_iter {
            match inst {
                'L' => col_bounds = col_bounds.lower(),
                'R' => col_bounds = col_bounds.upper(),
                _ => {}
            }
        }

        Ok(Seat(row_bounds.to_val()?, col_bounds.to_val()?))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Bounds(u8, u8);

impl Bounds {
    fn mid(&self) -> f64 {
        (self.1 as f64 - self.0 as f64) / 2.0
    }

    fn lower(&self) -> Bounds {
        Bounds(self.0, self.1 - self.mid().ceil() as u8)
    }

    fn upper(&self) -> Bounds {
        Bounds(self.0 + self.mid().ceil() as u8, self.1)
    }

    fn to_val(&self) -> Result<u8, String> {
        if self.0 == self.1 {
            Ok(self.0)
        } else {
            Err(format!("bounds not unified: {:?}", &self))
        }
    }
}

#[test]
fn bounds_works() {
    assert_eq!(Bounds(0, 127).lower(), Bounds(0, 63));
    assert_eq!(Bounds(0, 63).upper(), Bounds(32, 63));
    assert_eq!(Bounds(32, 63).lower(), Bounds(32, 47));
    assert_eq!(Bounds(32, 47).upper(), Bounds(40, 47));
    assert_eq!(Bounds(40, 47).upper(), Bounds(44, 47));
    assert_eq!(Bounds(44, 47).lower(), Bounds(44, 45));
}

#[test]
fn seat_works() {
    assert_eq!(Seat::from_str("FBFBBFFRLR").unwrap(), Seat(44, 5));

    let s = Seat::from_str("BFFFBBFRRR").unwrap();
    assert_eq!(s, Seat(70, 7));
    assert_eq!(s.id(), 567);

    let s = Seat::from_str("FFFBBBFRRR").unwrap();
    assert_eq!(s, Seat(14, 7));
    assert_eq!(s.id(), 119);

    let s = Seat::from_str("BBFFBBFRLL").unwrap();
    assert_eq!(s, Seat(102, 4));
    assert_eq!(s.id(), 820);
}
