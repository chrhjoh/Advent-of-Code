use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let (timestamp, buses, _) = parse_input(data);
    let (bus_index, earliest_possible_time) = buses
        .iter()
        .map(|&bus| bus * ((timestamp / bus) + 1))
        .enumerate()
        .min_by_key(|&(_, time)| time)
        .unwrap();
    let bus = buses[bus_index];
    println!(
        "bus: {}, earliest_possible_time: {}",
        bus, earliest_possible_time
    );
    return bus as i64 * (earliest_possible_time - timestamp);
}
// bus * t
fn exercise2(data: &str) -> i64 {
    let (_, buses, relative_times) = parse_input(data);
    let answer_formula = buses
        .iter()
        .zip(relative_times.iter())
        .map(|(&bus, &time)| format!("(x + {}) mod {} = 0", time, bus))
        .collect::<Vec<String>>()
        .join(", ");
    println!(
        "answer_formula: {} (paste and solve in wolfram alpha: www.wolframalpha.com)",
        answer_formula
    );
    return 0;
}

fn main() {
    utils::run(exercise1, exercise2);
}

fn parse_input(input: &str) -> (i64, Vec<i64>, Vec<i64>) {
    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|&x| x != "x")
        .map(|x| x.parse().unwrap())
        .collect();
    let relative_times: Vec<i64> = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, x)| *x != "x")
        .map(|(time, _)| time as i64)
        .collect();
    (timestamp, buses, relative_times)
}
