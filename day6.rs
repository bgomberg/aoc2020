use std::fs;
use std::collections::HashMap;

fn get_num_questions(s: &String) -> usize {
    let mut num = 0;
    let mut map: HashMap<char, i32> = HashMap::new();
    for line in s.trim().split("\n") {
        for c in line.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
    }
    for (_, n) in &map {
        if *n == s.trim().split("\n").collect::<Vec<&str>>().len() as i32 {
            num += 1
        }
    }
    num
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read file");

    let mut num_questions = 0;
    let mut buf: String = "".to_owned();
    for line in contents.trim().split("\n") {
        if line.trim() != "" {
            if buf == "" {
                buf = format!("{}", line.trim());
            } else {
               buf = format!("{}\n{}", buf, line.trim());
            }
            continue;
        }
        num_questions += get_num_questions(&buf);
        buf = "".to_owned();
    }
    num_questions += get_num_questions(&buf);
    println!("{}", num_questions);
}
