pub fn run() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer substation
    println!("1 - 2 = {}", 1i32 - 2);

    // Short-circuting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // bitwise operation
    println!("0011 AND 0101 is {:04b}", 0b0011 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
    let _x = 1_000_000u32;

    // 1e9
    println!("{:}", 1e9 as i64);
    println!("{:b}", 1e9 as i64);
    println!("{:x}", 1e9 as i64);
    println!("{:o}", 1e9 as i64);
}
