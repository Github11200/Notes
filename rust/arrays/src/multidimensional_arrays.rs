fn main() {
    // You can't have one sub array have 3 elements and the other have 4 because they are not the same data type
    let parking_lot = [[1, 2, 3], [4, 5, 6]];

    let number = parking_lot[0][1];
    println!("number is {}", number);

    let garage: [[[i32; 100]; 20]; 5];
    garage = [[[0; 100]; 20]; 5]; // This will give a garage full of zeroes

    println!("garage number is {}", garage[0][0][0]);
}
