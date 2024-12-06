fn read_file_to_string(filepath: &str) -> String {
    use std::fs::read_to_string;
    read_to_string(filepath).unwrap()
}

fn find_mul(text: &str) -> usize {
    let index: usize = 0;
    for i in 3..text.len() - 1 {
        if text.as_bytes()[i - 3] == b'm'
            && text.as_bytes()[i - 2] == b'u'
            && text.as_bytes()[i - 1] == b'l'
            && text.as_bytes()[i] == b'('
        {
            index = i + 1;
        }
    }
    return index;
}

fn is_valid_mul_instruction(instruction: &str) -> bool {
    if instruction.starts_with("mul(") && instruction.ends_with(")") {
        let mut index = find_mul(instruction);
        let inner = &instruction[index..instruction.len() - 1];
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() == 2 && parts[0].parse::<i32>().is_ok() && parts[1].parse::<i32>().is_ok() {
            return true;
        }
    }
    return false;
}

fn extract_mul_instructions(text: &str) -> Vec<String> {
    let mut instructions = Vec::new();
    let mut start = 0;
    while let Some(start_idx) = text[start..].find("mul(") {
        let start_idx = start + start_idx;
        if let Some(end_idx) = text[start_idx..].find(')') {
            let end_idx = start_idx + end_idx + 1;
            let instruction = &text[start_idx..end_idx];
            if is_valid_mul_instruction(instruction) {
                instructions.push(instruction.to_string());
            }
            start = end_idx;
        } else {
            break;
        }
    }
    return instructions;
}

fn calculate_sum_of_mul_instructions(instructions: Vec<String>) -> i32 {
    let mut total_sum = 0;
    for instruction in instructions {
        let index = find_mul(&instruction);
        let inner = &instruction[index..instruction.len() - 1];
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() == 2 {
            if let (Ok(a), Ok(b)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                total_sum += a * b;
            }
        }
    }
    return total_sum;
}

fn main() {
    let filepath: &str = "./../input.txt";
    let file_output: String = read_file_to_string(filepath);
    let instructions: Vec<String> = extract_mul_instructions(&file_output);
    let result: i32 = calculate_sum_of_mul_instructions(instructions);
    println!("Result: {}", result);
}
