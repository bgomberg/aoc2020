use std::fs;

const NUM_COLS: usize = 31;
const NUM_ROWS: usize = 323;
const SLOPE_X: usize = 1;
const SLOPE_Y: usize = 2;

fn has_tree(map: [[bool; NUM_COLS]; NUM_ROWS], x: usize, y: usize) -> bool {
    map[y % NUM_ROWS][x % NUM_COLS]
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read file");

    let mut map = [[false; NUM_COLS]; NUM_ROWS];
    for (y, line) in contents.trim().split("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            map[y][x] = c == '#';
        }
    }

    let mut x = 0;
    let mut y = 0;
    let mut num_trees = 0;
    while y < NUM_ROWS {
        if has_tree(map, x, y) {
            num_trees += 1;
        }
        x += SLOPE_X;
        y += SLOPE_Y;
    }
    println!("{}", num_trees);
}
