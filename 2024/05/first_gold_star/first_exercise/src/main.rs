use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_rules_from_file(file_path: &str) -> Vec<(u32, u32)> {
    let mut rules = Vec::new();
    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if let Some((left, right)) = parse_rule(&line) {
                    rules.push((left, right));
                }
            }
        }
    }
    return rules;
}

fn parse_rule(line: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() == 2 {
        if let (Ok(left), Ok(right)) = (
            parts[0].trim().parse::<u32>(),
            parts[1].trim().parse::<u32>(),
        ) {
            return Some((left, right));
        }
    }
    return None;
}

fn parse_updates(file_path: &str) -> Vec<Vec<u32>> {
    let mut updates = Vec::new();
    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if !line.contains('|') {
                    let update: Vec<u32> = line
                        .split(',')
                        .filter_map(|s| s.trim().parse::<u32>().ok())
                        .collect();
                    updates.push(update);
                }
            }
        }
    }
    return updates;
}

fn is_update_in_order(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    if update.len() < 1 {
        return false;
    }
    for i in 1..update.len() {
        let left = update[i - 1];
        let right = update[i];
        let mut found = false;

        for rule in rules {
            if rule.0 == left && rule.1 == right {
                found = true;
                break;
            }
        }

        if !found {
            return false;
        }
    }
    return true;
}

fn calculation(updates: &Vec<Vec<u32>>, rules: &Vec<(u32, u32)>) -> u32 {
    let mut sum = 0;

    for update in updates {
        if is_update_in_order(&update, &rules) {
            let middle_index = update.len() / 2;
            sum += update[middle_index];
        }
    }
    return sum;
}

fn main() {
    let file_path = "./../input.txt";
    let rules = parse_rules_from_file(file_path);
    let updates = parse_updates(file_path);
    let sum = calculation(&updates, &rules);
    println!("Sum: {}", sum);
}
