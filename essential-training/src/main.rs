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

    let a = true | false;
    println!("true bitwise OR false is {}", a);
    let b = true || false;
    println!("true logical OR false is {}", b);

    // The | operator operates on integers at the bit level, while || operates on boolean values.
    let x = 0b1010; // This is binary literal for decimal 10
    let y = 0b1100; // This is binary literal for decimal 12
    // Bitwise OR
    let z = x | y;
    println!("Bitwise OR: {}", z); // Output: 14 (binary 1110)

    // Logical OR
    if x > 5 || y > 15 {
        println!("At least one operand is true");
    } else {
        println!("Both operands are false");
    }
}
