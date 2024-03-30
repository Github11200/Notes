#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

fn main() {
    // In Rust, enums are commonly used for match operations to control the flow of the program, this
    // operator compares a value to a series of patterns to determine which code to execute

    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape is {:?}", my_shape);

    match my_shape {
        Shape::Circle(r) => println!("Circle with radius {}", r),
        Shape::Rectangle(w, h) => println!("{} X {} Rectangle", w, h),
        Shape::Triangle(a, b, c) => println!("Triangle with sides {}, {}, {}", a, b, c),
    }
}
