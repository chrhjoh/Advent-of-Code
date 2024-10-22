use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let instructions = data
        .lines()
        .map(Instruction::new)
        .collect::<Vec<Instruction>>();
    let mut ship = Ship::new();
    for instruction in instructions {
        ship.moveit(&instruction);
    }
    return ship.x.abs() as i64 + ship.y.abs() as i64;
}
fn exercise2(data: &str) -> i64 {
    let instructions = data
        .lines()
        .map(Instruction::new)
        .collect::<Vec<Instruction>>();
    let mut ship = Ship::new();
    for instruction in instructions {
        ship.moveit2(&instruction);
    }
    return ship.x.abs() as i64 + ship.y.abs() as i64;
}

fn main() {
    utils::run(exercise1, exercise2);
}

#[derive(Debug)]
struct Ship {
    x: i32,
    y: i32,
    direction: i32,
    waypoint: Waypoint,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            direction: 0,
            waypoint: Waypoint { x: 10, y: 1 },
        }
    }
    fn moveit(self: &mut Ship, instruction: &Instruction) {
        match instruction.action {
            Action::L => {
                self.direction += instruction.value;
                if self.direction >= 360 {
                    self.direction -= 360;
                }
            }
            Action::R => {
                self.direction -= instruction.value;
                if self.direction < 0 {
                    self.direction += 360;
                }
            }
            Action::N => {
                self.y += instruction.value;
            }
            Action::S => {
                self.y -= instruction.value;
            }
            Action::E => {
                self.x += instruction.value;
            }
            Action::W => {
                self.x -= instruction.value;
            }
            Action::F => match self.direction {
                0 => self.x += instruction.value,
                90 => self.y += instruction.value,
                180 => self.x -= instruction.value,
                270 => self.y -= instruction.value,
                _ => panic!("Invalid direction"),
            },
        }
    }
    fn moveit2(self: &mut Ship, instruction: &Instruction) {
        match instruction.action {
            Action::L => {
                let x = self.waypoint.x;
                let y = self.waypoint.y;
                match instruction.value {
                    90 => {
                        self.waypoint.x = -y;
                        self.waypoint.y = x;
                    }
                    180 => {
                        self.waypoint.x = -x;
                        self.waypoint.y = -y;
                    }
                    270 => {
                        self.waypoint.x = y;
                        self.waypoint.y = -x;
                    }
                    _ => panic!("Invalid rotation"),
                }
            }
            Action::R => {
                let x = self.waypoint.x;
                let y = self.waypoint.y;
                match instruction.value {
                    90 => {
                        self.waypoint.x = y;
                        self.waypoint.y = -x;
                    }
                    180 => {
                        self.waypoint.x = -x;
                        self.waypoint.y = -y;
                    }
                    270 => {
                        self.waypoint.x = -y;
                        self.waypoint.y = x;
                    }
                    _ => panic!("Invalid rotation"),
                }
            }
            Action::N => {
                self.waypoint.y += instruction.value;
            }
            Action::S => {
                self.waypoint.y -= instruction.value;
            }
            Action::E => {
                self.waypoint.x += instruction.value;
            }
            Action::W => {
                self.waypoint.x -= instruction.value;
            }
            Action::F => {
                self.x += self.waypoint.x * instruction.value;
                self.y += self.waypoint.y * instruction.value;
            }
        }
    }
}
#[derive(Debug)]
struct Waypoint {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Action {
    N,
    S,
    E,
    W,
    L,
    R,
    F,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    value: i32,
}

impl Instruction {
    fn new(instruction: &str) -> Instruction {
        let action = match instruction.chars().nth(0) {
            Some('N') => Action::N,
            Some('S') => Action::S,
            Some('E') => Action::E,
            Some('W') => Action::W,
            Some('L') => Action::L,
            Some('R') => Action::R,
            Some('F') => Action::F,
            _ => panic!("Invalid action"),
        };

        let value: i32 = instruction[1..].parse().unwrap();
        Instruction {
            action: action,
            value: value,
        }
    }
}
