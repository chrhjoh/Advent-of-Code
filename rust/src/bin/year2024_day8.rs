use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let (map, map_size) = parse_map(data);
    let mut antinode_map = create_antinode_map(map_size.x, map_size.y);
    for node in map.iter() {
        let same_frequency = get_same_frequency(node, &map);
        for node_check in same_frequency {
            if node.position != node_check.position {
                let distance = node.manhattan_distance(node_check);
                let antinode_position = node.position.add_delta_point(distance);

                if is_in_bounds(&antinode_position, &map_size) {
                    antinode_map[antinode_position.y as usize][antinode_position.x as usize] =
                        Antinode::Set;
                }
            }
        }
    }
    return antinode_map
        .iter()
        .flatten()
        .filter(|&x| match *x {
            Antinode::Set => true,
            _ => false,
        })
        .count() as i64;
}
fn exercise2(data: &str) -> i64 {
    let (map, map_size) = parse_map(data);
    let mut antinode_map = create_antinode_map(map_size.x, map_size.y);
    for node in map.iter() {
        let same_frequency = get_same_frequency(node, &map);
        for node_check in same_frequency {
            let distance = node.manhattan_distance(node_check);
            if distance.x == 0 && distance.y == 0 {
                continue;
            }
            let mut antinode_position = node.position.subtract_delta_point(&distance);

            loop {
                if !is_in_bounds(&antinode_position, &map_size) {
                    break;
                }
                antinode_map[antinode_position.y as usize][antinode_position.x as usize] =
                    Antinode::Set;

                let updated_node = Node::new(
                    antinode_position.x as usize,
                    antinode_position.y as usize,
                    node.frequency,
                );
                antinode_position = updated_node.position.subtract_delta_point(&distance);
            }
            if is_in_bounds(&antinode_position, &map_size) {
                antinode_map[antinode_position.y as usize][antinode_position.x as usize] =
                    Antinode::Set;
            }
        }
    }
    return antinode_map
        .iter()
        .flatten()
        .filter(|&x| match *x {
            Antinode::Set => true,
            _ => false,
        })
        .count() as i64;
}

fn main() {
    utils::run(exercise1, exercise2);
}

type Towers = Vec<Node>;
type AntinodeMap = Vec<Vec<Antinode>>;

fn parse_map(data: &str) -> (Towers, GridPoint) {
    let mut towers = Vec::new();
    let map_size = GridPoint::new(
        data.lines().nth(0).unwrap().chars().count(),
        data.lines().count(),
    );
    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                _ => towers.push(Node::new(x, y, c)),
            }
        }
    }
    return (towers, map_size);
}

fn create_antinode_map(x: usize, y: usize) -> AntinodeMap {
    let mut antinode_map = Vec::new();
    for _ in 0..y {
        let mut row = Vec::new();
        for _ in 0..x {
            row.push(Antinode::Empty);
        }
        antinode_map.push(row);
    }
    return antinode_map;
}

#[derive(Debug)]
enum Antinode {
    Empty,
    Set,
}

#[derive(Debug)]
struct GridPoint {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct DeltaPoint {
    x: i64,
    y: i64,
}

impl DeltaPoint {
    fn new(x: i64, y: i64) -> DeltaPoint {
        return DeltaPoint { x, y };
    }
}

impl GridPoint {
    fn new(x: usize, y: usize) -> GridPoint {
        return GridPoint { x, y };
    }

    fn add_delta_point(&self, delta: DeltaPoint) -> DeltaPoint {
        return DeltaPoint::new(self.x as i64 + delta.x, self.y as i64 + delta.y);
    }
    fn subtract_delta_point(&self, delta: &DeltaPoint) -> DeltaPoint {
        return DeltaPoint::new(self.x as i64 - delta.x, self.y as i64 - delta.y);
    }
}

impl PartialEq for GridPoint {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

fn is_in_bounds(point: &DeltaPoint, map_size: &GridPoint) -> bool {
    let x = match usize::try_from(point.x) {
        Ok(x) => x,
        Err(_) => return false,
    };
    let y = match usize::try_from(point.y) {
        Ok(y) => y,
        Err(_) => return false,
    };
    return x < map_size.x && y < map_size.y;
}

#[derive(Debug)]
struct Node {
    position: GridPoint,
    frequency: char,
}

impl Node {
    fn new(x: usize, y: usize, frequency: char) -> Node {
        return Node {
            position: GridPoint::new(x, y),
            frequency,
        };
    }
    fn manhattan_distance(&self, other: &Node) -> DeltaPoint {
        return DeltaPoint::new(
            self.position.x as i64 - other.position.x as i64,
            self.position.y as i64 - other.position.y as i64,
        );
    }
}

fn get_same_frequency<'a, 'b>(node: &'a Node, towers: &'b Vec<Node>) -> Vec<&'b Node> {
    let mut same_frequency = Vec::new();
    for node_check in towers.iter() {
        if node.frequency == node_check.frequency {
            same_frequency.push(node_check);
        }
    }
    return same_frequency;
}

fn show_antinode_map(map: &AntinodeMap) {
    for row in map {
        for node in row {
            match node {
                Antinode::Empty => print!("."),
                Antinode::Set => print!("X"),
            }
        }
        println!();
    }
}
