pub mod constants;
use constants::{FILEPATH};
use utils::read_file;
use std::collections::HashMap;
use std::error::Error;

fn main() {
    let contents = read_file(FILEPATH).expect("Failed to read file");
    let mut map = HashMap::new();

    parse_file(contents, &mut map);

    let total_calibration = total_calibration(map).expect("Failed to retieve calobration");
    println!("Total combinations: {}", total_calibration);
}

fn parse_file(contents: String, map: &mut HashMap<i64, Vec<i64>>) {
    for line in contents.lines() {
        if let Some((key, value)) = line.split_once(':') {
            if let Ok(key_number) = key.trim().parse::<i64>() {
                let numbers: Vec<i64> = value
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                map.insert(key_number, numbers);
            }
        }
    }

}

fn total_calibration(map: HashMap<i64, Vec<i64>>) -> Result<i64, Box<dyn Error>> {
    let mut calibration = 0;

    for (key, value) in map.iter() {
        let n = value.len();
        let mut found_combination = false;
        for bits in 0..(1 << (n - 1)) {
            let mut total = value[0];
            for i in 0..(n - 1) {
                if (bits & (1 << i)) != 0 {
                    total += value[i + 1];
                } else {
                    total *= value[i + 1];
                }
            }
            if total == *key {
                found_combination = true;
                break;
            }
        }
        if found_combination {
            calibration += key;
        }
    }

    Ok(calibration)
}