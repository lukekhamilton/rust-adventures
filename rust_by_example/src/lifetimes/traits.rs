#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

pub fn run() {
    let b: Borrowed = Default::default();
    println!("b: {:?}", b);
}
