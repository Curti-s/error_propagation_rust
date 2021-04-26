// Error propagation: entails returning the error to the calling code 
// so that it can decide what to do; hence giving more control to the
// calling code.
#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io;
use std::io::Read;

// ? placed after a Result enum value; if the value is an Ok variant, the value inside
// the Ok will get returned from this expression & the program will continue.
// If the value is an Err variant, Err will be returned from the whole function as if
// return keyword was used & propagated to the calling code.
//
// Error values that have the ? operator called upon them go through the `from` function,
// defined w/i the `From` trait, that is used to convert errors from one type to another.
// The error type is converted into the error type defined w/i the return type of the current
// function.
fn read_username_from_file_1() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
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
