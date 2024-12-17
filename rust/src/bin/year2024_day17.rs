use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let (instructions, mut registers) = parse_instructions(data);
    let outputs = run(&instructions, &mut registers);

    println!(
        "{}",
        outputs
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );

    return 0;
}
fn exercise2(data: &str) -> i64 {
    let (instructions, registers) = parse_instructions(data);
    let mut factors: Vec<u64> = instructions.iter().map(|_| 0).collect();
    let mut a;
    loop {
        let mut next_attempt_registry = registers;
        a = factors
            .iter()
            .enumerate()
            .map(|(i, f)| 8u64.pow(i as u32) * f)
            .sum();
        next_attempt_registry[0] = a;
        let output = run(&instructions, &mut next_attempt_registry);

        if output == instructions {
            break;
        }
        for i in (0..instructions.len()).rev() {
            if output.len() < i {
                factors[i] += 1;
                break;
            }
            if output[i] != instructions[i] {
                factors[i] += 1;
                break;
            }
        }
    }

    return a as i64;
}

fn main() {
    utils::run(exercise1, exercise2);
}

type Registers = [u64; 3];
type Program = Vec<u64>;

fn run(program: &Program, registers: &mut Registers) -> Vec<u64> {
    let mut output = Vec::new();
    let mut pointer = 0;

    while pointer < program.len() {
        let opcode = program[pointer];
        let operand = program[pointer + 1];

        let combo = match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            _ => panic!("Invalid operand"),
        };

        match opcode {
            0 => registers[0] /= 2u64.pow(combo as u32),
            1 => registers[1] ^= operand,
            2 => registers[1] = combo % 8,
            3 => {
                if registers[0] != 0 {
                    pointer = operand as usize;
                    continue;
                }
            }
            4 => registers[1] ^= registers[2],
            5 => output.push(combo % 8),
            6 => registers[1] = registers[0] / 2u64.pow(combo as u32),
            7 => registers[2] = registers[0] / 2u64.pow(combo as u32),
            _ => panic!(),
        };
        pointer += 2;
    }

    return output;
}

fn parse_instructions(data: &str) -> (Program, Registers) {
    let (registers_unparsed, program_unparsed) = data.split_once("\n\n").unwrap();
    let registers: Vec<u64> = registers_unparsed
        .lines()
        .map(|line| line.split_once(':').unwrap().1.trim().parse().unwrap())
        .collect();
    let registers: Registers = [registers[0], registers[1], registers[2]];

    let program: Vec<u64> = program_unparsed
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    return (program, registers);
}
