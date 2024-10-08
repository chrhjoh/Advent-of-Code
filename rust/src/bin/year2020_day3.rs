use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    let biome = Biome::new(data);
    let num_trees = run_trajectory(&biome, 3, 1);
    return num_trees;
}
fn exercise2(data: &str) -> i64 {
    let biome = Biome::new(data);
    let slopes = vec![vec![1, 1], vec![3, 1], vec![5, 1], vec![7, 1], vec![1, 2]];
    let num_trees = slopes
        .iter()
        .map(|slope| run_trajectory(&biome, slope[0], slope[1]));
    return num_trees.product();
}

fn main() {
    utils::run(exercise1, exercise2);
}

#[derive(Debug)]
struct Biome {
    tree_lines: Vec<Vec<char>>,
}
impl Biome {
    fn new(data: &str) -> Biome {
        let tree_lines = data.lines().map(|line| line.chars().collect()).collect();
        return Biome {
            tree_lines: tree_lines,
        };
    }
    fn check_for_tree(&self, pointer: &Pointer) -> bool {
        self.tree_lines[pointer.y][pointer.x] == '#'
    }
}

#[derive(Debug)]
struct Pointer {
    x: usize,
    y: usize,
    maximum_x: usize,
}
impl Pointer {
    fn new(maximum_x: usize) -> Pointer {
        return Pointer {
            x: 0,
            y: 0,
            maximum_x: maximum_x,
        };
    }
    fn update_position(&mut self, x: usize, y: usize) {
        self.x = self.x + x;
        self.y = self.y + y;
        if self.x >= self.maximum_x {
            self.x = self.x - self.maximum_x
        }
    }
}

fn run_trajectory(biome: &Biome, x_step: usize, y_step: usize) -> i64 {
    let mut num_trees = 0;
    let mut pointer = Pointer::new(biome.tree_lines[0].len());
    while pointer.y < biome.tree_lines.len() {
        if biome.check_for_tree(&pointer) {
            num_trees += 1;
        }

        pointer.update_position(x_step, y_step);
    }
    return num_trees;
}

