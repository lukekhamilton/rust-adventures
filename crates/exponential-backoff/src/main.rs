extern crate exponential_backoff;

use exponential_backoff::Backoff;
use std::io::Error;
use std::{fs, thread, time::Duration};

fn read_file(file_name: &str) -> std::io::Result<String> {
    println!("Hello, exponential_backoff");

    let retrise = 3;
    let backoff = Backoff::new(retrise)
        .timeout_range(Duration::from_millis(100), Duration::from_secs(10))
        .jitter(0.3)
        .factor(2);

    for duration in &backoff {
        match fs::read_to_string(file_name) {
            Ok(str) => {
                println!("Ok = {:?}", str);
                return Ok(str);
            }
            Err(err) => match duration {
                Some(duration) => {
                    println!("sleepping for ... {:?}", duration);
                    thread::sleep(duration)
                }
                None => {
                    println!("Err = {:?}", err);
                    return Err(err);
                }
            },
        }
    }

    unreachable!();
}

#[test]
fn test_main_known_file() {
    let result = read_file("test.md").unwrap();
    assert_eq!(result, "# Test File")
}

#[test]
fn test_main_unknown_file() {
    let result = read_file("bad.file").err().unwrap().to_string();
    assert_eq!(result, "No such file or directory (os error 2)")
}
