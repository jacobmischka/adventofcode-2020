use std::io::{self, BufRead};

const PREV_NUM_LOOKUP_LEN: usize = 25;

fn main() {
    let inputs: Vec<u64> = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let nonsum = get_first_nonsum(&inputs).unwrap();
    let contiguous = get_contiguous(&inputs, nonsum).unwrap();

    println!("Part 1: {}", nonsum);
    println!(
        "Part 2: {}",
        contiguous.iter().min().unwrap() + contiguous.iter().max().unwrap()
    );
}

fn get_first_nonsum(inputs: &Vec<u64>) -> Option<u64> {
    for (i, val) in inputs.iter().enumerate().skip(PREV_NUM_LOOKUP_LEN) {
        if pairs(inputs[i - PREV_NUM_LOOKUP_LEN..i].iter()).all(|(x, y)| x + y != *val) {
            return Some(*val);
        }
    }

    None
}

fn get_contiguous(inputs: &Vec<u64>, val: u64) -> Option<&[u64]> {
    for i in 1..inputs.len() {
        let mut j = i.checked_sub(1);
        while let Some(j_val) = j {
            let contiguous = &inputs[j_val..i];
            let sum = contiguous.iter().fold(0, |acc, x| acc + x);
            if sum == val {
                return Some(contiguous);
            } else if sum > val {
                break;
            }
            j = j_val.checked_sub(1);
        }
    }

    None
}

fn pairs<I, T>(iter: I) -> impl Iterator<Item = (T, T)>
where
    I: Iterator<Item = T> + Clone,
    T: Copy,
{
    iter.clone().enumerate().flat_map(move |(i, x)| {
        iter.clone()
            .enumerate()
            .filter_map(move |(j, y)| if i != j { Some((x, y)) } else { None })
    })
}
