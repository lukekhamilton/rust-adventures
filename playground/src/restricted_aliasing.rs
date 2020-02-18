pub fn run() {
    let mut v = Vec::new();

    v.push("Hello");

    println!("V: {:?}", v);

    let x = v[0];
    println!("X: {:?}", x);

    v.push("world!");

    println!("V: {:?}", v);

    let mut y = v;
    y.push("test");

    println!("Y: {:?}", y);

    println!("X: {:?}", x);
}
