struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32, // miles
}

// Since we can't directly print structs using println!("{}", struct_name) we can implement a trait to print out a nicely formatted
// description for it
trait Description {
    fn describe(&self) -> String;
}

impl Description for Satellite {
    fn describe(&self) -> String {
        format!(
            "the {} flying at {} miles per second!",
            self.name, self.velocity
        );
    }
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!(
            "the {} flying {} miles high with {} crew members aboard!",
            self.name, self.altitude, self.crew_size
        );
    }
}

fn main() {
    // Rust provides an abstract way to define the capabilities or functionality of specific data types using traits,
    // which are collections of methods representing a set of behaviors necessary to accomplish some task. When a data
    // type implements a trait it implements those specific methods so it can use them. We've already used traits
    // with generics to specify the capabilities of unknown data types. Traits are similar to interfaces in other languages

    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };

    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254,
    };

    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());
}
