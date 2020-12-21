use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let foods: Vec<Food> = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| Food::from_str(&line))
        .collect::<Result<Vec<Food>, String>>()
        .unwrap();

    let mut allergen_map: HashMap<&str, Vec<&Food>> = HashMap::new();

    for food in &foods {
        for allergen in &food.allergens {
            allergen_map
                .entry(allergen.as_str())
                .or_default()
                .push(food);
        }
    }

    let mut allergen_ingredients: HashMap<&str, Vec<&String>> = allergen_map
        .iter()
        .map(|(allergen, foods)| {
            let mut set: HashSet<&String> = foods[0].ingredients.iter().collect();
            for food in foods.iter().skip(1) {
                set = set
                    .intersection(&food.ingredients.iter().collect())
                    .copied()
                    .collect();
            }

            (*allergen, set.into_iter().collect())
        })
        .collect();

    let mut claimed: HashSet<&String> = HashSet::new();

    while allergen_ingredients
        .values()
        .any(|ingredients| ingredients.len() > 1)
    {
        for ingredients in allergen_ingredients.values_mut() {
            if ingredients.len() > 1 {
                ingredients.retain(|i| !claimed.contains(i));
            }

            if ingredients.len() == 1 {
                if !claimed.contains(&ingredients[0]) {
                    claimed.insert(ingredients[0]);
                }
            }
        }
    }

    let mut part_1 = 0;
    for food in &foods {
        for ingredient in &food.ingredients {
            if !claimed.contains(ingredient) {
                part_1 += 1;
            }
        }
    }

    println!("Part 1: {}", part_1);

    let mut allergen_ingredients: Vec<(&str, &String)> = allergen_ingredients
        .drain()
        .map(|(allergen, ingredients)| (allergen, ingredients[0]))
        .collect();
    allergen_ingredients.sort_by_key(|(a, _)| *a);
    let canonical_dangerous_ingredient_list: String = allergen_ingredients
        .into_iter()
        .map(|(_, i)| i.as_str())
        .collect::<Vec<_>>()
        .join(",");

    println!("Part 2: {}", canonical_dangerous_ingredient_list);
}

#[derive(Debug, Clone)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl FromStr for Food {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref FOOD_RE: Regex =
                Regex::new(r"(?P<ingredients>.+) \(contains (?P<allergens>.+)\)").unwrap();
        }

        let caps = FOOD_RE.captures(s).ok_or(format!("invalid food: {}", s))?;

        Ok(Food {
            ingredients: caps
                .name("ingredients")
                .ok_or(format!("no ingredients found: {}", s))?
                .as_str()
                .split_whitespace()
                .map(String::from)
                .collect(),
            allergens: caps
                .name("allergens")
                .ok_or(format!("no allergens found: {}", s))?
                .as_str()
                .split(", ")
                .map(String::from)
                .collect(),
        })
    }
}
