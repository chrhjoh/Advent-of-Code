use aocsuite::utils;

#[derive(Debug)]
struct Machine {
    a: (i64, i64),     // Movement in X and Y for button A
    b: (i64, i64),     // Movement in X and Y for button B
    prize: (i64, i64), // Target X and Y coordinates for the prize
}
fn parse_data(data: &str) -> Vec<Machine> {
    let mut machines = Vec::new();
    let unparsed_machines: Vec<&str> = data.split("\n\n").collect();
    for unparsed_machine in unparsed_machines {
        let (a_x, a_y) = unparsed_machine
            .lines()
            .nth(0)
            .unwrap()
            .split_once(",")
            .unwrap();
        let (b_x, b_y) = unparsed_machine
            .lines()
            .nth(1)
            .unwrap()
            .split_once(",")
            .unwrap();
        let (prize_x, prize_y) = unparsed_machine
            .lines()
            .nth(2)
            .unwrap()
            .split_once(",")
            .unwrap();
        machines.push(Machine {
            a: (
                trim_until_char(a_x, '+').parse().unwrap(),
                trim_until_char(a_y, '+').parse().unwrap(),
            ),
            b: (
                trim_until_char(b_x, '+').parse().unwrap(),
                trim_until_char(b_y, '+').parse().unwrap(),
            ),
            prize: (
                trim_until_char(prize_x, '=').parse().unwrap(),
                trim_until_char(prize_y, '=').parse().unwrap(),
            ),
        });
    }

    machines
}
fn trim_until_char(input: &str, c: char) -> &str {
    if let Some(pos) = input.find(c) {
        // Slice the string from the start up to and including the character
        &input[pos + 1..]
    } else {
        // Return the input string if the character isn't found
        input
    }
}

fn solve_claw_machine(machine: &Machine) -> (i64, i64) {
    let det = (machine.a.0 * machine.b.1) - (machine.a.1 * machine.b.0);
    if det == 0 {
        panic!("det == 0");
    }
    let n = (machine.b.1 * machine.prize.0 - machine.b.0 * machine.prize.1) / det;
    let m = (machine.a.0 * machine.prize.1 - machine.a.1 * machine.prize.0) / det;

    let a_correct = n * machine.a.0 + m * machine.b.0 == machine.prize.0;
    let b_correct = n * machine.a.1 + m * machine.b.1 == machine.prize.1;
    return if a_correct && b_correct {
        (n, m)
    } else {
        (0, 0)
    };
}
fn exercise1(data: &str) -> i64 {
    let machines = parse_data(data);
    let mut total_cost = 0;

    for machine in machines {
        let (n, m) = solve_claw_machine(&machine);
        total_cost += n * 3 + m;
    }
    return total_cost as i64;
}
fn exercise2(data: &str) -> i64 {
    let machines = parse_data(data);
    let mut total_cost = 0;

    for machine in machines {
        let mut machine = machine;
        machine.prize.0 += 10000000000000;
        machine.prize.1 += 10000000000000;
        let (n, m) = solve_claw_machine(&machine);
        total_cost += n * 3 + m;
    }
    return total_cost as i64;
}
fn main() {
    utils::run(exercise1, exercise2);
}
