use crate::utils::point::{Point, ORTHOGONAL};
use aocsuite::utils;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

const SAVING: i32 = 100;

fn exercise1(data: &str) -> i64 {
    let (points, start, end) = parse_points(data);
    let distances_to_end = bfs_fill(&points, end);
    let distances_to_start = bfs_fill(&points, start);
    let base_distance = distances_to_end[&start];
    points
        .iter()
        .cartesian_product(points.iter())
        .map(|(start_point, end_point)| {
            if manhattan_distance(start_point, end_point) == 2 {
                let distance = distances_to_end[end_point]
                    + distances_to_start[start_point]
                    + manhattan_distance(start_point, end_point);
                if distance <= base_distance - SAVING {
                    return 1;
                }
            }
            return 0;
        })
        .sum()
}
fn exercise2(data: &str) -> i64 {
    let (points, start, end) = parse_points(data);
    let distances_to_end = bfs_fill(&points, end);
    let distances_to_start = bfs_fill(&points, start);
    let base_distance = distances_to_end[&start];

    points
        .iter()
        .cartesian_product(points.iter())
        .map(|(start_point, end_point)| {
            if manhattan_distance(start_point, end_point) <= 20 {
                let distance = distances_to_end[end_point]
                    + distances_to_start[start_point]
                    + manhattan_distance(start_point, end_point);
                if distance <= base_distance - SAVING {
                    return 1;
                }
            }
            return 0;
        })
        .sum()
}
fn main() {
    utils::run(exercise1, exercise2);
}

fn parse_points(data: &str) -> (HashSet<Point>, Point, Point) {
    let mut valid_points = HashSet::new();
    let mut start: Point = Point::new(0, 0);
    let mut end: Point = Point::new(0, 0);
    for (y, x, c) in data.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .map(move |(x, c)| (y as i32, x as i32, c))
    }) {
        match c {
            '.' => {
                valid_points.insert(Point::new(x, y));
            }
            'S' => {
                start = Point::new(x, y);
                valid_points.insert(Point::new(x, y));
            }
            'E' => {
                end = Point::new(x, y);
                valid_points.insert(Point::new(x, y));
            }
            _ => {}
        }
    }
    return (valid_points, start, end);
}

fn bfs_fill(points: &HashSet<Point>, start: Point) -> HashMap<Point, i32> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    distances.insert(start, 0);

    while let Some(current) = queue.pop_front() {
        let current_distance = distances[&current];
        for dir in ORTHOGONAL.iter() {
            let neighbor = current + *dir;
            if points.contains(&neighbor) && !distances.contains_key(&neighbor) {
                distances.insert(neighbor, current_distance + 1);
                queue.push_back(neighbor);
            }
        }
    }

    distances
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    return (p1.x - p2.x).abs() + (p1.y - p2.y).abs();
}
