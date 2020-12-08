use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
enum Op {
    JMP,
    NOP,
    ACC,
}

#[derive(Debug)]
struct Instr {
    op: Op,
    arg: i32,
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read file");

    let mut program: Vec<Instr> = Vec::new();
    for line in contents.trim().split("\n") {
        let parts: Vec<&str> = line.trim().split(" ").collect();
        let instr: Instr = Instr {
            op: match parts[0] {
                "jmp" => Op::JMP,
                "nop" => Op::NOP,
                "acc" => Op::ACC,
                _ => panic!("Invalid instruction"),
            },
            arg: parts[1].parse().unwrap(),
        };
        program.push(instr);
    }

    for change_index in 0..=program.len() {
        let mut executed: HashSet<i32> = HashSet::new();
        let mut acc = 0;
        let mut pc = 0;
        loop {
            executed.insert(pc);
            let instr: &Instr = &program[pc as usize];
            match instr.op {
                Op::JMP => {
                    if pc == change_index as i32 {
                        pc += 1;
                    } else {
                        pc += instr.arg;
                    }
                },
                Op::NOP => {
                    if pc == change_index as i32 {
                        pc += instr.arg;
                    } else {
                        pc += 1;
                    }
                },
                Op::ACC => {
                    acc += instr.arg;
                    pc += 1;
                },
            }
            if executed.contains(&pc) {
                break;
            } else if pc as usize == program.len() {
                println!("Success! ({}, {})", change_index, acc);
                return;
            }
        }
    }
}
