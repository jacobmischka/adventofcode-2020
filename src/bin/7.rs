use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn main() {
    let stdin = io::stdin();

    let mut child_map: HashMap<BagType, Vec<(u32, BagType)>> = HashMap::new();
    let mut parent_map: HashMap<BagType, Vec<BagType>> = HashMap::new();

    let lines: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();

    for line in lines.iter() {
        let (_, (src_bag, dest_bags)) = input_line(line.as_str()).unwrap();
        for (_, dest_bag) in dest_bags.iter() {
            parent_map.entry(*dest_bag).or_default().push(src_bag);
        }
        child_map.insert(src_bag, dest_bags);
    }

    let our_bag = BagType("shiny", "gold");
    let outermost = get_outermost(&parent_map, &our_bag);
    let inside = count_inside(&child_map, &our_bag);

    println!("Part 1: {}", outermost.len());
    println!("Part 2: {}", inside);
}

fn get_outermost<'a, 'b, 'c>(
    parent_map: &'b HashMap<BagType<'a>, Vec<BagType<'a>>>,
    child: &'c BagType<'a>,
) -> HashSet<BagType<'a>> {
    let mut set = HashSet::new();

    if let Some(parents) = parent_map.get(child) {
        for p in parents.iter() {
            set.insert(*p);
        }

        let next: HashSet<_> = parents
            .iter()
            .flat_map(|p| get_outermost(parent_map, p))
            .collect();

        set.union(&next).copied().collect()
    } else {
        set
    }
}

fn count_inside(child_map: &HashMap<BagType, Vec<(u32, BagType)>>, bag: &BagType) -> u128 {
    if let Some(children) = child_map.get(bag) {
        children.iter().fold(0, |acc, (count, bag)| {
            acc + *count as u128 * (1u128 + count_inside(child_map, bag))
        })
    } else {
        0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BagType<'a>(&'a str, &'a str);

impl<'a> BagType<'a> {
    fn from_tuple(((a, _, b), _, _): ((&'a str, &'a str, &'a str), &'a str, &'a str)) -> Self {
        BagType(a, b)
    }
}

fn input_line(i: &str) -> IResult<&str, (BagType, Vec<(u32, BagType)>)> {
    let contain = tag("contain");
    let end = tag(".");

    let (input, (src_bag, _, _, _, dest_bags, _)) =
        tuple((bag_def, tag(" "), contain, tag(" "), possible_bags_def, end))(i)?;

    Ok((input, (BagType::from_tuple(src_bag), dest_bags)))
}

fn bag_def(s: &str) -> IResult<&str, ((&str, &str, &str), &str, &str)> {
    let bag_type = tuple((take_while(|c| c != ' '), tag(" "), take_while(|c| c != ' ')));
    let bag_word = alt((tag("bags"), tag("bag")));

    tuple((bag_type, tag(" "), bag_word))(s)
}

fn possible_bags_def(s: &str) -> IResult<&str, Vec<(u32, BagType)>> {
    let num = take_while(|c: char| c.is_numeric());
    let numbered_bag_def = tuple((num, tag(" "), bag_def));
    let mut bags_def = separated_list1(tag(", "), numbered_bag_def);

    if s.contains("no other bags") {
        Ok((".", Vec::new()))
    } else {
        let (input, defs) = bags_def(s)?;

        Ok((
            input,
            defs.iter()
                .map(|(num, _, b)| (num.parse::<u32>().unwrap(), BagType::from_tuple(*b)))
                .collect(),
        ))
    }
}
