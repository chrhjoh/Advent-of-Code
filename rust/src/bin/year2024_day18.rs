use crate::utils::point::{Point, ORTHOGONAL};
use aocsuite::utils;
use std::collections::HashSet;

const GRID_SIZE: i32 = 71;

fn exercise1(data: &str) -> i64 {
    let points = parse_points(data);
    let grid = build_grid(&points, 1024);
    let start = Point::new(0, 0);
    let end = Point::new(GRID_SIZE - 1, GRID_SIZE - 1);
    return bfs(&grid, start, end);
}
fn exercise2(data: &str) -> i64 {
    let points = parse_points(data);
    let start = Point::new(0, 0);
    let end = Point::new(GRID_SIZE - 1, GRID_SIZE - 1);
    let mut steps = 1;
    loop {
        let grid = build_grid(&points, steps);
        let result = bfs(&grid, start, end);
        if result == -1 {
            break;
        }
        steps += 1;
    }
    println!("{:?}", points[steps - 1]);
    return steps as i64;
}

fn main() {
    utils::run(exercise1, exercise2);
}

fn parse_points(data: &str) -> Vec<Point> {
    let points = data
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(x, y)| Point::new(x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    return points;
}

fn build_grid(points: &Vec<Point>, n: usize) -> HashSet<Point> {
    let mut grid = HashSet::new();
    let mut i = 0;
    while i < n {
        grid.insert(points[i].clone());
        i += 1;
    }
    return grid;
}

fn bfs(grid: &HashSet<Point>, start: Point, end: Point) -> i64 {
    let mut queue = Vec::new();
    let mut visited = HashSet::new();
    queue.push(start);
    visited.insert(start);
    let mut steps = 0;
    while !queue.is_empty() {
        let mut next = Vec::new();
        for point in queue {
            if point == end {
                return steps;
            }
            for dir in ORTHOGONAL.iter() {
                let new_point = point + *dir;
                if !grid.contains(&new_point)
                    && !visited.contains(&new_point)
                    && is_valid_point(&new_point, GRID_SIZE, GRID_SIZE)
                {
                    next.push(new_point);
                    visited.insert(new_point);
                }
            }
        }
        queue = next;
        steps += 1;
    }
    return -1;
}

fn print_grid(grid: &HashSet<Point>, visited: &HashSet<Point>, height: i32, width: i32) {
    println!();
    for y in 0..height {
        for x in 0..width {
            let point = Point::new(x, y);
            if grid.contains(&point) {
                print!("#");
            } else if visited.contains(&point) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn is_valid_point(point: &Point, height: i32, width: i32) -> bool {
    return point.x >= 0 && point.x < width && point.y >= 0 && point.y < height;
}
