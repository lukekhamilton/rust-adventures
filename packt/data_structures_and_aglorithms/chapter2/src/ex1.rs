use std::mem;

#[derive(Debug)]
struct MyStruct {
    a: u8,
    b: u8,
    c: u8,
}

pub fn assert_it() {
    assert_eq!(mem::size_of::<MyStruct>(), 3 * mem::size_of::<u8>());
    assert_eq!(mem::size_of::<[MyStruct; 2]>(), 3 * mem::size_of::<u8>() * 2);

    // let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let x : [MyStruct; 2]= [MyStruct {a:1, b:2, c:3}, MyStruct{a:3,b:2,c:1}];

    println!("{:#?}", x);
    println!("{:?}", mem::size_of::<MyStruct>());
    println!("{:?}", mem::size_of::<u8>())
}