fn main() {
    let mut count = 0;
    let letters = ["a", "b", "c"];

    while count < letters.len() {
        println!("Count is {}", letters[count]);
        count += 1;
    }
}
