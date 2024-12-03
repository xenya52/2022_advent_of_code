use std::fs::read_to_string;

fn read_file_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn count_valid_vectors(file_output: Vec<String>) -> i32 {
    let mut count = 0;

    for line in file_output {
        let split_line: Vec<i32> = line.split(" ").map(|s| s.parse().unwrap()).collect();
        let mut is_valid: bool = true;
        let mut prev_val: i32 = split_line[0];

        for i in 0..split_line.len() {
            if i == 0 {
                continue;
            }
            if prev_val - split_line[i] > 3 || prev_val - split_line[i] < 1 {
                is_valid = false;
                break;
            }
            prev_val = split_line[i];
        }
        if is_valid {
            count += 1;
        } else {
            is_valid = true;
            for i in 0..split_line.len() {
                if i == 0 {
                    continue;
                }

                if split_line[i] - prev_val > 3 || split_line[i] - prev_val < 1 {
                    is_valid = false;
                    break;
                }
                prev_val = split_line[i];
            }
            if is_valid {
                count += 1;
            }
        }
    }
    return count;
}

fn main() {
    let file_output: Vec<String> = read_file_lines("./../input.txt");
    let result = count_valid_vectors(file_output);
    println!("Result: {}", result);
}
