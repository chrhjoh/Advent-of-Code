use core::panic;

use aocsuite::utils;

type Layout = Vec<Vec<Seat>>;
fn exercise1(data: &str) -> i64 {
    let rows = data.lines();
    let mut current_layout: Layout = vec![];
    for (y, row) in rows.enumerate() {
        let mut row_layout: Vec<Seat> = vec![];
        for (x, symbol) in row.chars().enumerate() {
            row_layout.push(Seat::new(symbol, x as isize, y as isize))
        }
        current_layout.push(row_layout)
    }
    while count_status_changes(&current_layout).ne(&0) {
        print_layout(&current_layout);
        let mut next_layout: Layout = vec![];
        for row in current_layout.iter() {
            let row_layout = row
                .iter()
                .map(|seat| seat.update_status(&current_layout))
                .collect();
            next_layout.push(row_layout)
        }
        current_layout = next_layout;
    }

    return count_occupied_seats(&current_layout) as i64;
}
fn exercise2(data: &str) -> i64 {
    let rows = data.lines();
    let mut current_layout: Layout = vec![];
    for (y, row) in rows.enumerate() {
        let mut row_layout: Vec<Seat> = vec![];
        for (x, symbol) in row.chars().enumerate() {
            row_layout.push(Seat::new(symbol, x as isize, y as isize))
        }
        current_layout.push(row_layout)
    }
    while count_status_changes(&current_layout).ne(&0) {
        print_layout(&current_layout);
        let mut next_layout: Layout = vec![];
        for row in current_layout.iter() {
            let row_layout = row
                .iter()
                .map(|seat| seat.update_status_2(&current_layout))
                .collect();
            next_layout.push(row_layout)
        }
        current_layout = next_layout;
    }

    return count_occupied_seats(&current_layout) as i64;
}

fn main() {
    utils::run(exercise1, exercise2);
}
#[derive(Debug)]
enum SeatStatus {
    Empty,
    Occupied,
    Floor,
}

struct Seat {
    status: SeatStatus,
    x: isize,
    y: isize,
    status_changed: bool,
}

impl Seat {
    fn new(seat: char, x: isize, y: isize) -> Seat {
        let status = match seat {
            'L' => SeatStatus::Empty,
            '#' => SeatStatus::Occupied,
            '.' => SeatStatus::Floor,
            _ => panic!(),
        };
        Seat {
            status: status,
            x: x,
            y: y,
            status_changed: true,
        }
    }
    fn update_status(self: &Seat, layout: &Layout) -> Seat {
        let seats = get_surronding_seats(self, layout);
        let new_status: SeatStatus;
        let status_change: bool;
        let occupied_seats = seats
            .iter()
            .filter(|seat| matches!(seat.status, SeatStatus::Occupied))
            .count();
        if matches!(self.status, SeatStatus::Empty) && occupied_seats.eq(&0) {
            new_status = SeatStatus::Occupied;
            status_change = true;
        } else if matches!(self.status, SeatStatus::Occupied) && occupied_seats.ge(&4) {
            new_status = SeatStatus::Empty;
            status_change = true;
        } else {
            new_status = match self.status {
                SeatStatus::Empty => SeatStatus::Empty,
                SeatStatus::Floor => SeatStatus::Floor,
                SeatStatus::Occupied => SeatStatus::Occupied,
            };
            status_change = false;
        }
        Seat {
            status: new_status,
            x: self.x,
            y: self.y,
            status_changed: status_change,
        }
    }
    fn update_status_2(self: &Seat, layout: &Layout) -> Seat {
        let seats = get_surronding_seats_2(self, layout);
        let new_status: SeatStatus;
        let status_change: bool;
        let occupied_seats = seats
            .iter()
            .filter(|seat| matches!(seat.status, SeatStatus::Occupied))
            .count();
        if matches!(self.status, SeatStatus::Empty) && occupied_seats.eq(&0) {
            new_status = SeatStatus::Occupied;
            status_change = true;
        } else if matches!(self.status, SeatStatus::Occupied) && occupied_seats.ge(&5) {
            new_status = SeatStatus::Empty;
            status_change = true;
        } else {
            new_status = match self.status {
                SeatStatus::Empty => SeatStatus::Empty,
                SeatStatus::Floor => SeatStatus::Floor,
                SeatStatus::Occupied => SeatStatus::Occupied,
            };
            status_change = false;
        }
        Seat {
            status: new_status,
            x: self.x,
            y: self.y,
            status_changed: status_change,
        }
    }
}

fn get_surronding_seats<'a>(seat: &Seat, layout: &'a Layout) -> Vec<&'a Seat> {
    let x_min = (seat.x - 1).max(0);
    let y_min = (seat.y - 1).max(0);
    let x_max = (seat.x + 1).min((layout.iter().nth(0).unwrap().len() - 1) as isize);
    let y_max = (seat.y + 1).min((layout.len() - 1) as isize);
    let mut neighbours: Vec<&Seat> = vec![];
    for x in x_min..x_max + 1 {
        for y in y_min..y_max + 1 {
            if !(x.eq(&seat.x) && y.eq(&seat.y)) {
                neighbours.push(&layout[y as usize][x as usize])
            }
        }
    }
    return neighbours;
}
fn get_surronding_seats_2<'a>(seat: &Seat, layout: &'a Layout) -> Vec<&'a Seat> {
    let x_min = (seat.x - 1).max(0);
    let y_min = (seat.y - 1).max(0);
    let x_max = (seat.x + 1).min((layout.iter().nth(0).unwrap().len() - 1) as isize);
    let y_max = (seat.y + 1).min((layout.len() - 1) as isize);
    let mut neighbours: Vec<&Seat> = vec![];
    for x in x_min..x_max + 1 {
        for y in y_min..y_max + 1 {
            if !(x.eq(&seat.x) && y.eq(&seat.y)) {
                let (mut look_x, mut look_y) = (x, y);
                let mut potential_neighbour = &layout[look_y as usize][look_x as usize];
                let mut skip = false;

                while matches!(potential_neighbour.status, SeatStatus::Floor) {
                    look_x += x - seat.x;
                    look_y += y - seat.y;
                    if look_x.ge(&isize::try_from(layout.iter().nth(0).unwrap().len()).unwrap())
                        | look_x.lt(&0)
                        | look_y.ge(&isize::try_from(layout.len()).unwrap())
                        | look_y.lt(&0)
                    {
                        skip = true;
                        break;
                    } else if look_x.eq(&seat.x) && look_y.eq(&seat.y) {
                        continue;
                    }
                    potential_neighbour = &layout[look_y as usize][look_x as usize];
                }
                if !skip {
                    neighbours.push(potential_neighbour)
                }
            }
        }
    }
    return neighbours;
}

fn count_status_changes(layout: &Layout) -> usize {
    layout
        .iter()
        .map(|row| row.iter().filter(|seat| seat.status_changed).count())
        .sum()
}

fn count_occupied_seats(layout: &Layout) -> usize {
    layout
        .iter()
        .map(|row| {
            row.iter()
                .filter(|seat| matches!(seat.status, SeatStatus::Occupied))
                .count()
        })
        .sum()
}

fn print_layout(layout: &Layout) {
    let row_strings: Vec<String> = layout
        .iter()
        .map(|row| {
            row.iter()
                .map(|seat| match seat.status {
                    SeatStatus::Empty => 'L',
                    SeatStatus::Occupied => '#',
                    SeatStatus::Floor => '.',
                })
                .collect::<String>()
        })
        .collect();
    println!("{}\n", row_strings.join("\n"));
}
