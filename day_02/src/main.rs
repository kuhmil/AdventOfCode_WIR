pub mod constants;
use constants::FILEPATH;
use std::error::Error;
use std::fs;
use std::collections::HashMap;


fn main(){
    println!("Welcome to Day 2!");

    let file_path = FILEPATH;

    let contents = read_file(file_path).expect("Failed to read file");

    let mut number_map: HashMap<usize, Vec<i32>> = HashMap::new();
    let mut report: HashMap<usize, &str> = HashMap::new();
    let mut handling_report: HashMap<usize, &str> = HashMap::new();

    for (index, line) in contents.lines().enumerate() {
        let numbers: Vec<i32> = line
            .split_whitespace() 
            .filter_map(|num| num.parse::<i32>().ok()) 
            .collect();

        number_map.insert(index + 1, numbers);
    }

    for (index, line) in contents.lines().enumerate() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();
    
        let is_safe = is_safe_report(&numbers).unwrap_or(false);

        report.insert(index + 1, if is_safe { "safe" } else { "unsafe" });

    
        if !is_safe {
            let mut found_safe = false;
            for i in 0..numbers.len() {
                let mut modified_numbers = numbers.clone();
                modified_numbers.remove(i);
                if is_safe_report(&modified_numbers).unwrap_or(false) {
                    found_safe = true;
                    break;
                }
            }
            handling_report.insert(index + 1, if found_safe { "safe" } else { "unsafe" });
        } else {
            handling_report.insert(index + 1, "safe");
        }
    }
    let safe_count = report.values().filter(|&&status| status == "safe").count();
    let handling_report = handling_report.values().filter(|&&status| status == "safe").count();
    println!("Part 1, number of safe entries: {}", safe_count);
    println!("Part 2, number of safe entries after handling: {}", handling_report);

}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {

    println!("The file you have chosen is: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    Ok(contents)
}

fn is_safe_report(nums: &[i32]) -> Result<bool, Box<dyn Error>> {

    if nums.len() < 2 {
        return Ok(false);
    }

    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..nums.len() {
        let diff = (nums[i] - nums[i - 1]).abs();
        if diff < 1 || diff > 3 {
            return Ok(false);
        }
        if nums[i] > nums[i - 1] {
            decreasing = false;
        } else if nums[i] < nums[i - 1] {
            increasing = false;
        }
    }

    Ok(increasing || decreasing)

}