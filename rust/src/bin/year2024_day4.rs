use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let grid = parse_input(data);
    let x_starts = find_all_x(&grid);
    x_starts
        .iter()
        .map(|(x, y)| {
            check_all_directions(&grid, *x, *y)
                .iter()
                .filter(|x| **x)
                .count()
        })
        .sum::<usize>() as i64
}
fn exercise2(data: &str) -> i64 {
    let grid = parse_input(data);
    let a_starts = find_all_a(&grid);
    let result = a_starts
        .iter()
        .map(|(x, y)| check_directions_2(&grid, *x, *y))
        .filter(|x| *x)
        .count();
    return result as i64;
}

fn main() {
    utils::run(exercise1, exercise2);
}

#[derive(Debug)]
enum Xmas {
    X,
    M,
    A,
    S,
    O,
}
type Grid = Vec<Vec<Xmas>>;

fn parse_input(data: &str) -> Grid {
    let mut xmas = Vec::new();
    for line in data.lines() {
        let mut xmas_line = Vec::new();
        for c in line.chars() {
            match c {
                'X' => xmas_line.push(Xmas::X),
                'M' => xmas_line.push(Xmas::M),
                'A' => xmas_line.push(Xmas::A),
                'S' => xmas_line.push(Xmas::S),
                _ => xmas_line.push(Xmas::O),
            }
        }
        xmas.push(xmas_line);
    }
    return xmas;
}

fn find_all_a(grid: &Grid) -> Vec<(usize, usize)> {
    let mut x_positions = Vec::new();
    for (y, line) in grid.iter().enumerate() {
        for (x, xmas) in line.iter().enumerate() {
            match xmas {
                Xmas::A => x_positions.push((x, y)),
                _ => (),
            }
        }
    }
    return x_positions;
}
fn find_all_x(grid: &Grid) -> Vec<(usize, usize)> {
    let mut x_positions = Vec::new();
    for (y, line) in grid.iter().enumerate() {
        for (x, xmas) in line.iter().enumerate() {
            match xmas {
                Xmas::X => x_positions.push((x, y)),
                _ => (),
            }
        }
    }
    return x_positions;
}

fn check_direction(grid: &Grid, x: usize, y: usize, dx: i32, dy: i32) -> bool {
    let mut point_x = x as i32;
    let mut point_y = y as i32;
    let mut position = 0;

    while position < 4 {
        if point_x < 0
            || point_y < 0
            || point_x >= grid[0].len() as i32
            || point_y >= grid.len() as i32
        {
            println!("Out of bounds");
            return false;
        }
        match grid[point_y as usize][point_x as usize] {
            Xmas::X => {
                if position != 0 {
                    return false;
                }
            }
            Xmas::M => {
                if position != 1 {
                    return false;
                }
            }
            Xmas::A => {
                if position != 2 {
                    return false;
                }
            }
            Xmas::S => {
                if position != 3 {
                    return false;
                } else {
                    return true;
                }
            }
            Xmas::O => return false,
        }
        point_x += dx;
        point_y += dy;
        position += 1;
    }
    return false;
}

fn check_all_directions(grid: &Grid, x: usize, y: usize) -> Vec<bool> {
    let directions = vec![
        (0, 1),
        (1, 0),
        (1, 1),
        (1, -1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
    ];
    let result = directions
        .iter()
        .map(|(dx, dy)| check_direction(grid, x, y, *dx, *dy))
        .collect();
    return result;
}

fn check_directions_2(grid: &Grid, x: usize, y: usize) -> bool {
    let corners = vec![(-1, -1), (-1, 1), (1, -1), (1, 1)];
    let m_locations = corners
        .iter()
        .map(|(dx, dy)| {
            let point_x = x as i32 + dx;
            let point_y = y as i32 + dy;
            if point_x < 0
                || point_y < 0
                || point_x >= grid[0].len() as i32
                || point_y >= grid.len() as i32
            {
                return None;
            }
            match grid[point_y as usize][point_x as usize] {
                Xmas::M => return Some((*dx, *dy)),
                _ => return None,
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<(i32, i32)>>();

    if m_locations.len() != 2 {
        return false;
    }

    for (dx, dy) in m_locations {
        let point_x = x as i32 - dx;
        let point_y = y as i32 - dy;
        if point_x < 0
            || point_y < 0
            || point_x >= grid[0].len() as i32
            || point_y >= grid.len() as i32
        {
            return false;
        }
        match grid[point_y as usize][point_x as usize] {
            Xmas::S => {
                {};
            }
            _ => return false,
        }
    }

    return true;
}
