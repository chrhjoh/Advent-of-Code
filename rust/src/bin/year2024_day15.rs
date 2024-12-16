use core::panic;

use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let (map_str, move_str) = data.split_once("\n\n").unwrap();
    let (mut map, mut current) = parse_map(map_str);
    let moves = parse_moves(move_str);
    for mover in moves.iter() {
        (map, current) = mover.apply_move(current, map);
    }
    display_map(current, &map);
    return calculate_score(&map);
}
fn exercise2(data: &str) -> i64 {
    let (map_str, move_str) = data.split_once("\n\n").unwrap();
    let (mut map, mut current) = parse_map2(map_str);
    let moves = parse_moves(move_str);
    display_map2(current, &map);
    for mover in moves.iter() {
        (map, current) = mover.apply_move2(current, map);
    }
    display_map2(current, &map);
    return calculate_score2(&map);
}

fn main() {
    utils::run(exercise1, exercise2);
}
type Map = Vec<Vec<Position>>;
type Map2 = Vec<Vec<Position2>>;

type Coord = (i64, i64);

#[derive(Clone, Copy, PartialEq, Debug)]
enum PositionType2 {
    Empty,
    LeftBox,
    RightBox,
    Wall,
}

impl PositionType2 {
    fn from_char(c: char, left: bool) -> PositionType2 {
        match c {
            'O' => {
                if left {
                    PositionType2::LeftBox
                } else {
                    PositionType2::RightBox
                }
            }
            '#' => PositionType2::Wall,
            '@' | '.' => PositionType2::Empty,
            _ => panic!(),
        }
    }
}
struct Position2 {
    typ: PositionType2,
    loc: Coord,
}

impl Position2 {
    fn new(xs: (i64, i64), y: i64, c: char) -> (Position2, Position2) {
        let left = Position2 {
            typ: PositionType2::from_char(c, true),
            loc: (xs.0, y),
        };
        let right = Position2 {
            typ: PositionType2::from_char(c, false),
            loc: (xs.1, y),
        };
        (left, right)
    }
}

#[derive(PartialEq)]
enum PositionType {
    Empty,
    Box,
    Wall,
}

impl PositionType {
    fn from_char(c: char) -> PositionType {
        match c {
            'O' => PositionType::Box,
            '#' => PositionType::Wall,
            '@' | '.' => PositionType::Empty,
            _ => panic!(),
        }
    }
}

struct Position {
    typ: PositionType,
    loc: Coord,
}

impl Position {
    fn new(x: i64, y: i64, c: char) -> Position {
        Position {
            typ: PositionType::from_char(c),
            loc: (x, y),
        }
    }
}

fn parse_map(data: &str) -> (Map, Coord) {
    let mut map = Vec::new();
    let mut start = (0, 0);
    for (y, row) in data.lines().enumerate() {
        let mut map_row = Vec::new();
        for (x, c) in row.chars().enumerate() {
            map_row.push(Position::new(x as i64, y as i64, c));
            if c == '@' {
                start = (x as i64, y as i64)
            }
        }
        map.push(map_row)
    }
    (map, start)
}

fn parse_map2(data: &str) -> (Map2, Coord) {
    let mut map = Vec::new();
    let mut start = (0, 0);
    for (y, row) in data.lines().enumerate() {
        let mut map_row = Vec::new();
        let mut x = 0;
        for c in row.chars() {
            let (left, right) = Position2::new((x as i64, x as i64 + 1), y as i64, c);
            map_row.push(left);
            map_row.push(right);
            if c == '@' {
                start = (x as i64, y as i64)
            }
            x += 2;
        }
        map.push(map_row)
    }
    (map, start)
}

struct Move {
    direction: Coord,
}

