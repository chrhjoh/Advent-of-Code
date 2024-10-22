use std::collections::HashMap;

use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let mut numbers: Vec<i64> = data.lines().map(|num| num.parse().unwrap()).collect();
    numbers.push(0);
    numbers.push(numbers.iter().max().unwrap() + 3);
    numbers.sort();
    let (mut one_jump, mut two_jump, mut three_jump) = (0, 0, 0);
    for (index, number) in numbers[1..].iter().enumerate() {
        let diff = number - numbers[index];
        match diff {
            1 => {
                one_jump += 1;
            }
            2 => {
                two_jump += 1;
            }
            3 => {
                three_jump += 1;
            }
            _ => panic!(),
        }
    }
    return one_jump * (three_jump + 1);
}
fn exercise2(data: &str) -> i64 {
    let mut numbers: Vec<i64> = data.lines().map(|num| num.parse().unwrap()).collect();
    numbers.push(numbers.iter().max().unwrap() + 3);
    numbers.sort();
    let mut combinations: HashMap<i64, i64> = HashMap::new();
    combinations.insert(0, 1);
    for number in numbers.iter() {
        combinations.insert(
            *number,
            combinations.get(&(number - 1)).unwrap_or(&0)
                + combinations.get(&(number - 2)).unwrap_or(&0)
                + combinations.get(&(number - 3)).unwrap_or(&0),
        );
    }
    return combinations[&numbers[numbers.len() - 1]];
}

fn main() {
    utils::run(exercise1, exercise2);
}
