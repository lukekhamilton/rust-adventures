pub fn run() {
    greeting("Hello", "Jan");

    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // closure
    let n3 = 10;
    let add_sum = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum:{}", add_sum(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name)
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
