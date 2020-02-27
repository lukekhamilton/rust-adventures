use std::num::ParseIntError;
use std::str::ParseBoolError;

fn multiple(first_number_str: &str, second_number_str: &str) -> i32 {
    // Lets try using unwrap() to get the number out. Will it bite us??
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

pub fn run_v1() {
    let twenty = multiple("10", "2");
    println!("double is {}", twenty);

    // let tt = multiple("t", "2"); // Panic's !
    // println!("double is {}", tt);
}

pub fn run_v2() -> Result<(i32), ParseIntError> {
    let number_str = "10a";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };

    println!("{}", number);
    Ok((number))
}

pub fn run() {
    let res = run_v2();

    println!("{:?}", res);

    match res {
        Ok()
    }
}
