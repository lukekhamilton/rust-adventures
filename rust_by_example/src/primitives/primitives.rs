pub fn run() {
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let b_float = 2.0;
    let c_float = 1e9;

    let a_integer: i32 = 123;
    let b_integer = 5i32;

    let mut inferred_type = 12;
    inferred_type = 4294967296i64;
    inferred_type = 1i64;

    let mut mutable = 12;
    mutable = 21;

    let mutable = true;
}
