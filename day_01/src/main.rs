pub mod constants;
use constants::FILEPATH;

use std::fs;
use std::error::Error;
use std::collections::HashMap;

fn main() {
    print!("Welcome to Day 1!");
    let file_path = FILEPATH;

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();


    let contents = read_file(file_path).expect("Failed to read file");

    for line in contents.lines() {
        
        let parts: Vec<&str> = line.split_whitespace().collect();

        left_list.push(parts[0].parse().expect("Invalid number in input file"));
        right_list.push(parts[1].parse().expect("Invalid number in input file"));
    }

    left_list.sort();
    right_list.sort();

    
    let calc_total = calculate_difference(&left_list, &right_list).expect("Failed to calculate difference");
    let siml_total = calculate_similarity_score(&left_list, &right_list).expect( "Failed to calculate similarity score");


    println!("Part 1 value is: {}", calc_total);
    println!("Part 2 value is: {}", siml_total);


}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {

    println!("The file you have chosen is: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    Ok(contents)
}


fn calculate_difference(left_list: &[i32], right_list: &[i32]) -> Result<i32, Box<dyn Error>> {

    if left_list.len() != right_list.len() {
        panic!("The lists are not of the same length!");
    }

    let mut difference_list: Vec<i32> = Vec::new();

    let list_length = left_list.len();

    for i in 0..list_length {
        let value = (left_list[i] - right_list[i]).abs();
        difference_list.push(value);
    }

    let total: i32 = difference_list.iter().sum();

    Ok(total)
}

fn calculate_similarity_score(left_list: &[i32], right_list: &[i32]) -> Result<i32, Box<dyn Error>> {
    let mut counts = HashMap::new();

    for &number in right_list {
        *counts.entry(number).or_insert(0) += 1;
    }

    let mut similarity_score = 0;

    for &number in left_list {
        if let Some(&count) = counts.get(&number) {
            similarity_score += number * count;
        }
    }

    Ok(similarity_score)
}