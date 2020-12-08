use std::fs;

fn is_valid(passport: &String) -> bool {
    let mut valid_byr = false;
    let mut valid_iyr = false;
    let mut valid_eyr = false;
    let mut valid_hgt = false;
    let mut valid_hcl = false;
    let mut valid_ecl = false;
    let mut valid_pid = false;
    for field in passport.split(" ") {
        if field == "" {
            continue;
        }
        let parts: Vec<&str> = field.split(":").collect();
        let field_name = parts[0];
        let field_value = parts[1];
        match field_name {
            "byr" => valid_byr = field_value.parse::<i32>().unwrap() >= 1920 && field_value.parse::<i32>().unwrap() <= 2002,
            "iyr" => valid_iyr = field_value.parse::<i32>().unwrap() >= 2010 && field_value.parse::<i32>().unwrap() <= 2020,
            "eyr" => valid_eyr = field_value.parse::<i32>().unwrap() >= 2020 && field_value.parse::<i32>().unwrap() <= 2030,
            "hgt" => {
                if field_value[field_value.len()-2..] == *"cm" {
                    let hgt_cm = field_value[..field_value.len()-2].parse::<i32>().unwrap();
                    valid_hgt = hgt_cm >= 150 && hgt_cm <= 193;
                } else if field_value[field_value.len()-2..] == *"in" {
                    let hgt_in = field_value[..field_value.len()-2].parse::<i32>().unwrap();
                    valid_hgt = hgt_in >= 59 && hgt_in <= 76;
                }
            },
            "hcl" => {
                valid_hcl = field_value.chars().nth(0).unwrap() == '#' && field_value.len() == 7;
                if valid_hcl {
                    for n in 1..=6 {
                        let c = field_value.chars().nth(n).unwrap();
                        valid_hcl = valid_hcl && match c {
                            '0'..='9' | 'a'..='f' => true,
                            _ => false,
                        };
                    }
                }
            },
            "ecl" => valid_ecl = match field_value {
                "amb" | "blu" | "brn" | "grn" | "gry" | "hzl" | "oth" => true,
                _ => false,
            },
            "pid" => valid_pid = field_value.len() == 9 && field_value.parse::<i32>().unwrap() > 0,
            &_ => (),
        }
    }
    valid_byr && valid_iyr && valid_eyr && valid_hgt && valid_hcl && valid_ecl && valid_pid
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read file");

    let mut num_valid = 0;
    let mut passport: String = "".to_owned();
    for line in contents.trim().split("\n") {
        if line.trim() != "" {
            passport = format!("{} {}", passport, line.trim());
            continue;
        }
        num_valid += if is_valid(&passport) { 1 } else { 0 };
        passport = "".to_owned();
    }
    num_valid += if is_valid(&passport) { 1 } else { 0 };
    println!("{} valid", num_valid);
}
