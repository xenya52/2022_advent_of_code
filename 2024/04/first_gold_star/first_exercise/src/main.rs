fn read_file_as_string(path: &str) -> String {
    let file = std::fs::read_to_string(path).unwrap();
    return file;
}

fn string_to_a_2D_vec(text: String) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    for line in text.lines() {
        let temp: Vec<char> = line.chars().collect();
        result.push(temp);
    }
    return result;
}

fn xmas_finder_in_2_d_vec(v: Vec<Vec<char>>) -> i32 {
    let x_len: usize = v[0].len();
    let y_len: usize = v.len();
    let want_word: Vec<char> = vec!['X', 'M', 'A', 'S'];
    let mut count: i32 = 0;

    for i in 0..y_len {
        for j in 0..x_len {
            if v[i].len() != x_len {
                // In case of empty lines in the input file
                continue;
            }
            // Check horizontally
            if j + 3 < x_len {
                // Forward
                if v[i][j] == want_word[0]
                    && v[i][j + 1] == want_word[1]
                    && v[i][j + 2] == want_word[2]
                    && v[i][j + 3] == want_word[3]
                {
                    count += 1;
                }
                // Backward
                if v[i][j] == want_word[3]
                    && v[i][j + 1] == want_word[2]
                    && v[i][j + 2] == want_word[1]
                    && v[i][j + 3] == want_word[0]
                {
                    count += 1;
                }
            }

            // Check vertically
            if i + 3 < y_len {
                // Forward
                if v[i][j] == want_word[0]
                    && v[i + 1][j] == want_word[1]
                    && v[i + 2][j] == want_word[2]
                    && v[i + 3][j] == want_word[3]
                {
                    count += 1;
                }
                // Backward
                if v[i][j] == want_word[3]
                    && v[i + 1][j] == want_word[2]
                    && v[i + 2][j] == want_word[1]
                    && v[i + 3][j] == want_word[0]
                {
                    count += 1;
                }
            }

            // Check diagonally (top-left to bottom-right)
            if i + 3 < y_len && j + 3 < x_len {
                // Forward
                if v[i][j] == want_word[0]
                    && v[i + 1][j + 1] == want_word[1]
                    && v[i + 2][j + 2] == want_word[2]
                    && v[i + 3][j + 3] == want_word[3]
                {
                    count += 1;
                }
                // Backward
                if v[i][j] == want_word[3]
                    && v[i + 1][j + 1] == want_word[2]
                    && v[i + 2][j + 2] == want_word[1]
                    && v[i + 3][j + 3] == want_word[0]
                {
                    count += 1;
                }
            }

            // Check diagonally (bottom-left to top-right)
            if i >= 3 && j + 3 < x_len {
                // Forward
                if v[i][j] == want_word[0]
                    && v[i - 1][j + 1] == want_word[1]
                    && v[i - 2][j + 2] == want_word[2]
                    && v[i - 3][j + 3] == want_word[3]
                {
                    count += 1;
                }
                // Backward
                if v[i][j] == want_word[3]
                    && v[i - 1][j + 1] == want_word[2]
                    && v[i - 2][j + 2] == want_word[1]
                    && v[i - 3][j + 3] == want_word[0]
                {
                    count += 1;
                }
            }
        }
    }
    return count;
}

fn main() {
    let path: &str = "./../input.txt";
    let file: String = read_file_as_string(&path);
    let vector_2D: Vec<Vec<char>> = string_to_a_2D_vec(file);
    let result: i32 = xmas_finder_in_2_d_vec(vector_2D);
    println!("Result: {}", result);
}
