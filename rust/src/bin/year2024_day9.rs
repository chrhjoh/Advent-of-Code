use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let input = parse_input(data);
    let disk = build_disk(input);
    println!("{:?}", disk);
    let ordered_disk = reorder_disk1(disk);
    println!("{:?}", ordered_disk);
    return checksum(ordered_disk);
}
fn exercise2(data: &str) -> i64 {
    let input = parse_input(data);
    let disk = build_disk(input);
    println!("{:?}", disk);
    let ordered_disk = reorder_disk2(disk);
    println!("{:?}", ordered_disk);
    return checksum(ordered_disk);
}

fn main() {
    utils::run(exercise1, exercise2);
}

fn parse_input(input: &str) -> Vec<i64> {
    return input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();
}

fn build_disk(dense_format: Vec<i64>) -> Vec<DiskItem> {
    let mut disk: Vec<DiskItem> = Vec::new();
    let mut id = 0;
    for (i, val) in dense_format.iter().enumerate() {
        if i % 2 != 0 {
            for _ in 0..*val {
                disk.push(DiskItem::Empty);
            }
        } else {
            for _ in 0..*val {
                disk.push(DiskItem::File(id, *val));
            }
            id += 1;
        }
    }
    return disk;
}

fn reorder_disk1(disk: Vec<DiskItem>) -> Vec<DiskItem> {
    let mut new_disk = disk;
    let mut swap_index = new_disk.len() - 1;
    let mut index = 0;
    while index < swap_index {
        match new_disk[index] {
            DiskItem::File(_, _) => {
                index += 1;
            }
            DiskItem::Empty => match new_disk[swap_index] {
                DiskItem::File(_, _) => {
                    new_disk.swap(index, swap_index);
                    index += 1;
                    swap_index -= 1;
                }
                DiskItem::Empty => {
                    swap_index -= 1;
                }
            },
        }
    }
    return new_disk;
}

fn reorder_disk2(disk: Vec<DiskItem>) -> Vec<DiskItem> {
    let mut new_disk = disk;
    let mut swap_index = new_disk.len() as isize - 1 as isize;
    while 0 <= swap_index {
        match new_disk[swap_index as usize] {
            DiskItem::Empty => {
                swap_index -= 1;
            }
            DiskItem::File(_, size) => {
                let fit_index = find_file_fit(&new_disk, swap_index as usize, size);
                match fit_index {
                    Some(fit_index) => {
                        new_disk = swap_whole_file(new_disk, fit_index, swap_index as usize, size);
                        swap_index -= size as isize;
                    }
                    None => {
                        swap_index -= size as isize;
                    }
                }
            }
        }
    }
    return new_disk;
}
fn find_file_fit(disk: &Vec<DiskItem>, max_index: usize, file_size: i64) -> Option<usize> {
    let mut count = 0;
    for i in 0..max_index {
        match disk[i] {
            DiskItem::File(_, _) => {
                count = 0;
            }
            DiskItem::Empty => {
                count += 1;
                if count == file_size {
                    return Some(i - file_size as usize + 1);
                }
            }
        }
    }
    return None;
}
fn swap_whole_file(
    disk: Vec<DiskItem>,
    index: usize,
    swap_index: usize,
    file_size: i64,
) -> Vec<DiskItem> {
    let mut new_disk = disk;
    for i in 0..file_size as usize {
        new_disk.swap(index + i, swap_index - i);
    }
    return new_disk;
}
fn checksum(disk: Vec<DiskItem>) -> i64 {
    return disk
        .iter()
        .enumerate()
        .map(|(i, val)| match val {
            DiskItem::File(id, _) => i as i64 * id,
            DiskItem::Empty => 0,
        })
        .sum();
}

#[derive(Debug)]
enum DiskItem {
    File(i64, i64),
    Empty,
}
