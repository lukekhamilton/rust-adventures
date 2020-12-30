use std::thread;
use std::sync::{Mutex, Arc};

pub fn shared_state() {
    let v = Arc::new(Mutex::new(vec![]));

    let handles = (0..10).map(|i|{
        let numbers = Arc::clone(&v);
        thread::spawn(move || {
            let mut vector = numbers
                .lock()
                .unwrap();
            (*vector).push(i);
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", *v.lock().unwrap());

    print!("{:#?}", v);
}