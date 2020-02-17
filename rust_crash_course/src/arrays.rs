use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assign a value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single val
    println!("{}", numbers[0]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array: {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[1..4];
    println!("Slice: {:?}", slice);
}
