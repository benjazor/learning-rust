use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Paths to the different pyramids
    let paths = [
        Path::new("pyramid_3.txt"),
        Path::new("pyramid_30.txt"),
        Path::new("pyramid_300.txt"),
    ];

    // Count the number of blocks in each pyramid
    for path in paths {
        read_file(path);
    }
}

// Count the number of blocks ('#' character) in a string
fn count_blocks(pyramid: &str) -> u32 {
    let mut count = 0;
    for c in pyramid.chars() {
        if c == '#' {
            count += 1;
        }
    }
    count
}

// Read the content of a pyramid file and display the number of blocks it contains
fn read_file(path: &Path) {
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains {} Blocks\n", display, count_blocks(&s)),
    }
}
