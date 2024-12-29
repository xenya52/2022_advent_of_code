use std::fs;

fn file_to_tuple(path: &str) -> Vec<(usize, Vec<usize>)> {
    let file_content = fs::read_to_string(path).expect("Error reading file");
    let mut solution: Vec<(usize, Vec<usize>)> = Vec::new();

    for line in file_content.lines() {
        let splitted: Vec<&str> = line.split(":").collect::<Vec<_>>();

        let solution_number: usize = splitted[0].parse().unwrap();
        let solution_calc_values: Vec<usize> = splitted[1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        solution.push((solution_number, solution_calc_values));
    }

    return solution;
}

fn is_tuple_valid(tuple: &(usize, Vec<usize>)) -> bool {
    let mut queue: std::collections::VecDeque<(usize, usize)> = std::collections::VecDeque::new();
    let mut results: Vec<usize> = Vec::new();
    let nums = &tuple.1;

    // Start with the first element
    queue.push_back((nums[0], 1));

    while let Some((current_value, index)) = queue.pop_front() {
        if index == nums.len() {
            results.push(current_value);
        } else {
            // Add the next element
            queue.push_back((current_value + nums[index], index + 1));
            // Multiply the next element
            queue.push_back((current_value * nums[index], index + 1));
        }
    }

    for que in results {
        if que == tuple.0 {
            return true;
        }
    }
    return false;
}

fn calc_sum_valid_sums(tuples: Vec<(usize, Vec<usize>)>) -> usize {
    let mut sum: usize = 0;
    for tuple in tuples {
        if is_tuple_valid(&tuple) {
            sum += tuple.0;
        }
    }
    return sum;
}

fn main() {
    let path = "./../input.txt";
    let tuples = file_to_tuple(path);
    let result = calc_sum_valid_sums(tuples);
    println!("The sum of the valid sums is: {}", result);
}
