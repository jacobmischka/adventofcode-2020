use std::{
    collections::HashMap,
    io::{self, Read},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let stdin = io::stdin();

    let mut input = String::new();

    stdin.lock().read_to_string(&mut input).unwrap();

    let potential_passports: Vec<PotentialPassport> = input
        .split("\n\n")
        .map(PotentialPassport::from_str)
        .filter_map(Result::ok)
        .collect();

    let valid_passports: Vec<ValidPassport> = potential_passports
        .iter()
        .map(ValidPassport::from_potential)
        .filter_map(Result::ok)
        .collect();

    println!("Part 1: {}", potential_passports.len());
    println!("Part 2: {}", valid_passports.len());
}

struct PotentialPassport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl FromStr for PotentialPassport {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<&str, String> = HashMap::new();

        for piece in s.split_whitespace() {
            let mut iter = piece.split(':');
            let key = iter.next().ok_or("missing key")?;
            let val = iter.next().map(String::from).ok_or("missing val")?;
            map.insert(key, val);
        }

        Ok(PotentialPassport {
            byr: map.remove("byr").ok_or("missing byr")?,
            iyr: map.remove("iyr").ok_or("missing iyr")?,
            eyr: map.remove("eyr").ok_or("missing eyr")?,
            hgt: map.remove("hgt").ok_or("missing hgt")?,
            hcl: map.remove("hcl").ok_or("missing hcl")?,
            ecl: map.remove("ecl").ok_or("missing ecl")?,
            pid: map.remove("pid").ok_or("missing pid")?,
            cid: map.remove("cid"),
        })
    }
}

#[derive(Debug, Copy, Clone)]
enum LengthUnit {
    Cm,
    In,
}

impl FromStr for LengthUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cm" => Ok(LengthUnit::Cm),
            "in" => Ok(LengthUnit::In),
            x => Err(format!("Invalid unit: {}", x)),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Height {
    val: u16,
    unit: LengthUnit,
}

impl FromStr for Height {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref HEIGHT_RE: Regex = Regex::new(r"^(?P<value>\d+)(?P<unit>(cm|in))$").unwrap();
        }

        let caps = HEIGHT_RE
            .captures(s)
            .ok_or(format!("invalid height: {}", s))?;

        let unit = LengthUnit::from_str(
            caps.name("unit")
                .ok_or(format!("missing unit: {}", s))?
                .as_str(),
        )?;

        Ok(Height {
            val: caps
                .name("value")
                .ok_or(format!("missing value: {}", s))?
                .as_str()
                .parse()
                .map_err(|e| format!("invalid height value: {:?}", e))
                .and_then(|v| match unit {
                    LengthUnit::Cm => {
                        if 150 <= v && v <= 193 {
                            Ok(v)
                        } else {
                            Err(format!("invalid cm height value: {}", v))
                        }
                    }
                    LengthUnit::In => {
                        if 59 <= v && v <= 76 {
                            Ok(v)
                        } else {
                            Err(format!("invalid in height value: {}", v))
                        }
                    }
                })?,
            unit,
        })
    }
}

#[derive(Debug, Clone)]
struct Color(String);

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref COLOR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }

        if COLOR_RE.is_match(s) {
            Ok(Color(s.to_string()))
        } else {
            Err(format!("invalid color: {}", s))
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

impl FromStr for EyeColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use EyeColor::*;

        match s {
            "amb" => Ok(Amb),
            "blu" => Ok(Blu),
            "brn" => Ok(Brn),
            "gry" => Ok(Gry),
            "grn" => Ok(Grn),
            "hzl" => Ok(Hzl),
            "oth" => Ok(Oth),
            x => Err(format!("invalid eye color: {}", x)),
        }
    }
}

#[derive(Debug, Clone)]
struct PassportId(String);

impl FromStr for PassportId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PASSPORT_ID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        if PASSPORT_ID_RE.is_match(s) {
            Ok(PassportId(s.to_string()))
        } else {
            Err(format!("invalid Passport ID: {}", s))
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ValidPassport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: Height,
    hcl: Color,
    ecl: EyeColor,
    pid: PassportId,
    cid: Option<String>,
}

fn check_date(s: &str) -> Result<&str, String> {
    lazy_static! {
        static ref DATE_RE: Regex = Regex::new(r"^\d{4}$").unwrap();
    }

    if DATE_RE.is_match(s) {
        Ok(s)
    } else {
        Err(format!("invalid date: {}", s))
    }
}

impl ValidPassport {
    fn from_potential(p: &PotentialPassport) -> Result<Self, String> {
        Ok(ValidPassport {
            byr: check_date(&p.byr)?
                .parse()
                .ok()
                .and_then(|v| {
                    if 1920 <= v && v <= 2002 {
                        Some(v)
                    } else {
                        None
                    }
                })
                .ok_or(format!("invalid byr: {}", &p.byr))?,
            iyr: check_date(&p.iyr)?
                .parse()
                .ok()
                .and_then(|v| {
                    if 2010 <= v && v <= 2020 {
                        Some(v)
                    } else {
                        None
                    }
                })
                .ok_or(format!("invalid iyr: {}", &p.iyr))?,
            eyr: check_date(&p.eyr)?
                .parse()
                .ok()
                .and_then(|v| {
                    if 2020 <= v && v <= 2030 {
                        Some(v)
                    } else {
                        None
                    }
                })
                .ok_or(format!("invalid eyr: {}", &p.eyr))?,
            hgt: Height::from_str(&p.hgt)?,
            hcl: Color::from_str(&p.hcl)?,
            ecl: EyeColor::from_str(&p.ecl)?,
            pid: PassportId::from_str(&p.pid)?,
            cid: p.cid.clone(),
        })
    }
}
