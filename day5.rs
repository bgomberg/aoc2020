use std::fs;

fn get_seat_id(line: &str) -> i32 {
    let (row_str, col_str) = (&line[0..7], &line[7..10]);
    let mut row = 0;
    for (i, c) in row_str.chars().enumerate() {
        row |= match c {
            'F' => 0,
            'B' => 1 << (6 - i),
            _ => panic!("INVALID"),
        }
    }
    let mut col = 0;
    for (i, c) in col_str.chars().enumerate() {
        col |= match c {
            'L' => 0,
            'R' => 1 << (2 - i),
            _ => panic!("INVALID"),
        }
    }
    row * 8 + col
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read file");

    let mut ids = Vec::new();
    for line in contents.trim().split("\n") {
        ids.push(get_seat_id(line));
    }
    ids.sort();
    let mut prev_id = -1;
    for id in ids {
        if prev_id != -1 && id != prev_id + 1 {
            println!("{}, {}", prev_id, id);
        }
        prev_id = id;
    }
}
