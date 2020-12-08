use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read file");

    let mut numbers = Vec::new();
    for line in contents.trim().split("\n") {
        let num: i32 = line.parse().unwrap();
        numbers.push(num);
    }
    for num1 in &numbers {
        for num2 in &numbers {
            for num3 in &numbers {
                if num1 + num2 + num3 == 2020 {
                    println!("Numbers: {},{},{} ({})", num1, num2, num3, num1 * num2 * num3);
                }
            }
        }
    }
}
