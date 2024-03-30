use std::collections::HashMap;

fn main() {
    // Hash maps store data in key value pairs, so you provide a key to get the corresponding value, and
    // this key value pairing only works one way

    // Keys and values can be different data types
    // All keys must have the same data type
    // All values must have the same data type
    // Each key can only have one value associated with it at the same time, so you can't have duplicate keys

    let mut missions_flown = HashMap::new(); // missions flown as of 1 Jan 2021
    missions_flown.insert("Hadfield", 3); // Chris Hadfield
    missions_flown.insert("Hurley", 3); // Doug Hurley
    missions_flown.insert("Barron", 0); // Kayla Barron
    missions_flown.insert("Barron", 1); // Overwrite entry
    missions_flown.entry("Stone").or_insert(2); // Insert a value of 2 if the entry does not exist

    let kayla = missions_flown.entry("Barron").or_insert(0);
    *kayla += 1; // We can dereference this entry and increment by one

    println!("missions_flown is {:?}", missions_flown);

    let barron_missions = missions_flown.get("Barron");
    println!("barron_missions is {:?}", barron_missions);
}
