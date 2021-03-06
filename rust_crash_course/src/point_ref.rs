pub fn run() {
    // Primitive Array
    let array1 = [1, 2, 3];
    let array2 = array1;

    println!("Values: {:?}", (array1, array2));

    // Vectors
    let mut vec1 = vec![1, 2, 3];
    let vec2 = &vec1; // Borrowing: Restricted Aliasing
    let vec3 = &vec1; // Borrowing: Restricted Aliasing

    println!("Values: {:?}", vec1);
    println!("Values: {:?}", vec2);
    println!("Values: {:?}", vec3);

    vec1[0] = 21;

    println!("Values: {:?}", vec1);
    println!("Values: {:?}", vec2);
    println!("Values: {:?}", vec3);

    //    vec2[0] = 21;
    //    println!("Values: {:?}", vec2);

    // TODO how to do mmutable borrowing
    //    vec1[0] = 21;
    //
    //    println!("Values: {:?}", vec1);
    //    println!("Values: {:?}", vec2);
}
