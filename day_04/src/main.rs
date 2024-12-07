use std::error::Error;
use std::fs;
pub mod constants;
use constants::{FILEPATH, WORD};

fn main() {
    println!("Welcome to Day 4!");

    let file_path = FILEPATH;
    let word = WORD;

    let contents = read_file(file_path).expect("Failed to read file");
    let grid: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();

    let word_count = search_word(&grid, word).expect("Failed to search for word");
    let xmas_count = count_xmas_patterns(grid).expect("Failed to count XMAS patterns");

    println!("Part 1, the word {} appears {} times", word, word_count);
    println!("Part 2, the XMAS pattern appears {} times", xmas_count);
    
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    println!("The file you have chosen is: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    Ok(contents)
}

fn search_word(grid: &[Vec<char>], word: &str) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;
    let directions = [
        (0, 1),
        (1, 0), 
        (1, 1),
        (1, -1), 
        (0, -1),
        (-1, 0),
        (-1, 1),
        (-1, -1)
    ];

    let word_len = word.len();
    let grid_height = grid.len();
    let grid_width = grid[0].len();

    for row in 0..grid_height {
        for col in 0..grid_width {
            if grid[row][col] == word.chars().next().unwrap() {
                for &(dx, dy) in &directions {
                    let mut k = 1;
                    while k < word_len {
                        let x = row as isize + k as isize * dx;
                        let y = col as isize + k as isize * dy;

                        if x < 0 || x >= grid_height as isize || y < 0 || y >= grid_width as isize {
                            break;
                        }

                        if grid[x as usize][y as usize] != word.chars().nth(k).unwrap() {
                            break;
                        }

                        k += 1;
                    }

                    if k == word_len {
                        count += 1;
                    }
                }
            }
        }
    }
    Ok(count)
}

fn count_xmas_patterns(grid: Vec<Vec<char>>) -> Result<usize, Box<dyn Error>> {
    let grid_height = grid.len();
    let grid_width = grid[0].len();
    let mut count = 0;

    fn is_valid_xmas(grid: &Vec<Vec<char>>, row: usize, col: usize, grid_height: usize, grid_width: usize) -> bool {
        if row + 2 >= grid_height || col == 0 || col + 1 >= grid_width {
            return false;
        }

        let top_left = grid[row][col];
        let center = grid[row + 1][col];
        let bottom_right = grid[row + 2][col + 1];
        let top_right = grid[row][col + 1];
        let bottom_left = grid[row + 2][col - 1];

        (top_left == 'M' && center == 'A' && bottom_right == 'S' && top_right == 'S' && bottom_left == 'M') ||
        (top_left == 'S' && center == 'A' && bottom_right == 'M' && top_right == 'M' && bottom_left == 'S')
    }

    for row in 0..grid_height {
        for col in 0..grid_width {
            if is_valid_xmas(&grid, row, col, grid_height, grid_width) {
                count += 1;
            }
        }
    }

    Ok(count)
}
