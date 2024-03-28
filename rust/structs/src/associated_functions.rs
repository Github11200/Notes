struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    // In order to create a new associated function to build a new shuttle we can look at the code below
    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0,
        }
    }
}

fn main() {
    // In addition to using implementation blocks to define methods, we can also use them to define functions that are associated with a struct data type.
    // These look similar to methods but an associated function does not have a &self parameter, instead these fuhnctions can provide sub-routines
    // that are related to the struct data type in general.

    // Now that we have an associated function to create a new Shuttle instead we can simply use Shuttle::new() instead of Shuttle {...}
    let mut vehicle = Shuttle::new("Endeavour");
    let mut vehicle2 = Shuttle::new("Discovery");

    let vehicle_name = vehicle.get_name();
    println!("vehicle_name is {}", vehicle_name);

    println!("propellant is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant is {}", vehicle.propellant);
}
