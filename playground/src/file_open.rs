use std::fs::File;
use std::io::{Error, Read};

pub fn run() {
    // let f = File::open("1README.md");

    // match &f {
    //     Ok(f) => println!("File found: {:?}", f),
    //     Err(e) => {
    //         println!("Err: {:?}", e);
    //     }
    // };

    // let f = match File::open("README.md") {
    //     Ok(file) => file,
    //     Err(e) => println!("ERROR {:?}", e),
    // };

    // let f = match File::open("README.md") {
    //     Ok(file) => file,
    //     Err(e) => panic!("ERROR: {:?}", e),
    // };

    // println!("file: {:?}", file);

    // let r = read_file().unwrap();

    // println!("r: {:?}", r);

    let f = read();

    println!("{:?}", f);
}

fn read_file() -> Result<String, Error> {
    let f = File::open("README.md");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read() -> Result<File, Error> {
    let f = match File::open("README.md") {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    Ok(f)
}
