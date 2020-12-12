use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read file");

    let mut waypoint_x = 10;
    let mut waypoint_y = 1;
    let mut pos_x = 0;
    let mut pos_y = 0;
    for line in contents.trim().split("\n") {
        let line = line.trim();
        let action = line.chars().nth(0).unwrap();
        let value: i32 = line[1..].parse().unwrap();
        match action {
            'N' => {
                waypoint_y += value;
            },
            'S' => {
                waypoint_y -= value;
            },
            'E' => {
                waypoint_x += value;
            },
            'W' => {
                waypoint_x -= value;
            },
            'L' => {
                match value {
                    0 => (),
                    90 => {
                        let tmp = waypoint_y;
                        waypoint_y = waypoint_x;
                        waypoint_x = -tmp;
                    },
                    180 => {
                        waypoint_x = -waypoint_x;
                        waypoint_y = -waypoint_y;
                    },
                    270 => {
                        let tmp = waypoint_y;
                        waypoint_y = -waypoint_x;
                        waypoint_x = tmp;
                    },
                    _ => panic!("Invalid angle"),
                }
            },
            'R' => {
                match value {
                    0 => (),
                    90 => {
                        let tmp = waypoint_y;
                        waypoint_y = -waypoint_x;
                        waypoint_x = tmp;
                    },
                    180 => {
                        waypoint_x = -waypoint_x;
                        waypoint_y = -waypoint_y;
                    },
                    270 => {
                        let tmp = waypoint_y;
                        waypoint_y = waypoint_x;
                        waypoint_x = -tmp;
                    },
                    _ => panic!("Invalid angle"),
                }
            },
            'F' => {
                pos_x += waypoint_x * value;
                pos_y += waypoint_y * value;
            },
            _ => panic!("Invalid action ({})", action),
        }
    }
    println!("{}", pos_x.abs() + pos_y.abs());
}
