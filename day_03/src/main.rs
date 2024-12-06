pub mod constants;
use constants::{FILEPATH, REGEX_PART_1};
use std::error::Error;
use std::fs;
use regex::Regex;

fn main() {
    println!("Welcome to Day 3!");

    let file_path = FILEPATH;

    let contents = read_file(file_path).expect("Failed to read file");
    let total = multiply_total(contents.clone(), REGEX_PART_1).expect("Failed to multiply total");


    println!("Part 1, the total is: {}", total);

}

fn multiply_total(contents: String, regex_string: &str) -> Result<i32, Box<dyn Error>> {
    let re = Regex::new(regex_string).unwrap();

    let mut total = 0;

    for cap in re.captures_iter(&contents) {
        let caps = re.captures(&cap[0]).unwrap();
        let nums: Vec<&str> = caps[0][4..caps[0].len()-1].split(',').collect();
        let num1: i32 = nums[0].parse().unwrap();
        let num2: i32 = nums[1].parse().unwrap();
        let result = num1 * num2;
        total += result;
    }

    Ok(total)
}


fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {

    println!("The file you have chosen is: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    Ok(contents)
}

