use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Paths to the different pyramids
    let paths = ["pyramid_3.txt", "pyramid_30.txt", "pyramid_300.txt"];

    // Count the number of blocks in each pyramid
    for path in paths {
        // File hosts must exist in current path before this produces output
        // Using the "if let" operator
        if let Ok(lines) = read_lines(path) {
            let mut count = 0;
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(level) = line {
                    count += count_blocks(&level);
                }
            }
            println!("{} contains {} Blocks", path, count)
        }
    }
}

// Count the number of blocks ('#' character) in a string
fn count_blocks(text: &str) -> u32 {
    let mut count = 0;
    for c in text.chars() {
        if c == '#' {
            count += 1;
        }
    }
    count
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
