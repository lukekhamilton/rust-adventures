use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // let serialised = serde_json::to_string(&point).unwrap();
    let serialized = serde_json::to_string(&point).unwrap();
    // Print
    println!("serialized = {}", serialized);

    //  Convert the JSON string back to a Point
    let deserialised: Point = serde_json::from_str(&serialized).unwrap();

    println!("deserialised = {:?}", deserialised);
}
