use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let inputs = data.lines().map(|line| Input::from_str(line));
    inputs
        .filter(|input| can_hit_ceiling(0, &input.numbers, input.result, Operator1::Add))
        .map(|input| input.result)
        .sum()
}
fn exercise2(data: &str) -> i64 {
    let inputs = data.lines().map(|line| Input::from_str(line));
    inputs
        .filter(|input| can_hit_ceiling2(0, &input.numbers, input.result, Operator2::Add))
        .map(|input| input.result)
        .sum()
}

fn main() {
    utils::run(exercise1, exercise2);
}

struct Input {
    result: i64,
    numbers: Vec<i64>,
}

impl Input {
    fn from_str(data: &str) -> Input {
        let (result, number_sequence) = data.split_once(":").unwrap();
        let numbers = number_sequence
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        Input {
            result: result.parse().unwrap(),
            numbers,
        }
    }
}
fn can_hit_ceiling(current: i64, numbers: &Vec<i64>, ceiling: i64, operator: Operator1) -> bool {
    let next_number = match numbers.first() {
        None => {
            return current == ceiling;
        }
        Some(number) => number,
    };
    let new_current = operator.operate(current, *next_number);

    let new_numbers = numbers[1..].to_vec();
    return can_hit_ceiling(new_current, &new_numbers, ceiling, Operator1::Add)
        || can_hit_ceiling(new_current, &new_numbers, ceiling, Operator1::Multiply);
}

fn can_hit_ceiling2(current: i64, numbers: &Vec<i64>, ceiling: i64, operator: Operator2) -> bool {
    let next_number = match numbers.first() {
        None => {
            return current == ceiling;
        }
        Some(number) => number,
    };
    let new_current = operator.operate(current, *next_number);

    let new_numbers = numbers[1..].to_vec();
    return can_hit_ceiling2(new_current, &new_numbers, ceiling, Operator2::Add)
        || can_hit_ceiling2(new_current, &new_numbers, ceiling, Operator2::Multiply)
        || can_hit_ceiling2(new_current, &new_numbers, ceiling, Operator2::Concatenate);
}

enum Operator1 {
    Add,
    Multiply,
}

impl Operator1 {
    fn operate(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator1::Add => a + b,
            Operator1::Multiply => a * b,
        }
    }
}

enum Operator2 {
    Add,
    Multiply,
    Concatenate,
}

impl Operator2 {
    fn operate(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator2::Add => a + b,
            Operator2::Multiply => a * b,
            Operator2::Concatenate => format!("{}{}", a, b).parse().unwrap(),
        }
    }
}
