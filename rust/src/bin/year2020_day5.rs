use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    return data
        .lines()
        .map(|card| binary_search(&card) as i64)
        .max()
        .unwrap();
}
fn exercise2(data: &str) -> i64 {
    let mut possible_values: [u32; 127 * 8 + 7] = [0; 127 * 8 + 7];
    let seen_values = data.lines().map(|card| binary_search(&card) as usize);

    for index in seen_values {
        possible_values[index] = 1
    }
    for (i, seat) in possible_values.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let previous_seat: u32 = match possible_values.get(i - 1) {
            Some(val) => *val,
            None => continue,
        };
        let next_seat: u32 = match possible_values.get(i + 1) {
            Some(val) => *val,
            None => continue,
        };

        if *seat == 0 && previous_seat == 1 && next_seat == 1 {
            return i as i64;
        }
    }
    return -1;
}

fn main() {
    utils::run(exercise1, exercise2);
}

fn binary_search(search_str: &str) -> u32 {
    let mut low: u32 = 0;
    let mut high: u32 = 127;
    let row_string = &search_str[..7];
    let row: u32;

    for char in row_string.chars() {
        let mid = (high + low).div_ceil(2);

        match char {
            'F' => {
                high = mid - 1;
            }
            'B' => {
                low = mid;
            }
            _ => panic!(),
        }
    }
    match row_string.chars().last().unwrap() {
        'F' => {
            row = low;
        }
        'B' => {
            row = high;
        }
        _ => panic!(),
    };
    let mut low: u32 = 0;
    let mut high: u32 = 7;
    let col_string = &search_str[7..];
    let col: u32;

    for char in col_string.chars() {
        let mid = (high + low).div_ceil(2);

        match char {
            'L' => {
                high = mid - 1;
            }
            'R' => {
                low = mid;
            }
            _ => {
                panic!()
            }
        }
    }
    match col_string.chars().last().unwrap() {
        'L' => {
            col = low;
        }
        'R' => {
            col = high;
        }
        _ => panic!(),
    };
    let answer = row * 8 + col;
    return answer;
}
