use std::{
    collections::BTreeMap,
    io::{self, BufRead},
    str::FromStr,
};

fn main() {
    let mut rules = Rules(BTreeMap::new());
    let mut section = Section::Rules;
    let mut messages = Vec::new();

    let mut num_matching_messages = 0;

    for line in io::stdin().lock().lines().filter_map(Result::ok) {
        if line.is_empty() {
            section = Section::Messages;
            continue;
        }

        match section {
            Section::Rules => {
                let mut iter = line.split(": ");
                let id: u16 = iter.next().unwrap().parse().unwrap();
                let rule = Rule::from_str(iter.next().unwrap()).unwrap();
                rules.0.insert(id, rule);
            }
            Section::Messages => {
                if rules.matches(&line).unwrap() {
                    num_matching_messages += 1;
                }
                messages.push(line);
            }
        }
    }

    println!("Part 1: {}", num_matching_messages);

    rules
        .0
        .insert(8, Rule::Indirect(vec![vec![42], vec![42, 8]]));
    rules
        .0
        .insert(11, Rule::Indirect(vec![vec![42, 31], vec![42, 11, 31]]));

    println!(
        "Part 2: {}",
        messages
            .iter()
            .fold(0, |acc, m| if rules.matches(m).unwrap() {
                acc + 1
            } else {
                acc
            })
    );
}

struct Rules(BTreeMap<u16, Rule>);

impl Rules {
    fn is_recursive(&self, rule_id: u16) -> bool {
        self.0.get(&rule_id).unwrap().is_recursive(rule_id)
    }
    fn matches(&self, message: &str) -> Result<bool, String> {
        let chars: Vec<char> = message.chars().collect();

        let (matches, remaining) = self.rule_matches(0, chars.as_slice())?;

        Ok(matches && remaining.is_empty())
    }

    fn rule_matches<'a, 'c>(
        &'a self,
        rule_id: u16,
        chars: &'c [char],
    ) -> Result<(bool, &'c [char]), String> {
        if chars.is_empty() {
            return Ok((false, chars));
        }

        let rule = self
            .0
            .get(&rule_id)
            .ok_or_else(|| format!("no rule {}", rule_id))?;

        match rule {
            Rule::Direct(c) => {
                if *c == chars[0] {
                    Ok((true, &chars[1..]))
                } else {
                    Ok((false, chars))
                }
            }
            Rule::Indirect(subrules) => {
                let subrule_matches: Vec<&[char]> = subrules
                    .iter()
                    .filter_map(|subrule| {
                        let mut remaining = chars;

                        let mut i = 0;
                        'subrule_id_loop: while i < subrule.len() {
                            let subrule_id = subrule[i];
                            if self.is_recursive(subrule_id) {
                                if let Some(next_rule) = subrule.get(i + 1) {
                                    for j in 1..remaining.len() {
                                        if let Ok((matches, rest)) =
                                            self.rule_matches(*next_rule, &remaining[j..])
                                        {
                                            if matches {
                                                if let Ok((
                                                    recursive_matches,
                                                    recursive_remaining,
                                                )) =
                                                    self.rule_matches(subrule_id, &remaining[..j])
                                                {
                                                    if recursive_matches
                                                        && recursive_remaining.is_empty()
                                                    {
                                                        remaining = rest;
                                                        i += 2;
                                                        continue 'subrule_id_loop;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if let Ok((matches, rest)) = self.rule_matches(subrule_id, &remaining) {
                                if !matches {
                                    return None;
                                }
                                remaining = rest;
                            }

                            i += 1;
                        }

                        Some(remaining)
                    })
                    .collect();

                if subrule_matches.is_empty() {
                    Ok((false, chars))
                } else {
                    Ok((
                        true,
                        subrule_matches
                            .into_iter()
                            .fold(None, |acc: Option<&[char]>, remaining: &[char]| {
                                if let Some(shortest) = acc {
                                    if remaining.len() < shortest.len() {
                                        Some(remaining)
                                    } else {
                                        Some(shortest)
                                    }
                                } else {
                                    Some(remaining)
                                }
                            })
                            .unwrap(),
                    ))
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Rule {
    Direct(char),
    Indirect(Vec<Vec<u16>>),
}

impl Rule {
    fn is_recursive(&self, rule_id: u16) -> bool {
        match self {
            Rule::Direct(_) => false,
            Rule::Indirect(subrules) => subrules.iter().any(|subrule| subrule.contains(&rule_id)),
        }
    }
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('"') {
            Ok(Rule::Direct(
                s.replace('"', "")
                    .chars()
                    .next()
                    .ok_or_else(|| format!("invalid direct rule: {}", s))?,
            ))
        } else {
            Ok(Rule::Indirect(
                s.split(" | ")
                    .map(|subrule| {
                        subrule
                            .split_whitespace()
                            .map(|component| {
                                component.parse::<u16>().map_err(|e| {
                                    format!("invalid subrule component: {} {:?}", component, e)
                                })
                            })
                            .collect::<Result<Vec<u16>, String>>()
                    })
                    .collect::<Result<Vec<Vec<u16>>, String>>()?,
            ))
        }
    }
}

enum Section {
    Rules,
    Messages,
}
