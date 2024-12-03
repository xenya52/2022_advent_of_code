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

fn calculation(vector: [Vec<i32>; 2]) -> i32 {
    let mut count: i32 = 0;

    for item in vector[0].clone() {
        let amount: i32 = vector[1].iter().filter(|&&x| x == item).count() as i32;
        count += item * amount;
    }
    return count;
}

fn main() {
    let input_file: String = readfile("./../input.txt");
    let vectors: [Vec<i32>; 2] = rows_to_vectors(&input_file);
    let solution: i32 = calculation(vectors);
    println!("Solution: {}", solution);
}
