use aocsuite::utils;
use std::collections::{HashMap, HashSet};

const TURN_PENALTY: i64 = 1000;
const MOVE_PENALTY: i64 = 1;

#[derive(Clone, Debug)]
enum Compass {
    North,
    East,
    South,
    West,
    None,
}

impl Compass {
    fn turn(&self, direction: &Direction) -> Compass {
        match (self, direction) {
            (Compass::North, Direction::Clockwise) => Compass::East,
            (Compass::North, Direction::CounterClockwise) => Compass::West,
            (Compass::East, Direction::Clockwise) => Compass::South,
            (Compass::East, Direction::CounterClockwise) => Compass::North,
            (Compass::South, Direction::Clockwise) => Compass::West,
            (Compass::South, Direction::CounterClockwise) => Compass::East,
            (Compass::West, Direction::Clockwise) => Compass::North,
            (Compass::West, Direction::CounterClockwise) => Compass::South,
            (_, Direction::Forward) => self.clone(),
            (Compass::None, _) => Compass::None,
        }
    }

    fn mv(&self, &coord: &Coord) -> Coord {
        match self {
            Compass::North => (coord.0, coord.1 - 1),
            Compass::East => (coord.0 + 1, coord.1),
            Compass::South => (coord.0, coord.1 + 1),
            Compass::West => (coord.0 - 1, coord.1),
            Compass::None => coord,
        }
    }
}

fn exercise1(data: &str) -> i64 {
    let (mut map, start, end) = parse_input(data);
    for dir in vec![Compass::North, Compass::East, Compass::South, Compass::West] {
        let path = Path::new(end, 0, dir);
        score_all_nodes(&path, &mut map);
    }
    let mut best = map.get(&start).unwrap().clone();
    // The Reindeer may face the wrong direction at the start index and we need to turn
    best.penalty += match best.direction {
        Compass::East => 0,
        _ => TURN_PENALTY,
    };
    return best.penalty;
}
fn exercise2(data: &str) -> i64 {
    let (mut map, start, end) = parse_input(data);
    for dir in vec![Compass::North, Compass::East, Compass::South, Compass::West] {
        let path = Path::new(end, 0, dir);
        score_all_nodes(&path, &mut map);
    }
    let mut best = map.get(&start).unwrap().clone();
    // The Reindeer may face the wrong direction at the start index and we need to turn
    best.penalty += match best.direction {
        Compass::East => 0,
        _ => TURN_PENALTY,
    };
    let best_tiles = traverse_all_paths(&map, end, start);
    return best_tiles
        .0
        .iter()
        .map(|c| c.position)
        .collect::<HashSet<Coord>>()
        .len() as i64;
}

fn main() {
    utils::run(exercise1, exercise2);
}

type Coord = (i64, i64);

enum Direction {
    Forward,
    Clockwise,
    CounterClockwise,
}

#[derive(Clone, Debug)]
struct Path {
    position: Coord,
    penalty: i64,
    direction: Compass,
    history: Vec<Coord>,
}

impl Path {
    fn new(position: Coord, penalty: i64, direction: Compass) -> Path {
        Path {
            position,
            penalty,
            direction,
            history: vec![],
        }
    }
    fn turn(&self, direction: Direction) -> Path {
        let mut penalty = self.penalty;
        let new_direction = self.direction.turn(&direction);
        match direction {
            Direction::Forward => {}
            _ => {
                penalty += TURN_PENALTY;
            }
        }
        Path {
            position: self.position,
            penalty,
            direction: new_direction,
            history: self.history.clone(),
        }
    }
    fn step(&mut self) -> &Path {
        self.history.push(self.position);
        self.penalty += MOVE_PENALTY;
        self.position = self.direction.mv(&self.position);
        self
    }
    fn possible_paths(&self) -> Vec<Path> {
        vec![
            self.turn(Direction::Forward).step().clone(),
            self.turn(Direction::Clockwise).step().clone(),
            self.turn(Direction::CounterClockwise).step().clone(),
        ]
    }
}

fn parse_input(input: &str) -> (HashMap<Coord, Path>, Coord, Coord) {
    let mut map = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    let pos = (x as i64, y as i64);
                    let path = Path::new(pos, i64::MAX, Compass::None);
                    map.insert(pos, path);
                }
                'S' => {
                    start = (x as i64, y as i64);
                    let path = Path::new(start, i64::MAX, Compass::None);
                    map.insert(start, path);
                }
                'E' => {
                    end = (x as i64, y as i64);
                    let path = Path::new(end, 0, Compass::None);
                    map.insert(end, path);
                }
                '#' => {}
                _ => {
                    panic!("Invalid character in input");
                }
            }
        }
    }
    (map, start, end)
}

fn score_all_nodes<'a, 'b>(current: &'b Path, scores: &'a mut HashMap<Coord, Path>) {
    let possible_moves: Vec<Path> = current
        .possible_paths()
        .iter()
        .filter(|c| scores.contains_key(&c.position))
        .map(|c| c.clone())
        .collect();

    if possible_moves.len() == 0 {
        return;
    }
    for mv in possible_moves {
        let previous = scores.get(&mv.position).unwrap();

        if mv.penalty < previous.penalty {
            scores.insert(mv.position, mv.clone());
            score_all_nodes(&mv, scores);
        }
    }
}

fn traverse_all_paths(
    scores: &HashMap<Coord, Path>,
    seed: Coord,
    to_find: Coord,
) -> (Vec<&Path>, bool) {
    let current = scores.get(&seed).unwrap();
    let mut found = false;
    if seed == to_find {
        println!("Found it!");
        return (vec![current], true);
    }
    let mut tiles = vec![current];
    let possible_moves = vec![Compass::North, Compass::East, Compass::South, Compass::West]
        .iter()
        .filter(|d| scores.contains_key(&d.mv(&current.position)))
        .map(|d| scores.get(&d.mv(&current.position)).unwrap().clone())
        .collect::<Vec<Path>>();
    if found {
        println!(
            "Searching {:?}",
            possible_moves
                .iter()
                .map(|m| m.position)
                .collect::<Vec<Coord>>()
        );
    }

    let next_seeds_straigt = possible_moves
        .iter()
        .filter(|m| m.penalty == current.penalty + MOVE_PENALTY)
        .map(|m| m.position)
        .collect::<Vec<Coord>>();
    let next_seeds_turn = possible_moves
        .iter()
        .filter(|m| {
            return m.penalty == current.penalty + TURN_PENALTY + MOVE_PENALTY;
        })
        .map(|m| m.position)
        .collect::<Vec<Coord>>();
    let next_seeds = next_seeds_straigt
        .iter()
        .chain(next_seeds_turn.iter())
        .map(|c| c.clone())
        .collect::<Vec<Coord>>();

    for seed in next_seeds {
        let mut result = traverse_all_paths(scores, seed, to_find);
        if result.1 {
            tiles.append(&mut result.0);
            found = true;
        }
    }

    (tiles, found)
}
