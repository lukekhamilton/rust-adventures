pub fn run() {
    let person: (&str, &str, i8) = ("Brad", "Mass", 37);

    println!("{} is from {} and is {}", person.0, person.1, person.2);

    let person2: (&str, &str, &str, i8) = ("Brad", "Mass", "here", 23);

    println!(
        "{} is from {} and was {} when he was {}",
        person2.0, person2.1, person2.2, person2.3
    );
}
