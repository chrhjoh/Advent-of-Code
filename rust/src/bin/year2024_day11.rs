use aocsuite::utils;
use std::collections::HashMap;

fn exercise1(data: &str) -> i64 {
    let mut cache = HashMap::new();
    data.split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .map(|rock| count_recursive(rock, 25, &mut cache))
        .sum::<usize>() as i64
}
fn exercise2(data: &str) -> i64 {
    let mut cache = HashMap::new();
    data.split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .map(|rock| count_recursive(rock, 75, &mut cache))
        .sum::<usize>() as i64
}

fn main() {
    utils::run(exercise1, exercise2);
}

fn count_recursive(
    stone: usize,
    iterations: usize,
    cache: &mut HashMap<usize, Vec<usize>>,
) -> usize {
    if iterations == 0 {
        return 1;
    }

    if let Some(vec) = cache.get_mut(&stone) {
        if vec.len() <= iterations {
            vec.resize(iterations + 1, 0);
        }
        let result = vec[iterations];
        if result != 0 {
            return result;
        }
    } else {
        let mut vec = Vec::new();
        vec.resize(iterations + 1, 0);
        cache.insert(stone, vec);
    }

    let result: usize;
    if stone == 0 {
        result = count_recursive(1, iterations - 1, cache);
    } else if let Some((left, right)) = divide_stone(stone) {
        result = count_recursive(left, iterations - 1, cache)
            + count_recursive(right, iterations - 1, cache);
    } else {
        result = count_recursive(stone * 2024, iterations - 1, cache);
    }

    cache.get_mut(&stone).unwrap()[iterations] = result;
    result
}

fn divide_stone(stone: usize) -> Option<(usize, usize)> {
    let mut digits = 0;
    let mut number = stone;

    while number > 0 {
        number /= 10;
        digits += 1;
    }

    if digits % 2 != 0 {
        return None;
    }

    let d = 10usize.pow(digits / 2);
    return Some((stone / d, stone % d));
}
