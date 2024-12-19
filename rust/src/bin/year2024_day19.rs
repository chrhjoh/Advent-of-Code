use aocsuite::utils;
use std::collections::HashMap;

fn exercise1(data: &str) -> i64 {
    let mut sections = data.split("\n\n");
    let patterns: Vec<&str> = sections.next().unwrap().split(", ").collect();
    let mut memo = HashMap::new();

    sections
        .next()
        .unwrap()
        .lines()
        .filter(|design| can_construct(design, &patterns, &mut memo) != 0)
        .count() as i64
}
fn exercise2(data: &str) -> i64 {
    let mut sections = data.split("\n\n");
    let mut patterns: Vec<&str> = sections.next().unwrap().split(", ").collect();
    let mut memo = HashMap::new();
    patterns.sort_by_key(|p| p.len());

    sections
        .next()
        .unwrap()
        .lines()
        .map(|design| can_construct(design, &patterns, &mut memo) as i64)
        .sum()
}

fn main() {
    utils::run(exercise1, exercise2);
}

fn can_construct(design: &str, patterns: &Vec<&str>, memo: &mut HashMap<String, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(&count) = memo.get(design) {
        return count;
    }
    let mut count = 0;
    for pattern in patterns {
        if design.starts_with(pattern) {
            count += can_construct(&design[pattern.len()..], patterns, memo);
        }
    }
    memo.insert(design.to_string(), count);
    count
}
