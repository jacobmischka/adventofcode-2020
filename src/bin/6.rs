use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

fn main() {
    let stdin = io::stdin();

    let mut input = String::new();

    stdin.lock().read_to_string(&mut input).unwrap();

    let mut part_1 = 0;
    let mut part_2 = 0;
    for group_answers in input.split("\n\n") {
        let person_yeses: HashMap<usize, HashSet<char>> = group_answers
            .lines()
            .enumerate()
            .map(|(i, person_answers)| (i, person_answers.chars().collect::<HashSet<_>>()))
            .collect();

        let anyone_yeses: HashSet<&char> = person_yeses.values().flatten().collect();

        part_1 += anyone_yeses.len();
        part_2 += anyone_yeses.iter().fold(0, |acc, q| {
            if person_yeses.values().all(|answers| answers.contains(q)) {
                acc + 1
            } else {
                acc
            }
        });
    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
