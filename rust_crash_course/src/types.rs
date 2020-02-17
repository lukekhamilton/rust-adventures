pub fn run() {
    let x = 1;

    let y = 2.5;

    let z: i64 = 4545445454545;

    println!("Max i32: {}", std::i32::MAX);
    println!("Min i32: {}", std::i32::MIN);
    println!("Max i64: {}", std::i64::MAX);
    println!("Min i64: {}", std::i64::MIN);

    let is_active: bool = true;

    let is_greater: bool = 10 < 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
