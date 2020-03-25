use std::collections::HashSet;

pub fn run() {
    let mut books = HashSet::new();

    books.insert("A Dance With Dragons".to_string());
    books.insert("To Kill a Mockingbird".to_string());
    books.insert("The Odyssey".to_string());
    books.insert("The Great Gatsby".to_string());

    println!("books: {:?}", books);

    if !books.contains("The Winds of Winter") {
        println!(
            "We have {} books, but the Winds of Winter ain't one.",
            books.len()
        );
    }

    books.remove("The Odyssey");

    for book in &books {
        println!("{}", book);
    }

    let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [4, 2, 3, 4].iter().cloned().collect();

    println!("a: {:?}", a);
    println!("b: {:?}", b);

    // for x in a.union(&b) {
    //     println!("{}", x);
    // }

    let union: HashSet<_> = a.union(&b).collect();
    println!("union: {:?}", union);

    // for x in a.intersection(&b) {
    //     println!("{}", x)
    // }

    let intersection: HashSet<_> = a.intersection(&b).collect();
    println!("intersection: {:?}", intersection);

    let symmetric_difference: HashSet<_> = a.symmetric_difference(&b).collect();
    println!("symmetric_difference: {:?}", symmetric_difference);
}
