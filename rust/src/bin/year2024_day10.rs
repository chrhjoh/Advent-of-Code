use aocsuite::utils;
use std::collections::{HashMap, HashSet};
fn exercise1(data: &str) -> i64 {
    let map = parse_map(data);
    let trailheads = find_trailheads(&map);
    return trailheads
        .iter()
        .map(|trailhead| {
            trailhead
                .find_trails(&map)
                .iter()
                .collect::<HashSet<_>>()
                .len() as u32
        })
        .sum::<u32>() as i64;
}
fn exercise2(data: &str) -> i64 {
    let map = parse_map(data);
    let trailheads = find_trailheads(&map);
    return trailheads
        .iter()
        .map(|trailhead| trailhead.find_trails(&map).len() as u32)
        .sum::<u32>() as i64;
}

fn main() {
    utils::run(exercise1, exercise2);
}

fn parse_map(data: &str) -> HashMap<(usize, usize), u32> {
    let mut map = HashMap::new();
    for (y, c) in data.lines().enumerate() {
        for (x, d) in c.chars().enumerate() {
            map.insert((x, y), d.to_digit(10).unwrap());
        }
    }
    return map;
}

struct TrailPosition {
    x: usize,
    y: usize,
    height: u32,
}

impl TrailPosition {
    fn new(x: usize, y: usize, height: u32) -> TrailPosition {
        TrailPosition { x, y, height }
    }
    fn finished(&self) -> bool {
        return self.height == 9;
    }

    fn next(&self, map: &HashMap<(usize, usize), u32>) -> Vec<TrailPosition> {
        let mut next = Vec::new();
        let mut potential_next = Vec::new();
        if self.x > 0 {
            potential_next.push((self.x - 1, self.y));
        }
        if self.y > 0 {
            potential_next.push((self.x, self.y - 1));
        }
        potential_next.push((self.x + 1, self.y));
        potential_next.push((self.x, self.y + 1));
        for (x, y) in potential_next {
            if let Some(height) = map.get(&(x, y)) {
                if *height == self.height + 1 {
                    next.push(TrailPosition::new(x, y, *height));
                }
            }
        }

        return next;
    }

    fn find_trails(&self, map: &HashMap<(usize, usize), u32>) -> Vec<(usize, usize)> {
        let mut queue = Vec::new();
        let mut finished_trails = Vec::new();
        queue.extend(self.next(map));
        while let Some(current) = queue.pop() {
            if current.finished() {
                finished_trails.push((current.x, current.y));
                continue;
            }
            queue.extend(current.next(map));
        }

        return finished_trails;
    }
}
fn find_trailheads(map: &HashMap<(usize, usize), u32>) -> Vec<TrailPosition> {
    let mut trailheads = Vec::new();
    for (k, v) in map.iter() {
        if *v == 0 {
            trailheads.push(TrailPosition::new(k.0, k.1, *v));
        }
    }
    return trailheads;
}
