fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

pub fn run() {
    // Fixed-sized array
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    //    let xs2 = [1, 2, 3, 4, 5];

    // All elements can be initialised to the same value
    //    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // 'len' returns the size of the array
    println!("array size: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", std::mem::size_of_val(&xs));

    // Arrays ca be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // out of bound indexing
    //    println!("{}", xs[5]);
}
