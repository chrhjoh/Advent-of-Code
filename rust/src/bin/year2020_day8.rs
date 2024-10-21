use std::collections::HashSet;

use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let lines: Vec<&str> = data.lines().collect();
    let mut seen_instructions: HashSet<isize> = HashSet::new();
    let mut state = ProgramState::new();
    while !seen_instructions.contains(&state.index) {
        seen_instructions.insert(state.index);
        let (operation, value) = parse_instruction(lines[state.index as usize]);
        state = do_instruction(operation, value, state)
    }
    return state.accumulator;
}
fn exercise2(data: &str) -> i64 {
    return 0;
}

fn main() {
    utils::run(exercise1, exercise2);
}

fn do_instruction(operation: &str, value: i64, state: ProgramState) -> ProgramState {
    match operation {
        "nop" => return state.update(0, 1),
        "jmp" => return state.update(0, value as isize),
        "acc" => return state.update(value, 1),
        _ => panic!(),
    }
}

#[derive(Debug)]
struct ProgramState {
    accumulator: i64,
    index: isize,
}

impl ProgramState {
    fn new() -> ProgramState {
        ProgramState {
            accumulator: 0,
            index: 0,
        }
    }
    fn update(mut self: ProgramState, accumulate: i64, index: isize) -> ProgramState {
        self.accumulator += accumulate;
        self.index += index;
        return self;
    }
}

fn parse_instruction(unparsed_instruction: &str) -> (&str, i64) {
    let current_line: Vec<&str> = unparsed_instruction.split_whitespace().collect();
    let operation = current_line[0];
    let unparsed_value = current_line[1];
    let value: i64 = match unparsed_value.chars().nth(0).unwrap() {
        '+' => unparsed_value[1..].parse().unwrap(),
        '-' => -1 * unparsed_value[1..].parse::<i64>().unwrap(),
        _ => panic!(),
    };
    (operation, value)
}
