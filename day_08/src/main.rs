pub mod constants;
use constants::FILEPATH;
use std::collections::{HashMap, HashSet};

use utils::create_buf_reader;
use std::error::Error;
use std::io::{BufRead};

#[derive(Debug)]
struct MatrixDimensions {
    rows: usize,
    cols: usize,
}

impl MatrixDimensions {
    fn new(matrix: &[Vec<char>]) -> Self {
        let rows = matrix.len();
        let cols = if rows > 0 { matrix[0].len() } else { 0 };

        Self { rows, cols }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to Day 8!");

    let contents = create_buf_reader(FILEPATH).expect("Failed to create BufReader");
    let matrix = create_matrix(contents).expect("Failed to create matrix");

    let locations = find_char_locations(&matrix);
    let dimensions = MatrixDimensions::new(&matrix);

    let antinodes = find_antinodes_for_locations(locations, dimensions.rows, dimensions.cols).expect("Failed to find the antinodes");
    println!("Part 1, found {} unique antinode locations", antinodes);

    Ok(())
}


fn create_matrix<R: BufRead>(contents: R) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line_result in contents.lines() {
        match line_result {
            Ok(line) => {
                let row: Vec<char> = line.chars().collect();
                matrix.push(row);
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(matrix)
}

fn find_char_locations(matrix: &[Vec<char>]) -> HashMap<char, Vec<(usize, usize)>> {
    let mut locations: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch != '.' {
                locations
                    .entry(ch)
                    .or_insert_with(Vec::new)
                    .push((row_idx, col_idx));
            }
        }
    }
    locations    
}


fn find_antinodes_for_locations(
    locations: HashMap<char, Vec<(usize, usize)>>,
    rows: usize,
    cols: usize,
) -> Result<usize, String> {
    let mut unique_antinodes = HashSet::new();

    for (_, antennas) in &locations {
        let n = antennas.len();
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    unique_antinodes.insert((antennas[i].0 as isize, antennas[i].1 as isize));
                    continue;
                }

                let (x1, y1) = antennas[i];
                let (x2, y2) = antennas[j];

                let antinodes = find_antinodes_for_pair((x1, y1), (x2, y2), rows, cols);

                for antinode in antinodes {
                    if antinode.0 >= 0 && antinode.1 >= 0 && antinode.0 < rows as isize && antinode.1 < cols as isize {
                        unique_antinodes.insert(antinode);
                    }
                }
            }
        }
    }

    Ok(unique_antinodes.len())
}

fn find_antinodes_for_pair(
    x: (usize, usize),
    y: (usize, usize),
    rows: usize,
    cols: usize,
) -> Vec<(isize, isize)> {
    let midpoint = ((x.0 + y.0) / 2, (x.1 + y.1) / 2);

    let dx = y.0 as isize - x.0 as isize;
    let dy = y.1 as isize - x.1 as isize;
    

    let length_squared = dx * dx + dy * dy;

    if rows - cols != 0 {
        println!("Return")

    }
    let scale_squared = (length_squared as isize) / (rows as isize);


    let compute_antinode = |factor: isize| -> (isize, isize) {
        let scale = factor * scale_squared;
        let scaled_dx = scale * dx / length_squared;
        let scaled_dy = scale * dy / length_squared;
        (
            midpoint.0 as isize + scaled_dx,
            midpoint.1 as isize + scaled_dy,
        )
    };

    let antinodes = [compute_antinode(1), compute_antinode(-1)];

    antinodes.to_vec()
}