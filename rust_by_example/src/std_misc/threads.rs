use std::thread;
use std::time::Duration;

static NTHREADS: i32 = 10;

pub fn run() {}

pub fn ex0() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn ex1() {
    // Make a Vector to hold the children which are spawned
    let mut kids = Vec::new();

    for i in 0..NTHREADS {
        kids.push(thread::spawn(move || {
            sleeping_beauty(i, rando());
        }));
    }

    // Needed to wait for threads to finish, otherwise they get killed.
    for kid in kids {
        let _ = kid.join();
    }
}

fn sleeping_beauty(thread: i32, sleep: u32) {
    println!(
        "This is thread number {} and will now sleep for {} seconds",
        thread, sleep
    );
    thread::sleep(Duration::from_secs(sleep as u64));
    println!("Thread {}, just finished sleeping.", thread);
}

fn rando() -> u32 {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    nanos % 9
}
