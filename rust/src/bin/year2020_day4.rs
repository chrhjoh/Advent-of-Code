use regex::Regex;
use std::collections::HashMap;

use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let answer = data
        .split("\n\n")
        .map(|passport_infomation| parse_passport(passport_infomation))
        .filter(|passport| is_valid(passport, vec!["cid"]))
        .count() as i64;
    return answer;
}
fn exercise2(data: &str) -> i64 {
    let answer = data
        .split("\n\n")
        .map(|passport_infomation| parse_passport(passport_infomation))
        .filter(|passport| is_valid(passport, vec!["cid"]))
        .filter(|passport| has_valid_data(passport))
        .count() as i64;

    return answer;
}

fn main() {
    utils::run(exercise1, exercise2);
}

fn parse_passport(passport_infomation: &str) -> HashMap<&str, &str> {
    let mut passport: HashMap<&str, &str> = HashMap::new();
    let found_pairs = parse_to_map(passport_infomation);
    for (key, value) in found_pairs {
        passport.insert(key, value);
    }
    return passport;
}
fn is_valid(passport: &HashMap<&str, &str>, ignore_keys: Vec<&str>) -> bool {
    let requried_keys: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

    for key in requried_keys.iter() {
        if ignore_keys.contains(key) {
            continue;
        }
        match passport.get(key) {
            Some(_) => {
                continue;
            }
            None => {
                return false;
            }
        }
    }
    true
}

fn has_valid_data(passport: &HashMap<&str, &str>) -> bool {
    let byr: i32 = passport.get("byr").unwrap().parse().unwrap();
    if byr < 1920 || byr > 2002 {
        return false;
    }
    let iyr: i32 = passport.get("iyr").unwrap().parse().unwrap();
    if iyr < 2010 || iyr > 2020 {
        return false;
    }
    let eyr: i32 = passport.get("eyr").unwrap().parse().unwrap();
    if eyr < 2020 || eyr > 2030 {
        return false;
    }
    let hgt = passport.get("hgt").unwrap();
    let hgt_unit = &hgt[hgt.len() - 2..];

    match hgt_unit {
        "cm" => {
            let hgt_val: i32 = hgt[..hgt.len() - 2].parse().unwrap();

            if hgt_val < 150 || hgt_val > 193 {
                return false;
            }
        }
        "in" => {
            let hgt_val: i32 = hgt[..hgt.len() - 2].parse().unwrap();

            if hgt_val < 59 || hgt_val > 76 {
                return false;
            }
        }
        _ => {
            return false;
        }
    }
    let hcl = passport.get("hcl").unwrap();

    let hair_pattern = r"^#([a-f0-9]{6})$";
    let hair_re = Regex::new(hair_pattern).unwrap();
    if !hair_re.is_match(hcl) {
        return false;
    }
    let ecl = passport.get("ecl").unwrap();

    let eye_colors: Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    if !eye_colors.contains(ecl) {
        return false;
    }
    let pid = passport.get("pid").unwrap();

    let pid_pattern = r"^\d{9}$";
    let pid_re = Regex::new(pid_pattern).unwrap();
    if !pid_re.is_match(pid) {
        return false;
    }

    return true;
}

fn parse_to_map(input: &str) -> HashMap<&str, &str> {
    let mut map = HashMap::new();

    for pair in input.split_whitespace() {
        let mut parts = pair.split(':');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            map.insert(key, value);
        }
    }

    map
}
