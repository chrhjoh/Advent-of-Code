use aocsuite::utils;
use std::collections::{HashMap, HashSet};
fn exercise1(data: &str) -> i64 {
    let input = parse_input(data);
    let allergens = all_allergens(&input);
    let mut allergen_translation = HashSet::new();
    for allergen in &allergens {
        let possible = possible_translation(&input, allergen);
        println!("{}: {:?}", allergen, possible);
        allergen_translation = allergen_translation.union(&possible).cloned().collect();
    }
    let all_ingredients: HashSet<String> = input
        .iter()
        .flat_map(|line| line.ingredients.iter())
        .cloned()
        .collect();

    let safe_ingredients = all_ingredients
        .difference(&allergen_translation)
        .cloned()
        .collect::<HashSet<String>>();

    input
        .iter()
        .map(|line| {
            line.ingredients
                .iter()
                .filter(|ingredient| safe_ingredients.contains(*ingredient))
                .count()
        })
        .sum::<usize>() as i64
}
fn exercise2(data: &str) -> i64 {
    let input = parse_input(data);
    let translation = resolve_translation(&input);
    // order the allergens alphabetically
    let mut allergens: Vec<String> = translation.keys().cloned().collect();
    allergens.sort();
    let result = allergens
        .iter()
        .map(|allergen| translation.get(allergen).unwrap().to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("{:?}", result);
    return 0;
}

fn main() {
    utils::run(exercise1, exercise2);
}

struct InputLine {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

fn parse_input(data: &str) -> Vec<InputLine> {
    let mut result = Vec::new();
    for line in data.lines() {
        let mut parts = line.split(" (contains ");
        let ingredients = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        let allergens = parts
            .next()
            .unwrap()
            .replace(")", "")
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        result.push(InputLine {
            ingredients,
            allergens,
        });
    }
    return result;
}

fn all_allergens(data: &Vec<InputLine>) -> Vec<String> {
    let mut result = Vec::new();
    for line in data {
        for allergen in &line.allergens {
            if !result.contains(allergen) {
                result.push(allergen.clone());
            }
        }
    }
    return result;
}

fn possible_translation(data: &Vec<InputLine>, allergen: &String) -> HashSet<String> {
    let mut result = HashSet::new();
    for line in data {
        if line.allergens.contains(allergen) {
            if result.is_empty() {
                result = line.ingredients.iter().cloned().collect();
            } else {
                result = result
                    .intersection(&line.ingredients.iter().cloned().collect())
                    .cloned()
                    .collect();
            }
        }
    }
    return result;
}

fn resolve_translation(data: &Vec<InputLine>) -> HashMap<String, String> {
    let allergens = all_allergens(data);
    let mut allergen_translation: HashMap<String, HashSet<String>> = HashMap::new();
    allergens.iter().for_each(|allergen| {
        allergen_translation.insert(allergen.clone(), possible_translation(data, allergen));
    });
    while allergen_translation.iter().any(|(_, v)| v.len() > 1) {
        let resolved: HashSet<String> = allergen_translation
            .iter()
            .filter(|(_, v)| v.len() == 1)
            .map(|(_, v)| v.iter().next().unwrap().clone())
            .collect();
        allergen_translation.iter_mut().for_each(|(_, v)| {
            if v.len() > 1 {
                *v = v.difference(&resolved).cloned().collect();
            }
        });
    }
    return allergen_translation
        .iter()
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.insert(k.clone(), v.iter().next().unwrap().clone());
            acc
        });
}
