fn main() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    // You can replace 3 as f64 with just 3.0
    let average = (a as f64 + b as f64 + c as f64) / 3 as f64;

    assert_eq!(average, 45.1);
    println!("Test passed!");
}
