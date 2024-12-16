use aocsuite::utils;
use std::collections::HashSet;

use std::collections::VecDeque;

const DIR: [(i32, i32); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

fn parse_input(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|line| line.chars().collect()).collect()
}

fn get_neighbors(pos: (i32, i32), map: &Vec<Vec<char>>, plant: char) -> Vec<(i32, i32)> {
    let mut neighbors = Vec::new();

    for dir in DIR {
        let neighbor_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if (neighbor_pos.0 as usize) < map.len()
            && (neighbor_pos.1 as usize) < map[0].len()
            && map[neighbor_pos.0 as usize][neighbor_pos.1 as usize] == plant
        {
            neighbors.push(neighbor_pos);
        }
    }

    neighbors
}

fn expand_region(
    pos: (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
    map: &Vec<Vec<char>>,
) -> (HashSet<(i32, i32)>, usize) {
    let mut queue = VecDeque::new();
    let plant = map[pos.0 as usize][pos.1 as usize];
    let mut area = HashSet::default();
    let mut perimiter = 0;
    visited.insert(pos);
    queue.push_back(pos);
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        area.insert(curr);
        let neighbors = get_neighbors(curr, map, plant);
        perimiter += 4 - neighbors.len();
        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    (area, perimiter)
}

fn sides(region: &HashSet<(i32, i32)>) -> usize {
    let mut side_count = 0;
    for dir in DIR {
        let mut sides = HashSet::new();
        for pos in region {
            let tmp = (pos.0 + dir.0, pos.1 + dir.1);
            if !region.contains(&tmp) {
                sides.insert(tmp);
            }
        }
        let mut remove = HashSet::new();
        for side in &sides {
            let mut tmp = (side.0 + dir.1, side.1 + dir.0);
            while sides.contains(&tmp) {
                remove.insert(tmp);
                tmp = (tmp.0 + dir.1, tmp.1 + dir.0);
            }
        }
        side_count += sides.len() - remove.len();
    }

    side_count
}

fn exercise1(data: &str) -> i64 {
    let input = parse_input(&data);
    let mut visited = HashSet::default();
    let mut price = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if !visited.contains(&(i as i32, j as i32)) {
                let (region_area, region_perim) =
                    expand_region((i as i32, j as i32), &mut visited, &input);
                price += region_area.len() * region_perim;
            }
        }
    }

    price as i64
}
fn exercise2(data: &str) -> i64 {
    let input = parse_input(&data);
    let mut visited = HashSet::default();
    let mut price = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if !visited.contains(&(i as i32, j as i32)) {
                let (region_area, _) = expand_region((i as i32, j as i32), &mut visited, &input);
                let sides = sides(&region_area);
                price += region_area.len() * sides;
            }
        }
    }

    price as i64
}

fn main() {
    utils::run(exercise1, exercise2);
}
