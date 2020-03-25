pub fn run() {
    let list = vec![
        String::from("Orange"),
        String::from("Apple"),
        String::from("Banana"),
        String::from("Grape"),
    ];

    let out = map_for_each(list, |it| -> usize { 1 });

    println!("{:?}", out);
}

fn map_for_each(list: Vec<String>, func: fn(&String) -> usize) -> Vec<usize> {
    let mut new_array: Vec<usize> = Vec::new();

    for it in list.iter() {
        println!("it: {:?}", it.len());
        new_array.push(func(it));
    }
    new_array
}
