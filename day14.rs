use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read file");

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask_ones: u64 = 0;
    let mut mask_floating: u64 = 0;
    for line in contents.trim().split("\n") {
        let parts: Vec<&str> = line.split("=").map(|x| x.trim()).collect();
        if parts[0] == "mask" {
            mask_ones = 0;
            mask_floating = 0;
            let mask_str = parts[1];
            for (i, c) in mask_str.chars().enumerate() {
                match c {
                    '0' => (),
                    '1' => mask_ones |= 1 << (mask_str.len() - i - 1),
                    'X' => mask_floating |= 1 << (mask_str.len() - i - 1),
                    _ => panic!("Invalid mask char"),
                }
            }
        } else if parts[0][0..=2] == *"mem" {
            let addr: u64 = (parts[0][4..=parts[0].find(']').unwrap()-1].parse::<u64>().unwrap() | mask_ones) & !mask_floating;
            let mut value: u64 = parts[1].parse::<u64>().unwrap();
            let mut addrs: HashSet<u64> = HashSet::new();
            addrs.insert(addr);
            for i in 0..=35 {
                if mask_floating & (1 << i) != 0 {
                    let mut addrs_new: HashSet<u64> = HashSet::new();
                    for a in addrs {
                        addrs_new.insert(a);
                        addrs_new.insert(a | (1 << i));
                    }
                    addrs = addrs_new;
                }
            }
            for a in addrs {
                mem.insert(a, value);
            }
        } else {
            panic!("Invalid parts: {:?}", parts);
        }
    }
    println!("{}", mem.values().sum::<u64>());
}
