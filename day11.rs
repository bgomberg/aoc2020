use std::fs;

fn get_direction(seats: &Vec<Vec<char>>, row: usize, col: usize, row_dir: i32, col_dir: i32) -> bool {
    let row = row as i32 + row_dir;
    let col = col as i32 + col_dir;
    if row < 0 || col < 0 || row >= seats.len() as i32 || col >= seats[0].len() as i32 {
        return false;
    }
    match seats[row as usize][col as usize] {
        '#' => return true,
        'L' => return false,
        '.' => return get_direction(seats, row as usize, col as usize, row_dir, col_dir),
        _ => panic!("Invalid value"),
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read file");

    let mut seats: Vec<Vec<char>> = Vec::new();
    for line in contents.trim().split("\n") {
        let row: Vec<char> = line.trim().chars().collect();
        seats.push(row);
    }

    loop {
        let mut has_change = false;
        let mut next_seats: Vec<Vec<char>> = Vec::new();
        for row in 0..=seats.len()-1 {
            let mut next_row: Vec<char> = Vec::new();
            for col in 0..=seats[row].len()-1 {
                let mut num_neighbors = 0;
                // up-left
                if get_direction(&seats, row, col, -1, -1) {
                    num_neighbors += 1;
                }
                // up
                if get_direction(&seats, row, col, -1, 0) {
                    num_neighbors += 1;
                }
                // up-right
                if get_direction(&seats, row, col, -1, 1) {
                    num_neighbors += 1;
                }
                // left
                if get_direction(&seats, row, col, 0, -1) {
                    num_neighbors += 1;
                }
                // right
                if get_direction(&seats, row, col, 0, 1) {
                    num_neighbors += 1;
                }
                // down-left
                if get_direction(&seats, row, col, 1, -1) {
                    num_neighbors += 1;
                }
                // down
                if get_direction(&seats, row, col, 1, 0) {
                    num_neighbors += 1;
                }
                // down-right
                if get_direction(&seats, row, col, 1, 1) {
                    num_neighbors += 1;
                }
                let next_value = match seats[row][col] {
                    '.' => '.',
                    'L' => if num_neighbors == 0 { '#' } else { 'L' },
                    '#' => if num_neighbors >= 5 { 'L' } else { '#' },
                    _ => panic!("Invalid char"),
                };
                has_change = has_change || next_value != seats[row][col];
                next_row.push(next_value);
            }
            next_seats.push(next_row);
        }
        seats = next_seats;
        if !has_change {
            break;
        }
    }

    let mut num_occupied = 0;
    for row in seats {
        for chair in row {
            num_occupied += if chair == '#' { 1 } else { 0 };
        }
    }
    println!("{:?}", num_occupied);
}
