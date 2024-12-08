use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;

pub fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {

    println!("The file you have chosen is: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    Ok(contents)
}

pub fn read_file_to_string(file_path: &str) -> Result<String, Box<dyn Error>> {

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    Ok(contents)
}

pub fn create_buf_reader(file_path: &str) -> Result<BufReader<File>, Box<dyn Error>> {
    println!("The file you have chosen is: {}", file_path);

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(reader)
}