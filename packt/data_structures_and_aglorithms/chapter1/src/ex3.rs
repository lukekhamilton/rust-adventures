use std::thread;

pub fn threading() {
    let x = 10;
    let handle = thread::spawn(move || {
        println!("Hello from a thread, the number is {}", x);
    });

    handle.join().unwrap();
}