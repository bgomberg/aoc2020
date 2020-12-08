use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read file");

    let mut num_valid = 0;
    for line in contents.trim().split("\n") {
        let parts: Vec<&str> = line.split(":").map(|val| val.trim()).collect();
        let parts2: Vec<&str> = parts[0].split(" ").collect();
        let c = parts2[1].chars().nth(0).unwrap();
        let parts3: Vec<usize> = parts2[0].split("-").map(|val| val.parse().unwrap()).collect();
        let (min, max) = (parts3[0], parts3[1]);
        let password = parts[1];
        if (password.chars().nth(min - 1).unwrap() == c) != (password.chars().nth(max - 1).unwrap() == c) {
            num_valid += 1;
        }
    }
    println!("{}", num_valid);
}