impl Move {
    fn from_char(c: char) -> Move {
        match c {
            '^' => Move { direction: (0, -1) },
            '<' => Move { direction: (-1, 0) },
            'v' => Move { direction: (0, 1) },
            '>' => Move { direction: (1, 0) },
            _ => panic!(),
        }
    }
    fn apply_move<'a, 'b>(&'a self, current: Coord, map: Map) -> (Map, Coord) {
        let mut current = current;
        let mut map = map;
        let next_position = (current.0 + self.direction.0, current.1 + self.direction.1);
        match map
            .iter()
            .nth(next_position.1 as usize)
            .unwrap()
            .iter()
            .nth(next_position.0 as usize)
            .unwrap()
            .typ
        {
            PositionType::Wall => {}
            PositionType::Empty => current = next_position,
            PositionType::Box => {
                let result = get_boxes_to_move(next_position, self.direction, &map);
                if result.1 {
                    current = next_position;
                    let boxes_to_move = result.0;
                    for coord in boxes_to_move.iter().rev() {
                        map[coord.1 as usize][coord.0 as usize].typ = PositionType::Empty;
                        map[(coord.1 + self.direction.1) as usize]
                            [(coord.0 + self.direction.0) as usize]
                            .typ = PositionType::Box;
                    }
                }
            }
        }

        return (map, current);
    }
    fn apply_move2<'a, 'b>(&'a self, current: Coord, map: Map2) -> (Map2, Coord) {
        let mut current = current;
        let mut map = map;
        let next_position = (current.0 + self.direction.0, current.1 + self.direction.1);
        match map
            .iter()
            .nth(next_position.1 as usize)
            .unwrap()
            .iter()
            .nth(next_position.0 as usize)
            .unwrap()
            .typ
        {
            PositionType2::Wall => {}
            PositionType2::Empty => current = next_position,
            PositionType2::LeftBox => {
                let left = next_position;
                let right = (next_position.0 + 1, next_position.1);
                let result = get_boxes_to_move2(left, right, self.direction, &map);
                if result.1 {
                    current = next_position;
                    let boxes_to_move = result.0;
                    for coords in boxes_to_move.iter().rev() {
                        map[coords.0 .1 as usize][coords.0 .0 as usize].typ = PositionType2::Empty;
                        map[coords.1 .1 as usize][coords.1 .0 as usize].typ = PositionType2::Empty;
                    }

                    for coords in boxes_to_move.iter().rev() {
                        map[(coords.0 .1 + self.direction.1) as usize]
                            [(coords.0 .0 + self.direction.0) as usize]
                            .typ = PositionType2::LeftBox;
                        map[(coords.1 .1 + self.direction.1) as usize]
                            [(coords.1 .0 + self.direction.0) as usize]
                            .typ = PositionType2::RightBox;
                    }
                }
            }
            PositionType2::RightBox => {
                let right = next_position;
                let left = (next_position.0 - 1, next_position.1);
                let result = get_boxes_to_move2(left, right, self.direction, &map);
                if result.1 {
                    current = next_position;
                    let boxes_to_move = result.0;
                    for coords in boxes_to_move.iter().rev() {
                        map[coords.0 .1 as usize][coords.0 .0 as usize].typ = PositionType2::Empty;
                        map[coords.1 .1 as usize][coords.1 .0 as usize].typ = PositionType2::Empty;
                    }
                    for coords in boxes_to_move.iter().rev() {
                        map[(coords.0 .1 + self.direction.1) as usize]
                            [(coords.0 .0 + self.direction.0) as usize]
                            .typ = PositionType2::LeftBox;
                        map[(coords.1 .1 + self.direction.1) as usize]
                            [(coords.1 .0 + self.direction.0) as usize]
                            .typ = PositionType2::RightBox;
                    }
                }
            }
        }

        return (map, current);
    }
}

fn parse_moves(data: &str) -> Vec<Move> {
    data.replace("\n", "")
        .chars()
        .map(|c| Move::from_char(c))
        .collect()
}

