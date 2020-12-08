use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Content {
    quantity: i32,
    color: String,
}

#[derive(Debug)]
struct Rule {
    color: String,
    contents: Vec<Content>,
}

fn get_parents(rules: &Vec<Rule>, color: &str, ancestors: &mut HashSet<String>) -> bool {
    let mut has_new = false;
    for rule in rules {
        for content in rule.contents.iter() {
            if content.color == color {
                let s = String::from(&rule.color);
                has_new = ancestors.insert(s) || has_new;
                break;
            }
        }
    }
    return has_new;
}

fn get_num_bags(rules: &Vec<Rule>, color: &str) -> i32 {
    let mut num = 1;
    for rule in rules {
        if rule.color == color {
            for content in rule.contents.iter() {
                num += content.quantity * get_num_bags(rules, &content.color);
            }
        }
    }
    num
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read file");

    let mut rules: Vec<Rule> = Vec::new();
    for line in contents.trim().split("\n") {
        let mut rule = Rule {color: String::new(), contents: Vec::new()};
        let mut index = 0;
        for word in line.trim().split(" ") {
            match index {
                0 => {
                    rule.color.push_str(word);
                    index += 1;
                },
                1 => {
                    rule.color.push_str(" ");
                    rule.color.push_str(word);
                    index += 1;
                },
                2..=3 => { // "bags contain"
                    index += 1;

                },
                4 => {
                    match word {
                        "no" => {
                            // no contents, we're done
                            break;
                        },
                        _ => {
                            // should be a number
                            let content = Content {
                                quantity: word.parse().unwrap(),
                                color: String::new(),
                            };
                            rule.contents.push(content);
                            index += 1;
                        }
                    }
                },
                5 => {
                    // should be the first word of the color
                    rule.contents.last_mut().unwrap().color.push_str(word);
                    index += 1;
                },
                6 => {
                    // should be the second word of the color
                    rule.contents.last_mut().unwrap().color.push_str(" ");
                    rule.contents.last_mut().unwrap().color.push_str(word);
                    index += 1;
                }
                7 => {
                    match word {
                        "bags," | "bag," => {
                            // there are more
                            index = 4;
                        },
                        "bags." | "bag." => {
                            // no more
                            index = -1;
                        },
                        _ => panic!("Unexpected word ({})", word),
                    }
                }
                _ => panic!("Unexpected index ({}, '{}')", index, word),
            }
        }
        rules.push(rule);
    }

    // part 1
    let mut ancestors: HashSet<String> = HashSet::new();
    let current_color = "shiny gold";
    get_parents(&rules, &current_color, &mut ancestors);
    loop {
        let mut has_new = false;
        let mut stack: Vec<String> = Vec::new();
        for color in &ancestors {
            stack.push(color.to_string());
        }
        for color in stack {
            has_new = get_parents(&rules, &color, &mut ancestors) || has_new;
        }
        if !has_new {
            break;
        }
    }
    println!("{}", ancestors.len());

    // part 2
    println!("{}", get_num_bags(&rules, "shiny gold") - 1);
}
