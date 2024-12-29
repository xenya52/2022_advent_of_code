use std::fs::read_to_string;

fn read_file_to_vec(path: &str) -> Vec<Vec<char>> {
    let mut row_vec = vec![];

    //Read each line of a file
    for line in read_to_string(path).unwrap().lines() {
        let mut col_vec = Vec::new();

        //Split each line into characters
        for c in line.chars() {
            col_vec.push(c);
        }

        //Add each split line to the row vector
        row_vec.push(col_vec);
    }
    return row_vec;
}

fn find_guard(vec2D: &Vec<Vec<char>>) -> (usize, usize, usize) {
    for (i, row) in vec2D.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == '^' {
                return (j, i, 0);
            } else if *col == '>' {
                return (j, i, 1);
            } else if *col == 'v' {
                return (j, i, 2);
            } else if *col == '<' {
                return (j, i, 3);
            }
        }
    }
    panic!("Guard not found");
}

fn total_visited_distincts(mut vec2D: Vec<Vec<char>>) -> usize {
    // direction explanation x = index 0 / y = index 1
    let direction: Vec<(isize, isize)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let x_vec_len: isize = vec2D[0].len() as isize;
    let y_vec_len: isize = vec2D.len() as isize;

    let mut total_visited: usize = 1; // Start with 1 because the guard is already visited
    let mut is_running: bool = true;

    let (x, y, mut current_direction): (usize, usize, usize) = find_guard(&vec2D);
    let mut x_cur: isize = x.try_into().unwrap();
    let mut y_cur: isize = y.try_into().unwrap();

    while is_running {
        let x_upcoming: isize = x_cur + direction[current_direction].0;
        let y_upcoming: isize = y_cur + direction[current_direction].1;

        // If the upcoming x is out of bounds, stop
        if x_upcoming < 0 || x_upcoming >= x_vec_len {
            is_running = false;
            break;
        }

        // If the upcoming y is out of bounds, stop
        if y_upcoming < 0 || y_upcoming >= y_vec_len {
            is_running = false;
            break;
        }

        // If theres a block, rotate 90 degrees
        if vec2D[y_upcoming as usize][x_upcoming as usize] == '#' {
            current_direction = (current_direction + 1) % 4;
        } else {
            x_cur = x_upcoming;
            y_cur = y_upcoming;
        }

        // If is unvisited, add to total_visited places and mark as visited
        if vec2D[y_cur as usize][x_cur as usize] == '.' {
            total_visited += 1;
            vec2D[y_cur as usize][x_cur as usize] = 'X';
        }
    }
    return total_visited;
}

fn main() {
    let path: &str = "./../input.txt";
    let vec2D: Vec<Vec<char>> = read_file_to_vec(path);
    let result: usize = total_visited_distincts(vec2D);
    println!("{}", result)
}
