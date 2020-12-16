use std::{
    collections::{BTreeMap, HashSet},
    io::{self, BufRead},
    iter,
    str::FromStr,
};

fn main() {
    let mut section = InputSection::Rules;

    let mut rules: Vec<TicketRule> = Vec::new();
    let mut my_ticket = None;
    let mut nearby_tickets: Vec<Ticket> = Vec::new();

    for line in io::stdin().lock().lines().filter_map(Result::ok) {
        match line.as_str() {
            "" => {
                continue;
            }
            "your ticket:" => {
                section = InputSection::MyTicket;
            }
            "nearby tickets:" => {
                section = InputSection::NearbyTickets;
            }
            s => match section {
                InputSection::Rules => {
                    rules.push(TicketRule::from_str(s).unwrap());
                }
                InputSection::MyTicket => {
                    my_ticket = Ticket::from_str(s).ok();
                }
                InputSection::NearbyTickets => {
                    nearby_tickets.push(Ticket::from_str(s).unwrap());
                }
            },
        }
    }

    let rules = rules;
    let my_ticket = my_ticket.unwrap();
    let nearby_tickets = nearby_tickets;

    let mut valid_tickets = Vec::new();

    let invalid_values: Vec<_> = nearby_tickets
        .iter()
        .flat_map(|ticket| {
            let invalid_vals: Vec<_> = ticket
                .nums
                .iter()
                .filter(|n| !rules.iter().any(|rule| rule.matches(**n)))
                .collect();

            if invalid_vals.is_empty() {
                valid_tickets.push(ticket);
            }
            invalid_vals
        })
        .copied()
        .collect();

    let ticket_scanning_error_rate: u16 = invalid_values.into_iter().sum();

    println!("Part 1: {}", ticket_scanning_error_rate);

    let mut possible_col_rule_map: BTreeMap<usize, Vec<&TicketRule>> = BTreeMap::new();

    for (i, num) in my_ticket.nums.iter().enumerate() {
        for rule in &rules {
            if rule.matches(*num) {
                possible_col_rule_map.entry(i).or_default().push(rule);
            }
        }
    }

    for ticket in &valid_tickets {
        for (i, num) in ticket.nums.iter().enumerate() {
            let possible_rules = possible_col_rule_map.get_mut(&i).unwrap();
            possible_rules.retain(|rule| rule.matches(*num));
        }
    }

    let mut col_rule_map: BTreeMap<usize, &TicketRule> = BTreeMap::new();

    let mut claimed_rules: HashSet<&str> = HashSet::new();

    while col_rule_map.len() < possible_col_rule_map.len() {
        for (col, rules) in possible_col_rule_map.iter_mut() {
            rules.retain(|rule| !claimed_rules.contains(rule.name.as_str()));

            if !col_rule_map.contains_key(col) && rules.len() == 1 {
                let claimed_rule = rules.remove(0);
                claimed_rules.insert(&claimed_rule.name);
                col_rule_map.insert(*col, claimed_rule);
                break;
            }
        }
    }

    let part_2 = col_rule_map
        .iter()
        .filter_map(|(col, rule)| {
            if rule.name.starts_with("departure") {
                Some(*col)
            } else {
                None
            }
        })
        .map(|col| my_ticket.nums[col])
        .fold(1u128, |acc, x| acc * x as u128);

    println!("Part 2: {}", part_2);
}

enum InputSection {
    Rules,
    MyTicket,
    NearbyTickets,
}

#[derive(Debug, Clone)]
struct TicketRule {
    name: String,
    ranges: Vec<(u16, u16)>,
}

impl TicketRule {
    fn matches(&self, val: u16) -> bool {
        self.ranges
            .iter()
            .any(|range| range.0 <= val && val <= range.1)
    }
}

impl FromStr for TicketRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut halves = s.split(':').map(|s| s.trim());
        let name = halves
            .next()
            .ok_or_else(|| format!("missing rule name: {}", s))?
            .to_string();
        let ranges: Vec<_> = halves
            .next()
            .ok_or_else(|| format!("missing rule ranges: {}", s))?
            .split(" or ")
            .map(|s| {
                let mut iter = s.split('-');
                let start = iter
                    .next()
                    .ok_or(format!("invalid range: {}", s))?
                    .parse()
                    .map_err(|e| format!("invalid range start: {} {:?}", s, e))?;
                let end = iter
                    .next()
                    .ok_or(format!("invalid range: {}", s))?
                    .parse()
                    .map_err(|e| format!("invalid range end: {} {:?}", s, e))?;

                Ok((start, end))
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(TicketRule { name, ranges })
    }
}

#[derive(Debug, Clone)]
struct Ticket {
    nums: Vec<u16>,
}

impl FromStr for Ticket {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ticket {
            nums: s
                .split(',')
                .map(|n| {
                    n.parse()
                        .map_err(|e| format!("invalid ticket number: {} ({})", n, s))
                })
                .collect::<Result<Vec<_>, String>>()?,
        })
    }
}
