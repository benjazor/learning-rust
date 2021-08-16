use std::env;
use std::fs;

fn main() {
    if env::args().len() < 3 {
        eprintln!("Program requires at least 2 arguments to run.");
        std::process::exit(1);
    }

    // Store arguments in variables
    let file_path = env::args().nth(1).unwrap();
    let name = env::args().nth(2).unwrap();

    // Read the file
    let content = fs::read_to_string(&file_path).unwrap();

    // Check if the file contains the name
    for line in content.lines() {
        if line.eq(&name) {
            println!("{} contains the name {}", file_path, name);
            return;
        }
    }
    println!("{} does not contain the name {}", file_path, name);
}
