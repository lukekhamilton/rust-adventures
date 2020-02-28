pub fn run() {
    // Create variables to be borrowed below
    let (four, nine) = (4, 9);

    // Borrows ('&') of both variables are passed into the function.
    print_refs(&four, &nine);

    // Any input which is borrowed must outlive the borrower.
    // In other words, the lifetime of 'four' and 'nine' must
    // be longer than that of 'print_refs'
}

// `print_refs` takes two references to `i32` which have different lifetimes
// `a and `b. These two lifetimes must both be at least as long as the function `print_refs`
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x: {}, y: {}", x, y);
}

// A function which takes no arguments, but has a lifetime parameter `a
fn failed_borrow<'a>() {
    let _x = 12;

    // ERROR: _x does not live long enough
    //
}
