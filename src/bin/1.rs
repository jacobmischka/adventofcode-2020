use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let inputs: Vec<u32> = stdin
        .lock()
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    let mut pairs = inputs
        .iter()
        .flat_map(|&x| inputs.iter().map(move |&y| (x, y)));

    let mut triples = pairs
        .clone()
        .flat_map(|(x, y)| inputs.iter().map(move |&z| (x, y, z)));

    let part1 = pairs
        .find_map(|(x, y)| if x + y == 2020 { Some(x * y) } else { None })
        .unwrap();

    let part2 = triples
        .find_map(|(x, y, z)| {
            if x + y + z == 2020 {
                Some(x * y * z)
            } else {
                None
            }
        })
        .unwrap();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
