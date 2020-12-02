use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let inputs: Vec<String> = stdin.lock().lines().map(|s| s.unwrap()).collect();
    let pieces: Vec<_> = inputs
        .iter()
        .map(|s| {
            let mut iter = s.split(':').map(|s| s.trim());
            let policy = iter.next().unwrap();
            let password = iter.next().unwrap();

            let mut iter = policy.split(' ');
            let count = iter.next().unwrap();
            let c = iter.next().unwrap();

            (count, c, password)
        })
        .collect();

    let part_1 = pieces.iter().fold(0, |acc, (count, target, password)| {
        let mut iter = count.split('-');
        let min: u32 = iter.next().unwrap().parse().unwrap();
        let max: u32 = iter.next().unwrap().parse().unwrap();

        let target_char = target.chars().next().unwrap();

        let num_present = password
            .chars()
            .fold(0, |acc, c| if c == target_char { acc + 1 } else { acc });

        let valid = min <= num_present && num_present <= max;

        if valid {
            acc + 1
        } else {
            acc
        }
    });

    let part_2 = pieces.iter().fold(0, |acc, (count, target, password)| {
        let indices: Vec<u32> = count.split('-').map(|x| x.parse().unwrap()).collect();
        let target_char = target.chars().next().unwrap();

        let valid = password.char_indices().fold(false, |acc, (i, c)| {
            if indices.contains(&((i + 1) as _)) && c == target_char {
                !acc
            } else {
                acc
            }
        });

        if valid {
            acc + 1
        } else {
            acc
        }
    });

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
