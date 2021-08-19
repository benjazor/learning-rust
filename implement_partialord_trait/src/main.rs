use std::cmp::Ordering;

struct Satellite {
    name: String,
    velocity: f64,
}

impl PartialOrd for Satellite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
    fn lt(&self, other: &Satellite) -> bool {
        &self.velocity < &other.velocity
    }
    fn le(&self, other: &Satellite) -> bool {
        &self.velocity <= &other.velocity
    }
    fn gt(&self, other: &Satellite) -> bool {
        &self.velocity > &other.velocity
    }
    fn ge(&self, other: &Satellite) -> bool {
        &self.velocity >= &other.velocity
    }
}

impl Ord for Satellite {
    fn cmp(&self, other: &Self) -> Ordering {
        self.velocity.partial_cmp(&other.velocity).unwrap()
    }
}

impl PartialEq for Satellite {
    fn eq(&self, other: &Self) -> bool {
        self.velocity == other.velocity
    }
}

impl Eq for Satellite {}

fn main() {
    let sat_1 = Satellite {
        name: String::from("Hubble"),
        velocity: 4034.65,
    };
    let sat_2 = Satellite {
        name: String::from("Discovery"),
        velocity: 9843034.98,
    };

    println!("{} == {} = {}", sat_1.name, sat_2.name, sat_1 == sat_2);
    println!("{} == {} = {}", sat_1.name, sat_1.name, sat_1 == sat_1);
    println!("{} <  {} = {}", sat_1.name, sat_2.name, sat_1 < sat_2);
    println!("{} <= {} = {}", sat_1.name, sat_2.name, sat_1 <= sat_2);
    println!("{} >  {} = {}", sat_1.name, sat_2.name, sat_1 > sat_2);
    println!("{} >= {} = {}", sat_1.name, sat_2.name, sat_1 >= sat_2);
}
