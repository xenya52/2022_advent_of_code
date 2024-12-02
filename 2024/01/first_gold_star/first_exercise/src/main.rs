fn readfile(filename: &str) -> String {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

fn rows_to_vectors(list: &str) -> [Vec<i32>; 2] {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    for (index, x) in list.split_whitespace().enumerate() {
        let num: i32 = x.parse().unwrap();
        if index % 2 == 0 {
            v1.push(num);
        } else {
            v2.push(num);
        }
    }
    return [v1, v2];
}

fn find_min_in_vector(vector: &Vec<i32>) -> (usize, i32) {
    let mut min_index = 0;
    let mut min_value = vector[0];
    for (i, &value) in vector.iter().enumerate() {
        if value < min_value {
            min_index = i;
            min_value = value;
        }
    }
    (min_index, min_value)
}

fn calculation(mut vectors: [Vec<i32>; 2]) -> i32 {
    let mut sum: i32 = 0;
    let len = vectors[0].len().min(vectors[1].len());
    println!("{:?}", len);
    for _ in 0..len {
        let (min1_index, min1) = find_min_in_vector(&vectors[0]);
        let (min2_index, min2) = find_min_in_vector(&vectors[1]);
        sum += (min2 - min1).abs();
        vectors[0].remove(min1_index);
        vectors[1].remove(min2_index);
    }
    sum
}

fn main() {
    let input_file: String = readfile("./../input.txt");
    let vectors: [Vec<i32>; 2] = rows_to_vectors(&input_file);
    let solution: i32 = calculation(vectors);
    println!("Solution: {}", solution);
}
