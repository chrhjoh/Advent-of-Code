use aocsuite::utils;
use std::collections::{HashMap, HashSet};
fn exercise1(data: &str) -> i64 {
    let bags = build_graph(&data);
    let parents = find_unique_number_of_parents(&bags, "shiny gold");
    return parents;
}
fn exercise2(data: &str) -> i64 {
    let bags = build_graph(&data);
    let bags_inside = count_bags(&bags, "shiny gold") - 1; // -1 because f*ck recursion
    return bags_inside;
}

fn main() {
    utils::run(exercise1, exercise2);
}

fn build_graph(data: &str) -> HashMap<String, Vec<Bag>> {
    let mut bags = HashMap::new();

    for line in data.lines() {
        let parts: Vec<&str> = line.split(" bags contain ").collect();
        let parent = parts[0].to_string();
        let contained = parts[1];

        if contained != "no other bags." {
            let contained_bags: Vec<&str> = contained.split(", ").collect();
            let mut child_bags = vec![];
            for bag in contained_bags {
                let words: Vec<&str> = bag.split_whitespace().collect();
                let count: u32 = words[0].parse().unwrap();
                let contained_bag = format!("{} {}", words[1], words[2]);
                child_bags.push(Bag::new(contained_bag, count));
            }
            bags.entry(parent).or_insert(child_bags);
        }
    }
    return bags;
}

fn find_unique_number_of_parents(graph: &HashMap<String, Vec<Bag>>, start_bag: &str) -> i64 {
    let mut stack = vec![start_bag.to_string()];
    let mut parents = HashSet::new();
    while let Some(bag) = stack.pop() {
        if let Some(parent_bags) = graph.get(&bag) {
            for parent_bag in parent_bags {
                stack.push(parent_bag.name.to_string());
                parents.insert(parent_bag.name.to_string());
            }
        }
    }

    return parents.len() as i64;
}

fn count_bags(graph: &HashMap<String, Vec<Bag>>, start_bag: &str) -> i64 {
    if let Some(inside_bags) = graph.get(start_bag) {
        return inside_bags
            .iter()
            .map(|bag| bag.count as i64 * count_bags(graph, &bag.name))
            .sum::<i64>()
            + 1;
    }
    return 1;
}
#[derive(Debug)]
struct Bag {
    name: String,
    count: u32,
}

impl Bag {
    fn new(name: String, count: u32) -> Self {
        Bag { name, count }
    }
}
