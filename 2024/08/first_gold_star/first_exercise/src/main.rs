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

    return antenna_coordinates;
}

// Return all important charachter coordinates from the 2D vector
fn get_all_important_char_coordinates(vec_2D: &Vec<Vec<char>>) -> Vec<Vec<(usize, usize)>> {
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

fn is_antinode_possible(coordinates_xy: (usize, usize), vec_xy: (usize, usize)) -> bool {
    if coordinates_xy.0 < vec_xy.0 || coordinates_xy.1 < vec_xy.1 {
        return true;
    } else {
        return false;
    }
}

fn calculate_antinode_position(
    antenna_xy: (usize, usize),
    gap_xy: (usize, usize),
    direction: (isize, isize),
) -> (usize, usize) {
    match direction {
        (0, 1) => (antenna_xy.0, antenna_xy.1 + gap_xy.1),
        (1, 0) => (antenna_xy.0 + gap_xy.0, antenna_xy.1),
        (0, -1) => (antenna_xy.0, antenna_xy.1 - gap_xy.1),
        (-1, 0) => (antenna_xy.0 - gap_xy.0, antenna_xy.1),
        _ => panic!("Direction not valid"),
    }
}

fn calculate_two_antinodes_position_between_two_antennas(
    antenna_one_xy: (usize, usize),
    antenna_two_xy: (usize, usize),
) -> Vec<(usize, usize)> {
    // Calculate the gap between the two antennas
    let x_gap: usize = if antenna_one_xy.0 > antenna_two_xy.0 {
        antenna_one_xy.0 - antenna_two_xy.0
    } else {
        antenna_two_xy.0 - antenna_one_xy.0
    };
    let y_gap: usize = if antenna_one_xy.1 > antenna_two_xy.1 {
        antenna_one_xy.1 - antenna_two_xy.1
    } else {
        antenna_two_xy.1 - antenna_one_xy.1
    };

    // Calculate the two antinode positions
    let node_one_xy: (usize, usize) =
        calculate_antinode_position(antenna_one_xy, (x_gap, y_gap), (1, 0));
    let node_two_xy: (usize, usize) =
        calculate_antinode_position(antenna_one_xy, (x_gap, y_gap), (0, 1));
    vec![node_one_xy, node_two_xy]
}

fn sum_of_all_antinodes(vec_2D: &Vec<Vec<char>>) -> usize {
    let all_important_char_coordinates: Vec<Vec<(usize, usize)>> =
        get_all_important_char_coordinates(vec_2D);
    /*
    let antinode_positions: Vec<(usize, usize)> =
        calculate_two_antinodes_position_between_two_antennas(
            all_important_char_coordinates[0][0],
            all_important_char_coordinates[1][0],
        );
    */

    let mut sum: usize = 0;

    println!("{:?}", all_important_char_coordinates);

    for char_coordinates in all_important_char_coordinates {
        for i in 1..char_coordinates.len() {
            // Skip empty char coordinates positions
            if char_coordinates[i - 1].0 == 0 && char_coordinates[i - 1].1 == 0 {
                continue;
            }

            // Calculate the antinode positions between two antennas
            let antinode_positions: Vec<(usize, usize)> =
                calculate_two_antinodes_position_between_two_antennas(
                    char_coordinates[i - 1],
                    char_coordinates[i],
                );

            // Check if the antinode positions are possible
            for antinode in antinode_positions {
                if is_antinode_possible(antinode, (vec_2D[0].len(), vec_2D.len())) {
                    sum += 1;
                }
            }
        }
    }

    return sum;
}

fn main() {
    let path: &str = "./../input.txt";
    let vec_2D: Vec<Vec<char>> = read_input_to_vec(path);
    let result: usize = sum_of_all_antinodes(&vec_2D);
    println!("{}", result);
    // Important ascii areas for this exercise
    // 48 -> 57 is equal to '0' -> '9'
    // 65 -> 90 is equal to 'A' -> 'Z'
    // 97 -> 122 is equal to 'a' -> 'z'
}