fn get_boxes_to_move(loc: Coord, direction: Coord, map: &Map) -> (Vec<Coord>, bool) {
    let next_position = (loc.0 + direction.0, loc.1 + direction.1);
    let mut movable = false;
    let mut boxes = Vec::new();
    match map
        .iter()
        .nth(next_position.1 as usize)
        .unwrap()
        .iter()
        .nth(next_position.0 as usize)
        .unwrap()
        .typ
    {
        PositionType::Wall => {}
        PositionType::Empty => {
            movable = true;
            boxes.push(loc)
        }
        PositionType::Box => {
            let result = get_boxes_to_move(next_position, direction, map);
            if result.1 {
                boxes.push(loc);
                movable = true
            }
            boxes.extend(result.0);
        }
    }
    (boxes, movable)
}
fn get_boxes_to_move2(
    left: Coord,
    right: Coord,
    direction: Coord,
    map: &Map2,
) -> (Vec<(Coord, Coord)>, bool) {
    let next_position_left = (left.0 + direction.0, left.1 + direction.1);
    let next_position_right = (right.0 + direction.0, right.1 + direction.1);
    let mut movable = false;
    let mut boxes = Vec::new();
    match (
        map.iter()
            .nth(next_position_left.1 as usize)
            .unwrap()
            .iter()
            .nth(next_position_left.0 as usize)
            .unwrap()
            .typ,
        map.iter()
            .nth(next_position_right.1 as usize)
            .unwrap()
            .iter()
            .nth(next_position_right.0 as usize)
            .unwrap()
            .typ,
    ) {
        (PositionType2::Wall, _) => {}
        (_, PositionType2::Wall) => {}
        (PositionType2::Empty, PositionType2::Empty) => {
            movable = true;
            boxes.push((left, right))
        }
        (PositionType2::RightBox, PositionType2::Empty) => {
            let next_box_right = next_position_left;
            let next_box_left = (next_position_left.0 - 1, next_position_left.1);
            let mut result: (Vec<(Coord, Coord)>, bool) = (Vec::new(), false);

            if direction == (1, 0) {
                result.1 = true;
            } else {
                result = get_boxes_to_move2(next_box_left, next_box_right, direction, map);
            }
            if result.1 {
                boxes.push((left, right));
                movable = true
            }
            boxes.extend(result.0);
        }
        (PositionType2::Empty, PositionType2::LeftBox) => {
            let next_box_left = next_position_right;
            let next_box_right = (next_position_right.0 + 1, next_position_right.1);
            let mut result: (Vec<(Coord, Coord)>, bool) = (Vec::new(), false);

            if direction == (-1, 0) {
                result.1 = true;
            } else {
                result = get_boxes_to_move2(next_box_left, next_box_right, direction, map);
            }
            if result.1 {
                boxes.push((left, right));
                movable = true
            }
            boxes.extend(result.0);
        }
        (PositionType2::LeftBox, PositionType2::RightBox) => {
            let next_box_left = next_position_left;
            let next_box_right = next_position_right;
            let result = get_boxes_to_move2(next_box_left, next_box_right, direction, map);
            if result.1 {
                boxes.push((left, right));
                movable = true
            }
            boxes.extend(result.0);
        }
        (PositionType2::RightBox, PositionType2::LeftBox) => {
            // Check right box first and then left box
            let next_leftbox_right = next_position_left;
            let next_leftbox_left = (next_position_left.0 - 1, next_position_left.1);
            let mut leftresult: (Vec<(Coord, Coord)>, bool) = (Vec::new(), false);

            if direction == (1, 0) {
                leftresult.1 = true;
            } else {
                leftresult =
                    get_boxes_to_move2(next_leftbox_left, next_leftbox_right, direction, map);
            }
            let mut rightresult: (Vec<(Coord, Coord)>, bool) = (Vec::new(), false);
            let next_rightbox_left = next_position_right;
            let next_rightbox_right = (next_position_right.0 + 1, next_position_right.1);

            if direction == (-1, 0) {
                rightresult.1 = true;
            } else {
                rightresult =
                    get_boxes_to_move2(next_rightbox_left, next_rightbox_right, direction, map);
            }

            if leftresult.1 && rightresult.1 {
                boxes.push((left, right));
                movable = true;
                boxes.extend(leftresult.0);
                boxes.extend(rightresult.0);
            }
        }

        // And a bunch that sholdnt occur
        (PositionType2::Empty, PositionType2::RightBox) => panic!(),
        (PositionType2::LeftBox, PositionType2::Empty) => panic!(),
        (PositionType2::LeftBox, PositionType2::LeftBox) => panic!(),
        (PositionType2::RightBox, PositionType2::RightBox) => panic!(),
    }
    (boxes, movable)
}

fn display_map(current: Coord, map: &Map) {
    for (y, row) in map.iter().enumerate() {
        let row_display: String = row
            .iter()
            .enumerate()
            .map(|(x, pos)| {
                if (x as i64, y as i64) == current {
                    '@'
                } else {
                    match pos.typ {
                        PositionType::Box => 'O',
                        PositionType::Empty => '.',
                        PositionType::Wall => '#',
                    }
                }
            })
            .collect();
        println!("{}", row_display)
    }
    println!()
}

fn display_map2(current: Coord, map: &Map2) {
    for (y, row) in map.iter().enumerate() {
        let row_display: String = row
            .iter()
            .enumerate()
            .map(|(x, pos)| {
                if (x as i64, y as i64) == current {
                    '@'
                } else {
                    match pos.typ {
                        PositionType2::LeftBox => '[',
                        PositionType2::Empty => '.',
                        PositionType2::Wall => '#',
                        PositionType2::RightBox => ']',
                    }
                }
            })
            .collect();
        println!("{}", row_display)
    }
    println!()
}

fn calculate_score(map: &Map) -> i64 {
    map.iter()
        .flatten()
        .filter(|pos| pos.typ == PositionType::Box)
        .map(|pos| pos.loc.1 * 100 + pos.loc.0)
        .sum()
}

fn calculate_score2(map: &Map2) -> i64 {
    map.iter()
        .flatten()
        .filter(|pos| pos.typ == PositionType2::LeftBox)
        .map(|pos| pos.loc.1 * 100 + pos.loc.0)
        .sum()
}
