use std::{
    collections::BTreeMap,
    io::{self, BufRead},
    iter,
};

fn main() {
    let mut adapters_jolts: Vec<u16> = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|s| s.parse::<u16>().ok())
        .chain(iter::once(0))
        .collect();

    adapters_jolts.sort_unstable();
    let adapters_jolts = adapters_jolts;

    let mut differences: BTreeMap<u8, u16> = BTreeMap::new();

    for i in 1..adapters_jolts.len() {
        let diff = adapters_jolts[i] - adapters_jolts[i - 1];
        *differences.entry(diff as u8).or_default() += 1;
    }

    // Built-in difference
    *differences.entry(3).or_default() += 1;

    let part_1 = differences.get(&1).unwrap() * differences.get(&3).unwrap();
    println!("Part 1: {}", part_1);

    let mut edges: BTreeMap<u16, Vec<u16>> = BTreeMap::new();
    for i in (1..adapters_jolts.len()).rev() {
        let this = adapters_jolts[i];
        let entry = edges.entry(this).or_insert_with(|| Vec::with_capacity(3));
        for j in 1..=3 {
            if let Some(other_index) = i.checked_sub(j) {
                let other = adapters_jolts[other_index];
                if this - other <= 3 {
                    entry.push(other);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    let mut lookup: BTreeMap<u16, u128> = BTreeMap::new();
    lookup.insert(0, 1);
    println!(
        "Part 2: {}",
        count_inpaths(&mut lookup, &edges, *adapters_jolts.last().unwrap()).unwrap()
    );
}

fn count_inpaths(
    lookup: &mut BTreeMap<u16, u128>,
    edges: &BTreeMap<u16, Vec<u16>>,
    node: u16,
) -> Option<u128> {
    lookup.get(&node).copied().or_else(|| {
        edges.get(&node).map(|froms| {
            let val = froms.iter().fold(0, |acc, from| {
                if let Some(inpaths) = count_inpaths(lookup, edges, *from) {
                    acc + inpaths
                } else {
                    acc
                }
            });

            lookup.insert(node, val);

            val
        })
    })
}
