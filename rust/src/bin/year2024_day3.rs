use aocsuite::utils;
use regex::Regex;
fn exercise1(data: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let result = re
        .captures_iter(data)
        .map(|caps| {
            let a = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let b = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            a * b
        })
        .sum();
    return result;
}
fn exercise2(data: &str) -> i64 {
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let do_indexes = do_re
        .find_iter(data)
        .map(|m| m.start() as i64)
        .collect::<Vec<i64>>();
    println!("{:?}", do_indexes);
    let dont_indexes = dont_re
        .find_iter(data)
        .map(|m| {
            println!("{:?}", m);
            m.start()
        } as i64)
        .collect::<Vec<i64>>();

    let mut active_ranges = do_indexes
        .iter()
        .map(|do_index| {
            let dont_index = dont_indexes
                .iter()
                .find(|dont_index| *dont_index > do_index);
            if let Some(dont_index) = dont_index {
                do_index.clone()..dont_index.clone()
            } else {
                do_index.clone()..i64::try_from(data.len()).unwrap()
            }
        })
        .collect::<Vec<_>>();
    if let Some(dont_in) = dont_indexes.first() {
        active_ranges.push(0..dont_in.clone());
    }
    println!("{:?}", active_ranges);
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let result = re
        .captures_iter(data)
        .map(|caps| {
            let location = caps.get(0).unwrap().start();
            if active_ranges
                .iter()
                .any(|range| range.contains(&i64::try_from(location).unwrap()))
            {
                let a = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let b = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
                a * b
            } else {
                0
            }
        })
        .sum();

    return result;
}

fn main() {
    utils::run(exercise1, exercise2);
}
