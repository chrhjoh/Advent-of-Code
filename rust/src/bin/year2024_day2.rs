use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    return data
        .lines()
        .map(|line| line.split(" ").map(|x| x.parse().unwrap()).collect())
        .filter(|report| is_safe(report))
        .count() as i64;
}
fn exercise2(data: &str) -> i64 {
    let n_basic_safe = data
        .lines()
        .map(|line| line.split(" ").map(|x| x.parse().unwrap()).collect())
        .filter(|report| is_safe(report))
        .count() as i64;

    let unsafe_reports: Vec<Vec<i64>> = data
        .lines()
        .map(|line| line.split(" ").map(|x| x.parse().unwrap()).collect())
        .filter(|report| !is_safe(report))
        .collect();

    let n_safe_unsafe = unsafe_reports
        .iter()
        .filter(|report| check_all_sub_reports(report))
        .count();
    return n_basic_safe + n_safe_unsafe as i64;
}

fn main() {
    utils::run(exercise1, exercise2);
}

fn is_safe(report: &Vec<i64>) -> bool {
    let mut report_iter = report.iter();
    let mut previous = report_iter.next().unwrap();
    let mut current = report_iter.next().unwrap();

    let is_increasing: bool;

    if current - previous > 0 {
        is_increasing = true;
    } else if current - previous < 0 {
        is_increasing = false;
    } else {
        return false;
    }

    match (current - previous).abs() {
        1..4 => {}
        _ => return false,
    }

    while let Some(next_number) = report_iter.next() {
        previous = current;
        current = next_number;

        if current <= previous && is_increasing {
            return false;
        }
        if current >= previous && !is_increasing {
            return false;
        }
        match (current - previous).abs() {
            1..4 => {}
            _ => return false,
        }
    }
    return true;
}

fn check_all_sub_reports(report: &Vec<i64>) -> bool {
    for i in 0..report.len() {
        let subreport = report[..i]
            .iter()
            .chain(report[i + 1..].iter())
            .copied()
            .collect();
        if is_safe(&subreport) {
            return true;
        }
    }
    return false;
}
