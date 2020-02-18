use std::fmt;
use std::fmt::{Error, Formatter};

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "[")?;

        let vec = &self.0;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}:{}", count, v)?;
        }
        write!(f, "]")
    }
}

pub fn run() {
    let l = List(vec![1, 2, 3]);
    println!("{}", l);
}
