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

fn main() {
    let file_path = "./../input.txt";
    let rules = parse_rules_from_file(file_path);
    for rule in rules {
        println!("{:?}", rule);
    }
}
