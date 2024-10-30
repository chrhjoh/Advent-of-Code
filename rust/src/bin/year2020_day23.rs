use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let cups = data
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();
    let mut pointers = link_cups(&cups);
    let mut current = cups[0];
    for _ in 0..100 {
        current = make_move(&mut pointers, current);
    }
    print_cups(&pointers, 1);

    return 0;
}
fn exercise2(data: &str) -> i64 {
    let cups = data
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();
    let mut pointers = link_cups2(&cups);
    let mut current = cups[0];
    for _ in 0..10_000_000 {
        current = make_move(&mut pointers, current);
    }
    return pointers[1] * pointers[pointers[1] as usize];
}

fn main() {
    utils::run(exercise1, exercise2);
}

fn make_move(pointers: &mut Vec<i64>, current: i64) -> i64 {
    let mut picked = vec![];
    picked.push(pointers[current as usize]);
    picked.push(pointers[picked[0] as usize]);
    picked.push(pointers[picked[1] as usize]);

    // Close the gap
    pointers[current as usize] = pointers[picked[2] as usize];

    let mut destination = if current <= 1 {
        (pointers.len() - 1) as i64
    } else {
        current - 1
    };
    while picked.contains(&destination) {
        if destination <= 1 {
            destination = (pointers.len() - 1) as i64;
        } else {
            destination -= 1;
        }
    }
    pointers[picked[2] as usize] = pointers[destination as usize];
    pointers[destination as usize] = picked[0];
    return pointers[current as usize];
}

fn link_cups(cups: &Vec<i64>) -> Vec<i64> {
    let mut pointers = vec![0; cups.len() + 1];
    for i in 0..cups.len() {
        pointers[(cups[i]) as usize] = cups[(i + 1) % cups.len()];
    }
    return pointers;
}

fn link_cups2(cups: &Vec<i64>) -> Vec<i64> {
    let mut pointers = vec![0; 1000001];
    for i in 0..cups.len() {
        pointers[(cups[i]) as usize] = cups[(i + 1) % cups.len()];
    }
    pointers[cups[cups.len() - 1] as usize] = (cups.len() + 1) as i64;
    for i in (cups.len() + 1)..1000000 {
        pointers[i] = i as i64 + 1;
    }
    pointers[1000000] = cups[0];
    return pointers;
}

fn print_cups(pointers: &Vec<i64>, seed: i64) {
    let mut current = seed;
    loop {
        print!("{} ", current);
        current = pointers[current as usize];
        if current == seed {
            break;
        }
    }
    println!();
}
