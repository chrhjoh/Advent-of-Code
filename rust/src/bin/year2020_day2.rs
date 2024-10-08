use aocsuite::utils;
fn exercise1(data: &str) -> i32 {
    let answer = data
        .lines()
        .map(|line| process_line(&line).is_valid_1())
        .filter(|result| *result)
        .count() as i32;
    return answer;
}
fn exercise2(data: &str) -> i32 {
    let answer = data
        .lines()
        .map(|line| process_line(&line).is_valid_2())
        .filter(|result| *result)
        .count() as i32;
    return answer;
}

fn main() {
    utils::run(exercise1, exercise2);
}

#[derive(Debug)]
struct InputLine {
    low_bound: i32,
    high_bound: i32,
    restricted_char: char,
    password: String,
}
impl InputLine {
    fn new(low_bound: i32, high_bound: i32, restricted_char: char, password: String) -> InputLine {
        InputLine {
            low_bound: low_bound,
            high_bound: high_bound,
            restricted_char: restricted_char,
            password: password,
        }
    }

    fn is_valid_1(&self) -> bool {
        let char_seen = self
            .password
            .chars()
            .filter(|c| *c == self.restricted_char)
            .count() as i32;

        return self.low_bound <= char_seen && char_seen <= self.high_bound;
    }
    fn is_valid_2(&self) -> bool {
        println!("{:?}", self.password);
        let valid: bool = self
            .password
            .chars()
            .enumerate()
            .filter(|(i, character)| {
                vec![self.low_bound as usize - 1, self.high_bound as usize - 1].contains(i)
                    && *character == self.restricted_char
            })
            .count()
            == 1;
        println!("{:?}", valid);

        return valid;
    }
}

fn process_line(line: &str) -> InputLine {
    let low_bound: i32;
    let high_bound: i32;
    let restricted_char: char;
    let password: String;

    low_bound = line[0..line.find('-').unwrap()].parse().unwrap();
    high_bound = line[line.find('-').unwrap() + 1..line.find(' ').unwrap()]
        .parse()
        .unwrap();
    restricted_char = line[line.find(' ').unwrap()..].chars().nth(1).unwrap();
    password = line[line.find(':').unwrap() + 2..].to_string();

    return InputLine::new(low_bound, high_bound, restricted_char, password);
}
