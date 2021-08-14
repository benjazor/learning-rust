use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let pyramid = create_pyramid(3);
    let path = Path::new("pyramid.txt");
    write_file(&pyramid, &path)
}

fn create_pyramid(x: usize) -> String {
    let mut pyramid = String::from("");
    let mut level = 0;
    let mut n = 1;

    while level < x {
        pyramid.push_str(" ".repeat((1 + (x - 1) * 2 - n) / 2).as_str());
        pyramid.push_str("#".repeat(n).as_str());
        pyramid.push('\n');
        n += 2;
        level += 1;
    }

    pyramid
}

fn write_file(text: &String, path: &Path) {
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the pyramid string to `file`, returns `io::Result<()>`
    match file.write(text.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
