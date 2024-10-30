use aocsuite::utils;
use std::collections::HashSet;
fn exercise1(data: &str) -> i64 {
    let directions = data.lines().map(parse_directions).collect();
    let flipped_tiles = apply_exercise1_rules(&directions);
    return flipped_tiles.len() as i64;
}
fn exercise2(data: &str) -> i64 {
    let directions = data.lines().map(parse_directions).collect();
    let mut previous_black_tiles = apply_exercise1_rules(&directions);
    println!("Day 0: {}", previous_black_tiles.len());

    for day in 0..100 {
        let neighbors_to_black: HashSet<(i32, i32, i32)> = previous_black_tiles
            .iter()
            .flat_map(|(x, y, z)| get_neighbors(*x, *y, *z))
            .collect();
        let mut future_black_tiles = HashSet::new();

        for tile in neighbors_to_black.union(&previous_black_tiles) {
            let black_neighbors = get_neighbors(tile.0, tile.1, tile.2)
                .iter()
                .filter(|n| previous_black_tiles.contains(n))
                .count();
            let is_black = previous_black_tiles.contains(&tile);

            if is_black && (black_neighbors == 0 || black_neighbors > 2) {
            } else if !is_black && black_neighbors == 2 {
                future_black_tiles.insert(*tile);
            } else if is_black {
                future_black_tiles.insert(*tile);
            }
        }

        previous_black_tiles = future_black_tiles.clone();
        println!("Day {}: {}", day + 1, future_black_tiles.len());
    }
    return previous_black_tiles.len() as i64;
}

fn main() {
    utils::run(exercise1, exercise2);
}

enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn from_str(s: &str) -> Direction {
        match s {
            "e" => Direction::East,
            "se" => Direction::SouthEast,
            "sw" => Direction::SouthWest,
            "w" => Direction::West,
            "nw" => Direction::NorthWest,
            "ne" => Direction::NorthEast,
            _ => panic!("Invalid direction"),
        }
    }
}

struct Pointer {
    x: i32,
    y: i32,
    z: i32,
}

impl Pointer {
    fn new() -> Pointer {
        Pointer { x: 0, y: 0, z: 0 }
    }

    fn move_to(&mut self, direction: &Direction) {
        match direction {
            Direction::East => {
                self.x += 1;
                self.y -= 1
            }
            Direction::SouthEast => {
                self.y -= 1;
                self.z += 1
            }
            Direction::SouthWest => {
                self.x -= 1;
                self.z += 1
            }
            Direction::West => {
                self.x -= 1;
                self.y += 1
            }
            Direction::NorthWest => {
                self.y += 1;
                self.z -= 1
            }
            Direction::NorthEast => {
                self.x += 1;
                self.z -= 1;
            }
        }
    }
    fn coords(&self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }
}

fn get_neighbors(x: i32, y: i32, z: i32) -> Vec<(i32, i32, i32)> {
    vec![
        (x + 1, y - 1, z),
        (x, y - 1, z + 1),
        (x - 1, y, z + 1),
        (x - 1, y + 1, z),
        (x, y + 1, z - 1),
        (x + 1, y, z - 1),
    ]
}

fn parse_directions(line: &str) -> Vec<Direction> {
    let mut directions = Vec::new();
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        let direction = match c {
            'e' | 'w' => Direction::from_str(&c.to_string()),
            's' | 'n' => {
                let next = chars.next().unwrap();
                Direction::from_str(&format!("{}{}", c, next))
            }
            _ => panic!("Invalid direction"),
        };
        directions.push(direction);
    }
    directions
}

fn apply_exercise1_rules(directions: &Vec<Vec<Direction>>) -> HashSet<(i32, i32, i32)> {
    let mut flipped_tiles = HashSet::new();
    for direction in directions {
        let mut pointer = Pointer::new();
        for d in direction {
            pointer.move_to(&d);
        }
        if flipped_tiles.contains(&pointer.coords()) {
            flipped_tiles.remove(&pointer.coords());
        } else {
            flipped_tiles.insert(pointer.coords());
        }
    }
    flipped_tiles
}
