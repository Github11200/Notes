fn main() {
    let mut value = 0b1111_0101u8;
    println!("value is {}", value);

    // The colon denotes we want to use special formatting
    // The 0 means we want to print out zeroes before the number to fill up space instead of just spaces
    // The 8 means we want to print out 8 bits
    // The b means we want to display the values as binary bits
    println!("value is {:08b}", value);

    /*
        Bitwise operators do logical operations on patterns of bits at the individual bit level, it uses the ones listed below:
            -> NOT
            -> AND
            -> OR
            -> XOR
            -> SHIFT
    */

    // We can use the NOT bitwise operator as shown below
    value = !value;
    println!("value is {:08b}", value);

    // The bitwise AND operator is done using the & symbol
    value = value & 0b1111_0111;
    println!("value is {:08b}", value);
    println!("bit 6 is {}", value & 0b0100_0000); // We can also use logical AND to check what the bit's value is

    // The bitwise OR operator is done using the | symbol
    value = value | 0b0100_0000;
    println!("value is {:08b}", value);

    // The bitwise XOR (this stands for exclusive or) operator is done using the ^ symbol
    // The output of XOR is true when the two input bits are different and false when the input bits are the same (this is like an exclusive OR operator)
    value = value ^ 0b0101_0101;
    println!("value is {:08b}", value); // This shows what values differed from the alternating sequence

    // Bitwise shift operators can shift a bit pattern left or right by a certain number of bits; it will also fill empty spaces with 0 which may be left behind when shifting bits
    value = value << 4; // You use << to shift left, and then the number of bits you want to shift by
    println!("value is {:08b}", value);

    value = value >> 2; // You can use >> for right shift
    println!("value is {:08b}", value);
}
