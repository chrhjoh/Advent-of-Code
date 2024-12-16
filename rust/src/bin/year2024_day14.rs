use aocsuite::utils;
use std::io::stdin;
fn exercise1(data: &str) -> i64 {
    let robots = data
        .lines()
        .map(|line| Robot::from_str(line))
        .collect::<Vec<Robot>>();
    const X_MAX: i64 = 101;
    const Y_MAX: i64 = 103;
    const TURNS: i64 = 100;
    print_robots(&robots, X_MAX, Y_MAX);

    let mut updated_robots = robots.clone();
    for turn in 0..TURNS {
        updated_robots = updated_robots
            .iter()
            .map(|robot| robot.move_turns(1, X_MAX, Y_MAX))
            .collect::<Vec<Robot>>();
        println!("Turn: {}", turn + 1);
        print_robots(&updated_robots, X_MAX, Y_MAX);
    }

    return count_quardrants(&updated_robots, X_MAX, Y_MAX);
}
fn exercise2(data: &str) -> i64 {
    let robots = data
        .lines()
        .map(|line| Robot::from_str(line))
        .collect::<Vec<Robot>>();
    const X_MAX: i64 = 101;
    const Y_MAX: i64 = 103;
    print_robots(&robots, X_MAX, Y_MAX);

    let mut updated_robots = robots.clone();
    let mut turn = 0;
    let mut s = String::new();
    loop {
        updated_robots = updated_robots
            .iter()
            .map(|robot| robot.move_turns(1, X_MAX, Y_MAX))
            .collect::<Vec<Robot>>();
        turn += 1;
        println!("Turn: {}", turn);
        print_robots(&updated_robots, X_MAX, Y_MAX);
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        match s[..].trim() {
            "" => {}
            _ => break,
        }
    }
    return turn;
}

fn main() {
    utils::run(exercise1, exercise2);
}

#[derive(Clone)]
struct Coordinate {
    x: i64,
    y: i64,
}
impl Coordinate {
    fn new(x: i64, y: i64) -> Coordinate {
        Coordinate { x, y }
    }
}

#[derive(Clone)]
struct Direction {
    x: i64,
    y: i64,
}

#[derive(Clone)]
struct Robot {
    coordinate: Coordinate,
    direction: Direction,
}

impl Robot {
    fn from_str(line: &str) -> Robot {
        let (p, v) = line.split_once(" ").unwrap();
        let coordinate = p.trim_start_matches("p=").split_once(",").unwrap();
        let direction = v.trim_start_matches("v=").split_once(",").unwrap();
        Robot {
            coordinate: Coordinate::new(
                coordinate.0.parse().unwrap(),
                coordinate.1.parse().unwrap(),
            ),
            direction: Direction {
                x: direction.0.parse::<i64>().unwrap(),
                y: direction.1.parse::<i64>().unwrap(),
            },
        }
    }

    fn move_turns(&self, turns: i64, x_max: i64, y_max: i64) -> Robot {
        let mut new_x = (self.coordinate.x + self.direction.x * turns) % x_max;
        if new_x < 0 {
            new_x += x_max; // Adjust negative values to wrap around
        }

        let mut new_y = (self.coordinate.y + self.direction.y * turns) % y_max;
        if new_y < 0 {
            new_y += y_max; // Adjust negative values to wrap around
        }
        Robot {
            coordinate: Coordinate::new(new_x, new_y),
            direction: Direction {
                x: self.direction.x,
                y: self.direction.y,
            },
        }
    }
}

fn print_robots(robots: &Vec<Robot>, x_max: i64, y_max: i64) {
    let mut grid = vec![vec!["."; x_max as usize]; y_max as usize];
    for robot in robots {
        grid[robot.coordinate.y as usize][robot.coordinate.x as usize] = "#";
    }
    for row in grid {
        println!("{}", row.join(""));
    }
    println!();
}

fn count_quardrants(robots: &Vec<Robot>, x_max: i64, y_max: i64) -> i64 {
    let mut quardrants = vec![0; 4];
    for robot in robots {
        if robot.coordinate.x < x_max / 2 && robot.coordinate.y < y_max / 2 {
            quardrants[0] += 1;
        } else if robot.coordinate.x > x_max / 2 && robot.coordinate.y < y_max / 2 {
            quardrants[1] += 1;
        } else if robot.coordinate.x < x_max / 2 && robot.coordinate.y > y_max / 2 {
            quardrants[2] += 1;
        } else if robot.coordinate.x > x_max / 2 && robot.coordinate.y > y_max / 2 {
            quardrants[3] += 1;
        }
    }
    return quardrants.iter().product();
}
