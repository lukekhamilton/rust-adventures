pub fn run() {
    let name = "Brad";
    let mut age = 37;

    println!("My name is {}, and I am {}", name, age);

    age = age + 1;

    println!("My name is {}, and I am now {}", name, age);

    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (my_name, my_age) = ("Brad", 39);
    println!("{} is {}", my_name, my_age);
}
