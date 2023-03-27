fn main() {
    let mut value = 0b1111_0101u8;
    println!("The value is: {}", value);
    println!("The binary representation: {:08b}", value);

    value = !value; // NOT
    println!("The value is: {}", value);
    println!("The binary representation: {:08b}", value);

    value = value & 0b1111_0111; // clear bit with AND
    println!("The binary representation: {:08b}", value);
    println!("bit 6 is {}", value & 0b0100_0000); // check bit with AND

    value = value |  0b0100_0000; // set bit with OR
    println!("The binary representation: {:08b}", value);

    value = value ^ 0b0101_0101; // XOR
    println!("The binary representation: {:08b}", value);

    value = value << 4;
    // println!("The value is: {}", value);
    println!("The binary representation: {:08b}", value);

    value = value >> 2;
    println!("The binary representation: {:08b}", value);
}
