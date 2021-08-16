use std::env;
use std::fs;

/*
    Tell if someone is a member or add it to the members list if he is not
*/
fn main() {
    // Check if at least two arguments were passed
    if env::args().len() < 3 {
        eprintln!("This program requires at least 2 arguments:\n1. path of the members list file\n2. name of the member");
        std::process::exit(1);
    }

    // Store arguments values into variables
    let file_path = env::args().nth(1).unwrap();
    let member = env::args().nth(2).unwrap();

    // Read the file
    let mut content = fs::read_to_string(&file_path).unwrap();

    // Search for the member
    for line in content.lines() {
        if line.eq(&member) {
            println!("{} is a member!", &member);
            return;
        }
    }

    // Add the member to the end of the file
    println!(
        "{} is not a member yet!\nUpdating the members list...",
        &member
    );

    content.push('\n');
    content.push_str(&member);

    // Save the new members list
    fs::write(file_path, content);
    println!("done!")
}
