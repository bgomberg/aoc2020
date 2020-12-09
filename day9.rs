use std::fs;

const PREAMBLE_LENGTH: usize = 25;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read file");

    let numbers: Vec<i64> = contents.trim().split("\n").map(|n| { n.trim().parse().unwrap() }).collect();
    let len = numbers.len();

    // part 1
    let mut invalid_number: i64 = 0;
    for (i, n) in numbers.iter().enumerate() {
        if i >= PREAMBLE_LENGTH {
            let mut is_valid = false;
            for j in (i-PREAMBLE_LENGTH)..=(i-1) {
                for k in (i-PREAMBLE_LENGTH)..=(i-1) {
                    if j != k && (numbers[j] + numbers[k]) == *n {
                        is_valid = true;
                    }
                }
            }
            if !is_valid {
                println!("{} is not valid!", n);
                invalid_number = *n;
            }
        }
    }

    for i in 0..=len-2 {
        for j in i+1..=len-1 {
            let sum: i64 = numbers[i..j+1].iter().sum();
            if sum == invalid_number {
                println!("Weakness is {}", numbers[i..j+1].iter().min().unwrap() + numbers[i..j+1].iter().max().unwrap());
            }
            if sum > invalid_number {
                break;
            }
        }
    }
}
