// Error propagation: entails returning the error to the calling code 
// so that it can decide what to do; hence giving more control to the
// calling code.
#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file_1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // here we return early from the fn & pass the error value 
        // from File::open back to the calling code as this fn's error value.
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e), // here, no need to explicitly return, since this is the last expression.
    }
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
