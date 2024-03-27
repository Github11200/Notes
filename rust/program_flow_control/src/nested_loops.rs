fn main() {
    let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    // If we use iter_mut it will give mutable references
    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10; // We have to dereference num in order to add to it

            // Since we're using print instead of println it means there will not be an extra line at the end
            // The \t means it will insert a tab character after the number
            print!("{}\t", num);
        }
        println!();
    }
}
