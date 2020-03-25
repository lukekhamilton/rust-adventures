pub fn run() {
    let numbers_iterator = [1, 2, 3].iter();

    let sum = numbers_iterator.fold(0, |total, next| total + *next);
    println!("sum: {:?}", sum);

    // let squared: i32 = numbers_iterator.map(|&x| x * x).collect();

    let squared: Vec<i32> = (1..10).map(|x| x).collect();

    println!("squared: {:?}", squared);

    let numbers = [1, 2, 3];

    let x: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("x: {:?}", x);

    let mut x: Vec<i32> = vec![1, 2, 3];

    let y: Vec<i32> = x.iter().map(|x| x * 3).collect();

    println!("y: {:?}", y);

    for e in x.iter_mut() {
        *e += 2;
    }
    println!("x: {:?}", x);

    let vector = (1..)
        .filter(|x| x % 2 != 0)
        .take(5)
        .map(|x| x * x)
        .collect::<Vec<usize>>();

    println!("vector: {:?}", vector);

    let x = (1..100).take(10).collect::<Vec<usize>>();
    println!("x: {:?}", x);

    let sentence = "This is a sentence in Rust.";

    let words: Vec<&str> = sentence.split_whitespace().collect();

    println!("words: {:?}", words);

    let words_containing_i: Vec<&str> = words
        // .iter()
        .into_iter()
        .filter(|word| word.contains("i"))
        .collect();

    println!("words_containing_i: {:?}", words_containing_i);
}
