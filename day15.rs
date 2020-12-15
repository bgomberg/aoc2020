use std::collections::HashMap;


#[derive(Debug)]
struct ValueInfo {
    prev_turn: usize,
    prev_prev_turn: Option<usize>,
}

fn main() {
    let inital_numbers = vec![1,2,16,19,18,0];
    let mut numbers: Vec<i32> = Vec::new();
    let mut info: HashMap<i32, ValueInfo> = HashMap::new();
    for (i, n) in inital_numbers.iter().enumerate() {
        info.insert(*n, ValueInfo {
            prev_turn: i + 1,
            prev_prev_turn: None,
        });
        numbers.push(*n);
    }

    let mut prev_value = *inital_numbers.last().unwrap();
    for turn in (inital_numbers.len() + 1)..=30000000 {
        let value;
        if info[&prev_value].prev_prev_turn == None {
            value = 0;
        } else {
            value = (info[&prev_value].prev_turn - info[&prev_value].prev_prev_turn.unwrap()) as i32;
        }
        if info.contains_key(&value) {
            let prev_info = info.remove(&value).unwrap();
            info.insert(value, ValueInfo {
                prev_turn: turn,
                prev_prev_turn: Some(prev_info.prev_turn),
            });
        } else {
            info.insert(value, ValueInfo {
                prev_turn: turn,
                prev_prev_turn: None,
            });
        }
        numbers.push(value);
        prev_value = value;
    }
    println!("{}", prev_value);
}
