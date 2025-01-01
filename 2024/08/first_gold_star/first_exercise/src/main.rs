use std::collections::HashSet;

fn read_input_to_vec(path: &str) -> Vec<Vec<char>> {
    let file_content = std::fs::read_to_string(path).expect("Error reading file");
    let mut vec_2d: Vec<Vec<char>> = Vec::new();

    for line in file_content.lines() {
        let vec: Vec<char> = line.chars().collect();
        vec_2d.push(vec);
    }

    return vec_2d;
}

fn get_char_coordinates_from_2d_vec(
    vec_2d: &Vec<Vec<char>>,
    find_char: char,
) -> Vec<(usize, usize)> {
    let mut antenna_coordinates: Vec<(usize, usize)> = Vec::new();
    let x_len: usize = vec_2d[0].len();
    let y_len: usize = vec_2d.len();

    for y in 0..y_len {
        for x in 0..x_len {
            if vec_2d[y][x] == find_char {
                antenna_coordinates.push((x, y));
            }
        }
    }

    return antenna_coordinates;
}

fn get_all_important_char_coordinates(vec_2d: &Vec<Vec<char>>) -> Vec<Vec<(usize, usize)>> {
    let mut all_important_char_coordinates: Vec<Vec<(usize, usize)>> = Vec::new();

    // 48 -> 57 is equal to '0' -> '9'
    for numerics in 48..=57 {
        let c: char = char::from(numerics);
        let coordinates = get_char_coordinates_from_2d_vec(vec_2d, c);
        if !coordinates.is_empty() {
            all_important_char_coordinates.push(coordinates);
        }
    }

    // 65 -> 90 is equal to 'A' -> 'Z'
    for upper_case in 65..=90 {
        let c: char = char::from(upper_case);
        let coordinates = get_char_coordinates_from_2d_vec(vec_2d, c);
        if !coordinates.is_empty() {
            all_important_char_coordinates.push(coordinates);
        }
    }

    // 97 -> 122 is equal to 'a' -> 'z'
    for lower_case in 97..=122 {
        let c: char = char::from(lower_case);
        let coordinates = get_char_coordinates_from_2d_vec(vec_2d, c);
        if !coordinates.is_empty() {
            all_important_char_coordinates.push(coordinates);
        }
    }

    return all_important_char_coordinates;
}

fn calculate_antinode_position(
    antenna_xy: (usize, usize),
    gap_xy: (isize, isize),
    direction: (isize, isize),
) -> (isize, isize) {
    (
        antenna_xy.0 as isize + gap_xy.0 * direction.0,
        antenna_xy.1 as isize + gap_xy.1 * direction.1,
    )
}

fn calculate_two_antinodes_position_between_two_antennas(
    antenna_one_xy: (usize, usize),
    antenna_two_xy: (usize, usize),
) -> Vec<(isize, isize)> {
    // Calculate the gap between the two antennas
    let x_gap: isize = (antenna_one_xy.0 as isize - antenna_two_xy.0 as isize).abs();
    let y_gap: isize = (antenna_one_xy.1 as isize - antenna_two_xy.1 as isize).abs();

    // Calculate the two antinode positions
    let node_one_xy: (isize, isize) =
        calculate_antinode_position(antenna_one_xy, (x_gap, y_gap), (1, 1));
    let node_two_xy: (isize, isize) =
        calculate_antinode_position(antenna_one_xy, (x_gap, y_gap), (-1, -1));
    vec![node_one_xy, node_two_xy]
}

fn is_antinode_possible(antinode: (isize, isize), vec_2d: &Vec<Vec<char>>) -> bool {
    antinode.0 >= 0
        && antinode.0 < vec_2d[0].len() as isize
        && antinode.1 >= 0
        && antinode.1 < vec_2d.len() as isize
        && vec_2d[antinode.1 as usize][antinode.0 as usize] == '.'
}

fn sum_of_all_antinodes(vec_2d: &Vec<Vec<char>>) -> usize {
    let all_important_char_coordinates: Vec<Vec<(usize, usize)>> =
        get_all_important_char_coordinates(vec_2d);

    let mut antinode_positions: HashSet<(isize, isize)> = HashSet::new();

    for char_coordinates in all_important_char_coordinates {
        for i in 0..char_coordinates.len() {
            for j in i + 1..char_coordinates.len() {
                let antinode_positions_between_two =
                    calculate_two_antinodes_position_between_two_antennas(
                        char_coordinates[i],
                        char_coordinates[j],
                    );

                for antinode in antinode_positions_between_two {
                    if is_antinode_possible(antinode, vec_2d) {
                        antinode_positions.insert(antinode);
                    }
                }
            }
        }
    }

    return antinode_positions.len();
}

// Debugging
fn place_antinodes_in_grid(vec_2d: &mut Vec<Vec<char>>) {
    let all_important_char_coordinates: Vec<Vec<(usize, usize)>> =
        get_all_important_char_coordinates(vec_2d);

    let mut antinode_positions: HashSet<(isize, isize)> = HashSet::new();

    for char_coordinates in all_important_char_coordinates {
        for i in 0..char_coordinates.len() {
            for j in i + 1..char_coordinates.len() {
                let antinode_positions_between_two =
                    calculate_two_antinodes_position_between_two_antennas(
                        char_coordinates[i],
                        char_coordinates[j],
                    );

                for antinode in antinode_positions_between_two {
                    if is_antinode_possible(antinode, vec_2d) {
                        antinode_positions.insert(antinode);
                    }
                }
            }
        }
    }

    for &(x, y) in &antinode_positions {
        vec_2d[y as usize][x as usize] = '#';
    }
}

fn main() {
    let path: &str = "./../input.txt";
    let mut vec_2d: Vec<Vec<char>> = read_input_to_vec(path);
    for line in &vec_2d {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
    place_antinodes_in_grid(&mut vec_2d);
    for line in vec_2d {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}
