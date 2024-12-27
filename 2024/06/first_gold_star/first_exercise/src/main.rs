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

fn total_visited_distincts(vec2D: Vec<Vec<char>>) -> usize {
    direction = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    current_direction = 0;
    is_running: bool = true;
    total_visited: usize = 0;
    x_cur = 0;
    y_cur = 0;
    x_vec_len = vec2D[0].len();
    y_vec_len = vec2D.len();

    while is_running {
        x_upcoming = x_cur + direction[current_direction].0;
        y_upcoming = y_cur + direction[current_direction].1;

        // If the upcoming x is out of bounds, stop
        if x_upcoming < 0 || x_upcoming >= x_vec_len {
            is_running = false;
            break;
        }

        // If theres a block, rotate 90 degrees and move forward
        if vec2D[y_upcoming][x_upcoming] == "#" {
            current_direction = (current_direction + 1) % 4;
            x_cur += direction[current_direction].0;
            y_cur += direction[current_direction].1;
        } else {
            x_cur = x_upcoming;
            y_cur = y_upcoming;
        }
        total_visited += 1;

    }
    return total_visited;
}

fn main() {
    let path: &str = "./../input.txt";
    let vec2D: Vec<Vec<char>> = read_file_to_vec(path);
    for row in vec2D {
        for col in row {
            print!("{}", col);
        }
        println!("");
    }
}
