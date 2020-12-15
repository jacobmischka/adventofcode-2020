use std::{
    collections::BTreeMap,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut nums: BTreeMap<usize, usize> = input
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
        .enumerate()
        .map(|(i, x)| (x, i + 1))
        .collect();

    let mut last_num = 0; // Assuming starting numbers are all unique
    let mut i = nums.len() + 1;

    while i < 30000000 {
        let num = nums
            .get(&last_num)
            .map(|last_spoken| i - *last_spoken)
            .unwrap_or(0);
        nums.insert(last_num, i);
        last_num = num;
        i += 1;

        if i == 2020 {
            println!("Part 1: {}", last_num);
        }
    }

    println!("Part 2: {}", last_num);
}
