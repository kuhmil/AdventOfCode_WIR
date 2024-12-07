use std::error::Error;
use std::fs;

pub fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {

    println!("The file you have chosen is: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    Ok(contents)
}