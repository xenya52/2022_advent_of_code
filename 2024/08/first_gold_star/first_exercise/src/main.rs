
//Returns a file with the content as a vector of vectors of characters ... yea hihi
fn read_input_to_vec(path: &str) -> Vec<Vec<char>> {
    let file_content = std::fs::read_to_string(path).expect("Error reading file");
    let mut vec2D: Vec<Vec<char>> = Vec::new();

    for line in file_content.lines() {
        let vec: Vec<char> = line.chars().collect();
        vec2D.push(vec);
    }

    return vec2D;
}

// Returns tuple with (x, y) of each antenna
fn get_char_cooridnates_from_2D_vec(
    vec_2D: Vec<Vec<char>>,
    find_char: char,
) -> Vec<(usize, usize)> {
    let mut antenna_coordinates: Vec<(usize, usize)> = Vec::new();
    let x_len: usize = vec_2D[0].len();
    let y_len: usize = vec_2D.len();

    for y in 0..y_len {
        for x in 0..x_len {
            if vec_2D[y][x] == find_char {
                antenna_coordinates.push((x, y));
            }
        }
    }

    return antenna_pairs;
}

// Return all important charachter coordinates from the 2D vector
fn get_all_important_char_coordinates(vec_2D: Vec<Vec<char>>) -> Vec<Vec<(usize, usize)>> {
    let mut all_important_char_coordinates: Vec<Vec<(usize, usize)>> = Vec::new();

    // 48 -> 57 is equal to '0' -> '9'
    for numerics in 48..=57 {
        let c: char = char::from(numerics);
        let coordinates = get_char_cooridnates_from_2D_vec(vec_2D.clone(), c);
        all_important_char_coordinates.push(coordinates);
    }

    // 65 -> 90 is equal to 'A' -> 'Z'
    for upper_case in 65..=90 {
        let c: char = char::from(upper_case);
        let coordinates = get_char_cooridnates_from_2D_vec(vec_2D.clone(), c);
        all_important_char_coordinates.push(coordinates);
    }

    // 97 -> 122 is equal to 'a' -> 'z'
    for lower_case in 97..=122 {
        let c: char = char::from(lower_case);
        let coordinates = get_char_cooridnates_from_2D_vec(vec_2D.clone(), c);
        all_important_char_coordinates.push(coordinates);
    }

    return all_important_char_coordinates;
}

fn main() {
    let path: &str = "./../input.txt";
    let vec_2D: Vec<Vec<char>> = read_input_to_vec(path);

    let all_important_char_coordinates: Vec<Vec<(usize, usize)> = get_all_important_char_coordinates(vec_2D);
    for coordinates in all_important_char_coordinates {
        println!("{:?}", coordinates);
    }
    // Important ascii areas for this exercise
    // 48 -> 57 is equal to '0' -> '9'
    // 65 -> 90 is equal to 'A' -> 'Z'
    // 97 -> 122 is equal to 'a' -> 'z'
}
