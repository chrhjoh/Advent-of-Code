use aocsuite::utils;
use std::collections::HashSet;
fn exercise1(data: &str) -> i64 {
    let (fields, your_ticket, mut other_tickets) = parse_data(&data);
    other_tickets.push(your_ticket.clone());
    let mut error_rate = 0;
    for ticket in other_tickets.iter() {
        for number in ticket.iter() {
            let mut valid = false;
            for field in fields.iter() {
                for range in field.ranges.iter() {
                    if *number >= range.start && *number <= range.end {
                        valid = true;
                        break;
                    }
                }
            }
            if !valid {
                error_rate += number;
            }
        }
    }
    return error_rate;
}
fn exercise2(data: &str) -> i64 {
    let (fields, your_ticket, mut other_tickets) = parse_data(&data);
    other_tickets.push(your_ticket.clone());
    let mut valid_tickets: Vec<Vec<i64>> = vec![];
    for ticket in other_tickets.iter() {
        let mut valid = true;
        for number in ticket.iter() {
            let mut valid_number = false;

            for field in fields.iter() {
                for range in field.ranges.iter() {
                    if *number >= range.start && *number <= range.end {
                        valid_number = true;
                        break;
                    }
                }
            }
            if !valid_number {
                valid = false;
                break;
            }
        }
        if valid {
            valid_tickets.push(ticket.clone());
        }
    }

    let mut assigned_fields: HashSet<usize> = HashSet::new();
    let mut possible_fields: Vec<Option<&Field>> = vec![None; fields.len()];
    let mut field_column: usize = 0;

    while possible_fields.iter().any(|field| field.is_none()) {
        for field_idx in 0..fields.len() {
            if possible_fields[field_idx].is_some() {
                continue;
            }
            let mut possible_fields_for_idx: Vec<&Field> = vec![];
            for (i, field) in fields.iter().enumerate() {
                if assigned_fields.contains(&i) {
                    continue;
                }
                println!("Checking field {} for field {}", field_idx, i);
                let mut valid = true;
                for ticket in valid_tickets.iter() {
                    let number = ticket[field_idx];
                    let mut valid_number = false;
                    for range in field.ranges.iter() {
                        if number >= range.start && number <= range.end {
                            valid_number = true;
                            break;
                        }
                    }
                    if !valid_number {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    field_column = i;
                    possible_fields_for_idx.push(field);
                }
            }
            println!(
                "Field {} has {} possible fields",
                field_idx,
                possible_fields_for_idx.len()
            );
            if possible_fields_for_idx.len() == 1 {
                possible_fields[field_idx] = Some(possible_fields_for_idx[0]);
                assigned_fields.insert(field_column);
                println!("Field {} is {}", field_idx, possible_fields_for_idx[0].name);
            }
        }
    }
    let mut result = 1;
    for (i, field) in possible_fields.iter().enumerate() {
        if field.unwrap().name.starts_with("departure") {
            result *= your_ticket[i];
        }
    }

    return result;
}

fn main() {
    utils::run(exercise1, exercise2);
}

fn parse_data(data: &str) -> (Vec<Field>, Vec<i64>, Vec<Vec<i64>>) {
    let valid_ranges = data
        .split("\n\n")
        .nth(0)
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let name = parts.next().unwrap();
            let ranges = parts
                .next()
                .unwrap()
                .split(" or ")
                .map(|range| {
                    let mut range_parts = range.split('-');
                    let start = range_parts.next().unwrap().parse().unwrap();
                    let end = range_parts.next().unwrap().parse().unwrap();
                    return Range { start, end };
                })
                .collect();
            return Field {
                name: name.to_string(),
                ranges,
            };
        })
        .collect();

    let your_ticket: Vec<i64> = data
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();
    let mut other_tickets: Vec<Vec<i64>> = vec![];
    for ticket in data.split("\n\n").nth(2).unwrap().lines().skip(1) {
        other_tickets.push(
            ticket
                .split(",")
                .map(|number| number.parse().unwrap())
                .collect(),
        )
    }
    return (valid_ranges, your_ticket, other_tickets);
}

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

#[derive(Debug)]
struct Field {
    name: String,
    ranges: Vec<Range>,
}
