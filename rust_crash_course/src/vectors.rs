use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    let numbers1 = vec![4, 3, 2, 1];
    let numbers2 = &numbers1;

    println!("Numbers: {:?}", numbers);
    println!("Numbers1: {:?}", numbers1);
    println!("Numbers2: {:?}", numbers2);

    // Re-assign a value
    numbers[2] = 20;

    numbers.push(5);
    numbers.push(6);

    // Pop
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val
    println!("{}", numbers[0]);

    // Get length
    println!("Length: {}", numbers.len());

    // Are stack allocated
    println!("{} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[1..4];
    println!("Slice: {:?}", slice);

    // loop
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop and mut values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", numbers)
}
