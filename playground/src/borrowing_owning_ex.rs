pub fn run() {
    let mut v = Vec::new();
    println!("V: {:?}", v);
    v.push(1);
    v.push(2);
    println!("V: {:?}", v);

    borrow(&mut v);
    v.push(4);
    println!("V: {:?}", v);

    shared_borrow(&v);

    take(v);
    //    println!("V: {:?}", v); // This will not work as take destroys the data on fn exit.
}

// This will borrow a reference and have mutation access and then return it back.
fn borrow(x: &mut Vec<i32>) {
    println!("X: {:?}", x);
    x.push(3);
    println!("X: {:?}", x);
}

// Shared borrow allows for lots of aliasing / reading of data just not mutations.
fn shared_borrow(z: &Vec<i32>) {
    println!("Z: {:?}", z);
}

// This will take and destroy
fn take(y: Vec<i32>) {
    println!("Y: {:?}", y);
}
