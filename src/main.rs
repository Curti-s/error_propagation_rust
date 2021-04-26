// Error propagation: entails returning the error to the calling code 
// so that it can decide what to do; hence giving more control to the
// calling code.
#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;
use std::io;

fn read_username_from_file_1() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    let username_1 = read_username_from_file_1();
    let username_1 = match username_1 {
        Ok(u) => String::from(u),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => panic!("file not found: error kind: {:?}", e.kind()),
            _ => panic!("Problem"),
            },
    };

    println!("username {} ", username_1);
}
