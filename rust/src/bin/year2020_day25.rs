use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let (card_public_key, door_public_key) = parse_input(data);
    let mut card_loop_size = 0;
    let mut door_loop_size = 0;
    let mut value = 1;
    let subject_number = 7;
    let divisor = 20201227;
    loop {
        value = (value * subject_number) % divisor;
        card_loop_size += 1;
        if value == card_public_key {
            break;
        }
    }
    value = 1;
    loop {
        value = (value * subject_number) % divisor;
        door_loop_size += 1;
        if value == door_public_key {
            break;
        }
    }
    value = 1;
    for _ in 0..card_loop_size {
        value = (value * door_public_key) % divisor;
    }
    return value;
}
fn exercise2(data: &str) -> i64 {
    return 0;
}

fn main() {
    utils::run(exercise1, exercise2);
}

fn parse_input(input: &str) -> (i64, i64) {
    let card_public_key: i64 = input.lines().nth(0).unwrap().parse().unwrap();
    let door_public_key: i64 = input.lines().nth(1).unwrap().parse().unwrap();
    return (card_public_key, door_public_key);
}
