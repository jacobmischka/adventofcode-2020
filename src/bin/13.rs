use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut lines = stdin.lock().lines();
    let earliest_timestamp: u32 = lines.next().unwrap().unwrap().parse().unwrap();
    let bus_ids: Vec<Option<u32>> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(str::parse::<u32>)
        .map(Result::ok)
        .collect();

    let earliest_float = earliest_timestamp as f64;

    let (min_id, min_diff) = bus_ids
        .iter()
        .filter_map(|x| {
            x.map(|x| {
                let mult = (earliest_float / x as f64).ceil();
                (x, (mult as u32 * x) - earliest_timestamp)
            })
        })
        .fold(None, |acc, (id, diff)| match acc {
            Some((_min_id, min_diff)) => {
                if diff < min_diff {
                    Some((id, diff))
                } else {
                    acc
                }
            }
            None => Some((id, diff)),
        })
        .unwrap();

    let part_1 = min_id * min_diff;

    println!("Part 1: {}", part_1);

    let offsets_and_ids: Vec<_> = bus_ids
        .iter()
        .enumerate()
        .filter_map(|(offset, id)| id.map(|id| (offset as u128, id as u128)))
        .collect();

    // TODO: I'd like to actually do the math to unify myself here, but too lazy right now
    println!("Part 2: Stick this boy into WolframAlpha, lol");
    for (offset, id) in offsets_and_ids.iter() {
        print!("((x + {}) mod {}) = ", offset, id);
    }
    println!("0");

    // Naive solution for examples
    //
    // let (max_id_offset, max_id) =
    //     offsets_and_ids
    //         .iter()
    //         .fold((0, 0), |acc, pair| if pair.1 > acc.1 { *pair } else { acc });
    //
    // let mut max_relative_t = max_id;
    // loop {
    //     if offsets_and_ids
    //         .iter()
    //         .all(|(offset, id)| (max_relative_t - max_id_offset + offset) % *id == 0)
    //     {
    //         break;
    //     }
    //
    //     max_relative_t += max_id;
    // }
    //
    // let part_2 = max_relative_t - max_id_offset;
    // println!("Part 2: {}", part_2);
}
