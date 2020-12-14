use lazy_static::lazy_static;
use regex::Regex;

use std::{
    collections::BTreeMap,
    io::{self, BufRead},
    num::ParseIntError,
    str::FromStr,
};

fn main() {
    let stdin = io::stdin();
    let iter = stdin.lock().lines().filter_map(Result::ok);
    let mut computer_v1 = ComputerV1::new().unwrap();
    let mut computer_v2 = ComputerV2::new().unwrap();

    for line in iter {
        computer_v1.parse_line(&line).unwrap();
        computer_v2.parse_line(&line).unwrap();
    }

    let part_1: u128 = computer_v1
        .memory
        .values()
        .fold(0u128, |acc, x| acc + *x as u128);
    let part_2: u128 = computer_v2
        .memory
        .values()
        .fold(0u128, |acc, x| acc + *x as u128);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

struct ComputerV1 {
    mask: Option<MaskV1>,
    memory: BTreeMap<u64, u64>,
}

fn parse_mask_line(s: &str) -> Result<&str, String> {
    s.split_whitespace()
        .skip(2)
        .next()
        .ok_or_else(|| format!("invalid mask line: {}", s))
}

fn parse_mem_line(s: &str) -> Result<(u64, u64), String> {
    lazy_static! {
        static ref MEM_RE: Regex = Regex::new(r"mem\[(?P<addr>\d+)\] = (?P<val>\d+)").unwrap();
    }

    let caps = MEM_RE
        .captures(s)
        .ok_or_else(|| format!("invalid mem line: {}", s))?;
    let addr: u64 = caps
        .name("addr")
        .ok_or_else(|| format!("missing addr: {}", s))
        .map(|addr| addr.as_str())
        .and_then(|addr: &str| {
            addr.parse()
                .map_err(|e| format!("invalid addr: {} {:?}", addr, e))
        })?;
    let val: u64 = caps
        .name("val")
        .ok_or_else(|| format!("missing val: {}", s))
        .map(|val| val.as_str())
        .and_then(|val: &str| {
            val.parse()
                .map_err(|e| format!("invalid val: {} {:?}", val, e))
        })?;

    Ok((addr, val))
}

impl ComputerV1 {
    fn new() -> Result<Self, ParseIntError> {
        Ok(ComputerV1 {
            mask: None,
            memory: BTreeMap::new(),
        })
    }

    fn parse_line(&mut self, s: &str) -> Result<(), String> {
        if s.starts_with("mask") {
            self.mask = Some(
                MaskV1::from_str(parse_mask_line(s)?)
                    .map_err(|e| format!("invalid mask value: {}, {:?}", s, e))?,
            );
        } else {
            let (addr, val) = parse_mem_line(s)?;

            self.memory.insert(
                addr,
                self.mask
                    .as_ref()
                    .map(|mask| mask.apply(val))
                    .unwrap_or(val),
            );
        }

        Ok(())
    }
}

struct MaskV1 {
    and_val: u64,
    or_val: u64,
}

impl FromStr for MaskV1 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MaskV1 {
            and_val: u64::from_str_radix(&s.replace('X', "1"), 2)?,
            or_val: u64::from_str_radix(&s.replace('X', "0"), 2)?,
        })
    }
}

impl MaskV1 {
    fn apply(&self, val: u64) -> u64 {
        val & self.and_val | self.or_val
    }
}

struct MaskV2 {
    and_mask: u64,
    or_mask: u64,
    floating_masks: Vec<u64>,
}

impl FromStr for MaskV2 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let or_mask = u64::from_str_radix(&s.replace('X', "0"), 2)?;
        let and_mask = u64::from_str_radix(&s.replace('0', "1").replace('X', "0"), 2)?;
        let len = s.len() - 1;
        let floating_bitmasks: Vec<u64> = s
            .match_indices('X')
            .map(|(i, _)| 1 << (len - i as usize))
            .collect();
        let mut floating_masks = Vec::with_capacity(2usize.pow(floating_bitmasks.len() as u32));

        for i in 0..floating_masks.capacity() {
            let mut val = 0;

            for k in 0..floating_bitmasks.len() {
                if (i >> k) & 1 == 1 {
                    val |= floating_bitmasks[k];
                }
            }

            floating_masks.push(val);
        }

        Ok(MaskV2 {
            and_mask,
            or_mask,
            floating_masks,
        })
    }
}

struct ComputerV2 {
    mask: Option<MaskV2>,
    memory: BTreeMap<u64, u64>,
}

impl ComputerV2 {
    fn new() -> Result<Self, ParseIntError> {
        Ok(ComputerV2 {
            mask: None,
            memory: BTreeMap::new(),
        })
    }

    fn parse_line(&mut self, s: &str) -> Result<(), String> {
        if s.starts_with("mask") {
            self.mask = Some(
                MaskV2::from_str(parse_mask_line(s)?)
                    .map_err(|e| format!("invalid mask value: {}, {:?}", s, e))?,
            );
        } else {
            let (addr, val) = parse_mem_line(s)?;

            let mask = self.mask.as_ref().ok_or("No mask")?;

            let base = (addr | mask.or_mask) & mask.and_mask;

            for masked_addr in mask.floating_masks.iter().map(|m| m | base) {
                self.memory.insert(masked_addr, val);
            }
        }

        Ok(())
    }
}
