pub mod constants;
use constants::{FILEPATH};
use utils::read_file;
use std::error::Error;
use std::cmp::Ordering::{Less, Greater};



fn main() {
    println!("Welcome to Day 5!");

    let contents = read_file(FILEPATH).expect("Failed to read file");
    let mut rules = Vec::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();

    parse_lines(&contents, &mut rules, &mut updates).expect("Failed to parse lines");

    let page_number_total = correct_updates(&updates, &rules).expect("Failed to find correct updates");
    let incorrect_page_number_total = incorrect_updates(&updates, &rules).expect("Failed to find incorrect updates");

    println!("Part 1, the sum of the middle numbers is: {}", page_number_total);
    println!("Part 2, the sum of the middle numbers is: {}", incorrect_page_number_total);

}

fn parse_lines(contents: &str, rules: &mut Vec<(usize, usize)>, updates: &mut Vec<Vec<usize>>) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
    let mut parsing_rules: bool = true;

    for line in contents.lines() {
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                let rule_1: usize = parts[0].parse().unwrap();
                let rule_2: usize = parts[1].parse().unwrap();
                rules.push((rule_1, rule_2));
            }
        } else {
            let update: Vec<usize> = line.split(',').map(|s| s.parse().unwrap()).collect();
            updates.push(update.clone());
        }
    }
    Ok(updates.clone())

}


fn correct_updates(updates: &Vec<Vec<usize>>, rules: &Vec<(usize, usize)>) -> Result<usize, Box<dyn Error>>{
    let mut correct_updates = Vec::new();
    for update in updates {
        let mut is_correct = true;
        for i in 0..update.len() {
            for j in i + 1..update.len() {
                if rules.contains(&(update[j], update[i])) {
                    is_correct = false;
                    break;
                }
            }
            if !is_correct {
                break;
            }
        }
        if is_correct {
            correct_updates.push(update.clone());
        }
    }

    let mut middle_sum = 0;

    for update in &correct_updates {
        let middle_index = update.len() / 2;
        middle_sum += update[middle_index];
    }
    Ok(middle_sum)
}

fn incorrect_updates(updates: &Vec<Vec<usize>>, rules: &Vec<(usize, usize)>) -> Result<usize, Box<dyn Error>> {
    let mut incorrect_updates = Vec::new();

    for update in updates {
        if !is_correct_update(update, rules) {
            incorrect_updates.push(update.clone());
        }
    }

    for update in &mut incorrect_updates {
        update.sort_by(|a, b| {
            if rules.contains(&(*a, *b)) {
                Less
            } else {
                Greater
            }
        });
    }

    let middle_sum: usize = incorrect_updates.iter()
        .map(|update| update[update.len() / 2])
        .sum();

    Ok(middle_sum)
}

fn is_correct_update(update: &Vec<usize>, rules: &Vec<(usize, usize)>) -> bool {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            if rules.contains(&(update[j], update[i])) {
                return false;
            }
        }
    }
    true
}


