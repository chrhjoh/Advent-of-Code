use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let numbers: Vec<i64> = data
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect();
    let preamble_length = 25;
    for index in preamble_length..numbers.len() {
        let preamble = &numbers[index - preamble_length..index];
        let number = &numbers[index];
        let to_find: Vec<i64> = preamble.iter().map(|p| number - p).collect();
        let mut found = false;
        for lookup in to_find.iter() {
            if preamble.contains(lookup) && !(lookup * 2).eq(number) {
                found = true;
            }
        }
        if !found {
            return *number;
        }
    }

    return -1;
}
fn exercise2(data: &str) -> i64 {
    let search_number = exercise1(data);
    let numbers: Vec<i64> = data
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect();

    for index in 0..numbers.len() {
        let mut number_range: Vec<i64> = vec![];
        let mut value = 0;
        for number in &numbers[index..] {
            if search_number.gt(&(number + value)) {
                value += number;
                number_range.push(number.to_owned())
            } else if search_number.eq(&(number + value)) {
                return number_range.iter().max().unwrap() + number_range.iter().min().unwrap();
            } else {
                break;
            }
        }
    }

    return -1;
}

fn main() {
    utils::run(exercise1, exercise2);
}
