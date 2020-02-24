struct Foo<'a> {
    x: Option<&'a u32>,
}

fn main(){
    // problem();
    fix();
}

// fn problem() {
//     let mut x = Foo { x: None };
//     {
//         let y = 0;
//         x.x = Some(&y); // error: `y` does not live long enough
//     }
//     println!("{:?}", x.x);
// }

fn fix() {
    let mut x = Foo { x: None };

    let mut y = 1;
    x.x = Some(&y); // Borrowed

    println!("x.x: {:?}", x.x);

    println!("y: {:?}", y);

    y = 2;
    println!("y: {:?}", y);

    // println!("x.x: {:?}", x.x);
}