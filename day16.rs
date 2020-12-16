use std::fs;
use std::collections::HashSet;

#[derive(Debug)]
struct Rule {
    range1_start: i64,
    range1_end: i64,
    range2_start: i64,
    range2_end: i64,
    index: Option<usize>,
}

impl Rule {
    fn is_value_valid(&self, value: i64) -> bool {
        (value >= self.range1_start && value <= self.range1_end) || (value >= self.range2_start && value <= self.range2_end)
    }
}

fn parse_rules(contents: &str) -> Vec<Rule> {
    let mut rules: Vec<Rule> = Vec::new();
    for rule_line in contents.split("\n") {
        let nums: Vec<i64> = rule_line.split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split("or")
            .map(|x| x.trim().split("-"))
            .flatten()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let rule = Rule {
            range1_start: nums[0],
            range1_end: nums[1],
            range2_start: nums[2],
            range2_end: nums[3],
            index: None,
        };
        rules.push(rule);
    }
    return rules;
}

fn parse_my_ticket(contents: &str) -> Vec<i64> {
    let line = contents.split("\n").nth(1).unwrap();
    return line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
}

fn parse_nearby_tickets(contents: &str) -> Vec<Vec<i64>> {
    let mut tickets: Vec<Vec<i64>> = Vec::new();
    for line in contents.split("\n").collect::<Vec<&str>>()[1..].iter() {
        tickets.push(line.split(",").map(|x| x.parse::<i64>().unwrap()).collect());
    }
    return tickets;
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read file");
    let contents: Vec<&str> = contents.split("\n\n").collect();
    let mut rules = parse_rules(contents[0].trim());
    let my_ticket = parse_my_ticket(contents[1].trim());
    let nearby_tickets = parse_nearby_tickets(contents[2].trim());

    let mut cols: Vec<Vec<i64>> = Vec::new();
    for _ in 0..=my_ticket.len()-1 {
        cols.push(Vec::new());
    }
    let mut error_rate = 0;
    for ticket in nearby_tickets {
        let mut is_ticket_valid = true;
        for value in &ticket {
            let value = *value;
            let mut is_value_valid = false;
            for rule in rules.iter() {
                if rule.is_value_valid(value) {
                    is_value_valid = true;
                    break;
                }
            }
            if !is_value_valid {
                is_ticket_valid = false;
                error_rate += value;
            }
        }
        if is_ticket_valid {
            for (i, col) in cols.iter_mut().enumerate() {
                col.push(ticket[i]);
            }
        }
    }
    println!("Part 1: {}", error_rate);

    for (i, col) in cols.iter_mut().enumerate() {
        col.push(my_ticket[i]);
    }
    let mut found_col_indexes: HashSet<usize> = HashSet::new();
    loop {
        let mut made_progress = false;
        for (i, col) in cols.iter().enumerate() {
            if found_col_indexes.contains(&i) {
                continue;
            }
            let mut num_matches = 0;
            let mut matching_rule_index = 0;
            for (j, rule) in rules.iter().enumerate() {
                if rule.index == None && col.iter().all(|x| rule.is_value_valid(*x)) {
                    num_matches += 1;
                    matching_rule_index = j;
                }
            }
            if num_matches == 1 {
                rules.get_mut(matching_rule_index).unwrap().index = Some(i);
                made_progress = true;
                found_col_indexes.insert(i);
            }
        }
        if !made_progress {
            break;
        }
    }
    assert!(found_col_indexes.len() == cols.len());
    let mut result = 1;
    for i in 0..=5 {
        result *= my_ticket[rules[i].index.unwrap()];
    }
    println!("Part 2: {}", result);
}
