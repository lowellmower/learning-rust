// Integers can be expressed using hexadecimal, octal or binary
// notation using either of these prefixes: 0x, 0o or 0b

fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // if one attempts or accidentally uses an unsigned integer on L9
    // the compiler would catch such a thing and give a helpful error
    // such as the like:
    // $:> rustc lits-and-ops.rs
    // error: attempt to subtract with overflow
    //  --> lits-and-ops.rs:9:28
    //   |
    // 9 |     println!("1 - 2 = {}", 1u32 - 2);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // NAND
    let a = 0b01u32;
    let b = 0b00u32;
    println!("1 AND NOT 1 is {:02b}", a & !a ); // 0
    println!("1 AND NOT 0 is {:02b}", a & !b ); // 1
    println!("0 AND NOT 1 is {:02b}", b & !a ); // 0
    println!("0 AND NOT 0 is {:02b}", b & !b ); // 0

    // Bitwise operations
    println!("0011 AND 0101 is {:06b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}