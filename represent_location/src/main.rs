use std::fmt;

enum Location {
    Unknown,
    Anonymous,
    Known(f32, f32),
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Location::Unknown => write!(f, "Unknown location"),
            Location::Anonymous => write!(f, "Anonymous location"),
            Location::Known(x, y) => write!(f, "Location at {}, {}", x, y),
        }
    }
}

impl Location {
    fn display(&self) {
        println!("{}", &self);
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();
}
