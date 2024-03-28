struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

// To define methods within the context of a struct we have ot put them in an implementation block
impl Shuttle {
    // This method is meant to retreive the name of the vehicle
    fn get_name(&self) -> &str {
        // We can simply return a slice of the name
        &self.name
    }

    // We have to use &mut self as a mutable reference so that we can modify this struct's data
    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }
}

fn main() {
    // We can also add methods to structs which can have input parameters and return values, an import thing
    // here is that the first parameter is a reference to the struct instance (like the this keyword)

    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 0.0,
    };

    let vehicle_name = vehicle.get_name();
    println!("vehicle_name is {}", vehicle_name);

    println!("propellant is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant is {}", vehicle.propellant);
}
