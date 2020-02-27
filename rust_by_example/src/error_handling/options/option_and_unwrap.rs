pub fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck! I'm putting this snake back in the forest!"),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No gift? Oh well. "),
    }
}

pub fn give_princesss(gift: Option<&str>) {
    // unwrap returns a panic
    let inside = gift.unwrap();
    if inside == "snake" {
        panic!("AAAaaaa!!!!");
    }

    println!("I love {}'s!!!", inside);
}
