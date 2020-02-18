use std::fmt;
use std::fmt::{Error, Formatter};

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        //        unimplemented!()
        write!(f, "{}", self.0)
    }
}

pub fn run() {
    let s = Structure(11);
    println!("{}", s);
}
