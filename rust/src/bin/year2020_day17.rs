use aocsuite::utils;
use std::collections::{HashMap, HashSet};
fn exercise1(data: &str) -> i64 {
    let mut active_cubes = parse_input_3d(data);

    for _ in 0..6 {
        active_cubes = cycle_3d(&active_cubes);
    }
    active_cubes.len() as i64
}
fn exercise2(data: &str) -> i64 {
    let mut active_cubes = parse_input_4d(data);

    for _ in 0..6 {
        active_cubes = cycle_4d(&active_cubes);
    }
    active_cubes.len() as i64
}

fn main() {
    utils::run(exercise1, exercise2);
}

type Point3D = (i32, i32, i32);
type Point4D = (i32, i32, i32, i32);

fn get_neighbors_3d(point: Point3D) -> Vec<Point3D> {
    let mut neighbors = Vec::new();
    let (x, y, z) = point;

    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if dx != 0 || dy != 0 || dz != 0 {
                    neighbors.push((x + dx, y + dy, z + dz));
                }
            }
        }
    }
    neighbors
}
fn get_neighbors_4d(point: Point4D) -> Vec<Point4D> {
    let mut neighbors = Vec::new();
    let (x, y, z, w) = point;

    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                for dw in -1..=1 {
                    if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                        neighbors.push((x + dx, y + dy, z + dz, w + dw));
                    }
                }
            }
        }
    }
    neighbors
}

fn cycle_3d(active_cubes: &HashSet<Point3D>) -> HashSet<Point3D> {
    let mut neighbor_count: HashMap<Point3D, i32> = HashMap::new();

    for &cube in active_cubes.iter() {
        for neighbor in get_neighbors_3d(cube) {
            *neighbor_count.entry(neighbor).or_insert(0) += 1;
        }
    }

    neighbor_count
        .into_iter()
        .filter_map(|(cube, count)| {
            if (count == 3) || (count == 2 && active_cubes.contains(&cube)) {
                Some(cube)
            } else {
                None
            }
        })
        .collect()
}
fn cycle_4d(active_cubes: &HashSet<Point4D>) -> HashSet<Point4D> {
    let mut neighbor_count: HashMap<Point4D, i32> = HashMap::new();

    for &cube in active_cubes.iter() {
        for neighbor in get_neighbors_4d(cube) {
            *neighbor_count.entry(neighbor).or_insert(0) += 1;
        }
    }

    neighbor_count
        .into_iter()
        .filter_map(|(cube, count)| {
            if (count == 3) || (count == 2 && active_cubes.contains(&cube)) {
                Some(cube)
            } else {
                None
            }
        })
        .collect()
}

fn parse_input_3d(input: &str) -> HashSet<Point3D> {
    let mut active_cubes = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes.insert((x as i32, y as i32, 0));
            }
        }
    }
    active_cubes
}

fn parse_input_4d(input: &str) -> HashSet<Point4D> {
    let mut active_cubes = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes.insert((x as i32, y as i32, 0, 0));
            }
        }
    }
    active_cubes
}
