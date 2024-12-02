use aocsuite::utils;
use std::collections::HashMap;
fn exercise1(data: &str) -> i64 {
    let mut line1: Vec<i64> = data
        .lines()
        .map(|x| x.split("   ").nth(0).unwrap().parse().unwrap())
        .collect();
    let mut line2: Vec<i64> = data
        .lines()
        .map(|x| x.split("   ").last().unwrap().parse().unwrap())
        .collect();
    line1.sort();
    line2.sort();
    return line1
        .iter()
        .zip(line2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
}
fn exercise2(data: &str) -> i64 {
    let line1: Vec<i64> = data
        .lines()
        .map(|x| x.split("   ").nth(0).unwrap().parse().unwrap())
        .collect();
    let line2: Vec<i64> = data
        .lines()
        .map(|x| x.split("   ").last().unwrap().parse().unwrap())
        .collect();
    let mut counts: HashMap<i64, i64> = HashMap::new();

    for num in line2 {
        *counts.entry(num).or_insert(0) += 1;
    }
    return line1.iter().map(|x| counts.get(&x).unwrap_or(&0) * x).sum();
}

fn main() {
    utils::run(exercise1, exercise2);
}
