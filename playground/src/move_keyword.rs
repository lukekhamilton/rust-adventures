pub fn move_keyword() {
    let mut capture = "hello";
    let closure = || {
        println!("rust sags {}", capture);
    };

    closure();
}
