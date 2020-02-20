use std::fmt;
use std::fmt::Formatter;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to vec.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over 'v' in 'vec' while enumerating the interation
        // count in 'count'
        //        for (count, v) in vec.iter().enumerate() {
        //            if count != 0 {
        //                write!(f, ", ")?;
        //            }
        //            write!(f, "{}", v)?;
        //        }

        // Backwards for fun
        for (count, v) in vec.iter().enumerate().rev() {
            if count != vec.len() - 1 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }

        // close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

pub fn run() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v)
}
