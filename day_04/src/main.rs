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
    let total = search_word(&grid, word).expect("Failed to search word");

    println!("Part 1, the total is: {}", total);
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

    for i in 0..grid_height {
        for j in 0..grid_width {
            if grid[i][j] == word.chars().next().unwrap() {
                for &(dx, dy) in &directions {
                    let mut k = 1;
                    while k < word_len {
                        let x = i as isize + k as isize * dx;
                        let y = j as isize + k as isize * dy;

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

