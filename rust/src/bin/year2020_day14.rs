use aocsuite::utils;
use std::collections::HashMap;

fn exercise1(data: &str) -> i64 {
    let mut program = Program::new();
    data.lines().for_each(|line| {
        let mut parts = line.split(" = ");
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();
        if key == "mask" {
            program.set_mask(value);
        } else {
            let address = key[4..key.len() - 1].parse().unwrap();
            let value = value.parse().unwrap();
            program.set_memory(address, value);
        }
    });
    return program.sum_memory();
}
fn exercise2(data: &str) -> i64 {
    let mut program = Program::new();
    data.lines().for_each(|line| {
        let mut parts = line.split(" = ");
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();
        if key == "mask" {
            program.set_mask(value);
        } else {
            let address = key[4..key.len() - 1].parse().unwrap();
            let value = value.parse().unwrap();
            program.set_memory_2(address, value);
        }
    });
    return program.sum_memory();
}

fn main() {
    utils::run(exercise1, exercise2);
}

struct Program {
    mask: String,
    memory: HashMap<i64, i64>,
}

impl Program {
    fn new() -> Program {
        Program {
            mask: String::new(),
            memory: HashMap::new(),
        }
    }

    fn set_mask(&mut self, mask: &str) {
        self.mask = mask.to_string();
    }

    fn set_memory(&mut self, address: i64, value: i64) {
        let mut value = value;
        for (i, c) in self.mask.chars().enumerate() {
            let bit = 1 << (self.mask.len() - i - 1);
            match c {
                '0' => value &= !bit,
                '1' => value |= bit,
                'X' => (),
                _ => panic!("Invalid mask character"),
            }
        }
        self.memory.insert(address, value);
    }

    fn sum_memory(&self) -> i64 {
        self.memory.values().sum()
    }
    fn set_memory_2(&mut self, address: i64, value: i64) {
        let mut addresses = vec![address];
        for (i, c) in self.mask.chars().enumerate() {
            let bit = 1 << (self.mask.len() - i - 1);

            match c {
                '0' => (),
                '1' => addresses.iter_mut().for_each(|a| *a |= bit),
                'X' => {
                    let mut new_values = Vec::new();
                    for a in addresses.iter() {
                        new_values.push(a | bit);
                        new_values.push(a & !bit);
                    }
                    addresses = new_values;
                }
                _ => panic!("Invalid mask character"),
            }
        }
        for address in addresses.iter() {
            self.memory.insert(*address, value);
        }
    }
}
