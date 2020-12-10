use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read file");

    let mut numbers: Vec<i64> = contents.trim().split("\n").map(|n| { n.trim().parse().unwrap() }).collect();
    numbers.sort();

    // part 1
    let mut prev_n: i64 = 0;
    let mut counts = HashMap::new();
    for i in 0..=numbers.len() {
        let diff = if i == numbers.len() { 3 } else { numbers[i] - prev_n};
        let count = counts.entry(diff).or_insert(0);
        *count += 1;
        if i != numbers.len() {
            prev_n = numbers[i];
        }
    }
    println!("{:?}", counts);

    // part 2
    let mut segments: Vec<Vec<i64>> = Vec::new();
    let mut current_segment: Vec<i64> = Vec::new();
    let mut prev_n: i64 = 0;
    current_segment.push(0);
    for n in numbers {
        if n - prev_n == 3 {
            if current_segment.len() > 2 {
                segments.push(current_segment);
            }
            current_segment = Vec::new();
        }
        current_segment.push(n);
        prev_n = n;
    }
    segments.push(current_segment);
    let mut result: i64 = 1;
    for segment in segments {
        let l = segment.len() as i64 - 2;
        result *= l * (l + 1) / 2 + 1;
    }
    println!("{:?}", result);
}
