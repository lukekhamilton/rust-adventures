extern crate backoff;
extern crate reqwest;

use backoff::{Error, ExponentialBackoff, Operation};
use reqwest::Url;

use std::fmt::Display;
use std::io::{self, Read};

fn new_io_err<E: Display>(err: E) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err.to_string())
}

fn fetch_url(url: &str) -> Result<String, Error<io::Error>> {
    let mut op = || {
        println!("Fetching {}", url);
        let url = Url::parse(url)
            .map_err(new_io_err)
            // Permanent errors need to be explicitly constucted.
            .map_err(Error::Permanent)?;

        let mut resp = reqwest::blocking::get(url)
            // Transient errors can be constructed with the ? operator
            // or with the try! macro. No explicit conversion needed
            // from E: Error to backoff::Error;
            .map_err(new_io_err)?;

        let mut content = String::new();
        let _ = resp.read_to_string(&mut content);
        Ok(content)
    };

    let mut backoff = ExponentialBackoff::default();
    op.retry(&mut backoff)
}

#[test]
fn test_fetch_url() {
    let content = fetch_url("https://httpbin.org/ip").unwrap();
    assert_eq!(content, "{\n  \"origin\": \"103.212.227.125\"\n}\n")
}

#[test]
fn test_fetch_url_retry() {
    let content = fetch_url("https://bad.c.au").err().unwrap().to_string();
    assert_eq!(content, "")
}
