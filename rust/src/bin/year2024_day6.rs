use std::collections::HashSet;

use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let (mut guard, grid) = parse_input(data);

    let mut path: Vec<Tile> = Vec::new();
    let mut finished = false;
    while !finished {
        let result = guard.do_move(&grid);
        path.append(result.0.clone().as_mut());
        finished = result.1;
        guard.turn_right();
    }
    return path.iter().collect::<HashSet<&Tile>>().len() as i64;
}
fn exercise2(data: &str) -> i64 {
    let (_, grid) = parse_input(data);
    let mut loop_count = 0;
    for i in 0..grid.len() {
        for j in 0..grid.iter().nth(0).unwrap().len() {
            let (mut guard, mut test_grid) = parse_input(data);
            if (j as i32, i as i32) == guard.position {
                continue;
            }
            test_grid[i][j] = Position::Obstacle;
            let mut seen_ends: Vec<Tile> = Vec::new();
            let mut finished = false;
            while !finished {
                let result = guard.do_move(&test_grid);
                if result.0.len() == 0 {
                    guard.turn_right();
                    continue;
                }
                if seen_ends.contains(&guard.position) {
                    loop_count += 1;
                    finished = true;
                } else {
                    seen_ends.push(guard.position);
                    finished = result.1;
                    guard.turn_right();
                }
            }
        }
    }
    return loop_count as i64;
}

fn main() {
    utils::run(exercise1, exercise2);
}
fn get_position<'a>(grid: &'a Grid, tile: &Tile) -> Result<&'a Position, String> {
    match &grid.iter().nth(tile.0 as usize) {
        Some(row) => match row.iter().nth(tile.1 as usize) {
            Some(position) => Ok(position),
            None => Err("Invalid x position".to_string()),
        },
        None => Err("Invalid y position".to_string()),
    }
}
type Grid = Vec<Vec<Position>>;
type Tile = (i32, i32);

#[derive(Debug)]
enum Position {
    Obstacle,
    Empty,
}
impl Position {
    fn from_char(c: char) -> Position {
        match c {
            '#' => Position::Obstacle,
            '.' => Position::Empty,
            '^' | 'v' | '<' | '>' => Position::Empty,
            _ => panic!("Invalid position: {c}"),
        }
    }
}

#[derive(Debug)]
struct Direction {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Guard {
    direction: Direction,
    position: Tile,
}

impl Guard {
    fn from_char(guard_token: char, position: Tile) -> Guard {
        let direction = match guard_token {
            '^' => Direction { x: 0, y: -1 },
            'v' => Direction { x: 0, y: 1 },
            '<' => Direction { x: -1, y: 0 },
            '>' => Direction { x: 1, y: 0 },
            _ => panic!("Invalid guard token: {guard_token}"),
        };
        Guard {
            direction,
            position,
        }
    }
    fn do_move(&mut self, grid: &Grid) -> (Vec<Tile>, bool) {
        let mut path = Vec::new();
        self.move_forward();
        loop {
            let position = get_position(grid, &self.position);

            match position {
                Ok(Position::Obstacle) => {
                    self.move_reverse();
                    return (path, false);
                }
                Ok(Position::Empty) => {
                    path.push(self.position.clone());
                }
                Err(_) => {
                    self.move_reverse();
                    return (path, true);
                }
            }
            self.move_forward();
        }
    }
    fn move_forward(&mut self) {
        self.position = (
            self.position.0 + self.direction.y,
            self.position.1 + self.direction.x,
        );
    }
    fn move_reverse(&mut self) {
        self.position = (
            self.position.0 - self.direction.y,
            self.position.1 - self.direction.x,
        );
    }

    fn turn_right(&mut self) {
        let x = self.direction.x;
        let y = self.direction.y;
        self.direction = Direction { x: -y, y: x };
    }
}

fn parse_input(input: &str) -> (Guard, Grid) {
    let mut grid = Vec::new();
    let mut guard: Option<Guard> = None;
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == '^' || c == 'v' || c == '<' || c == '>' {
                guard = Some(Guard::from_char(c, (y as i32, x as i32)));
            }
            row.push(Position::from_char(c));
        }
        grid.push(row);
    }
    match guard {
        Some(g) => return (g, grid),
        None => panic!("No guard found"),
    }
}
