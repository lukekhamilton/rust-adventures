use std::fmt;
use std::fmt::{Error, Formatter};

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

pub fn run() {
    let c = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Display: {}", c);
    println!("Debug: {:?}", c);
}
