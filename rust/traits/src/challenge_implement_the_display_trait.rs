use std::fmt::Display;

struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

impl Display for Satellite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} travelling at {} miles per second",
            self.name, self.velocity
        )
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescrope"),
        velocity: 4.72,
    };

    println!("hubble is {}", hubble);
}
