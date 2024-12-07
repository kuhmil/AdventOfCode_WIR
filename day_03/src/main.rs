pub mod constants;
use constants::{DONT_REGEX, DO_REGEX, FILEPATH, MUL_REGEX, REGEX_PART_1, REGEX_PART_2};
use std::fs;
use regex::Regex;
use std::error::Error;


fn main() {
    println!("Welcome to Day 3!");

    let file_path = FILEPATH;

    let contents = read_file(file_path).expect("Failed to read file");
    let total = multiply_total(&contents, REGEX_PART_1).expect("Failed to multiply total");
    let multiply_total = parse_and_sum_memory(&contents).expect("Failed to multiply total");

    println!("Part 1, the total is: {}", total);
    println!("Part 2, the total is: {:?}", &multiply_total);

}

fn multiply_total(contents: &str, regex_string: &str) -> Result<i32, Box<dyn Error>> {
    let re = create_regex(regex_string);

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

fn parse_and_sum_memory(contents: &str) -> Result<i64, Box<dyn Error>> {
    let mut enabled = true;
    let mut total_sum = 0;

    let mul_regex = create_regex(MUL_REGEX);
    let do_regex = create_regex(DO_REGEX);
    let dont_regex = create_regex(DONT_REGEX);

    for cap in Regex::new(REGEX_PART_2)
        .unwrap()
        .find_iter(contents)
    {
        let instruction = cap.as_str();

        if let Some(captures) = mul_regex.captures(instruction) {
            if enabled {
                let x: i64 = captures[1].parse().unwrap();
                let y: i64 = captures[2].parse().unwrap();
                total_sum += x * y;
            }
        } else if do_regex.is_match(instruction) {
            enabled = true;
        } else if dont_regex.is_match(instruction) {
            enabled = false;
        }
    }
    Ok(total_sum)
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {

    println!("The file you have chosen is: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    Ok(contents)
}


fn create_regex(regex_string: &str) -> Regex {
    Regex::new(regex_string).unwrap()
}

