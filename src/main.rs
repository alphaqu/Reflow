use std::fs::File;
use std::io::{Cursor, SeekFrom};
use std::io::prelude::*;

mod java;

fn main() {
    println!("Hello, world!");
    let mut f = File::open("./run/test.class").expect("hahafunny");
    let mut buffer = Vec::new();
    // read the whole file

    f.read_to_end(&mut buffer).expect("fuckk");
    let mut cursor = Cursor::new(buffer);
    cursor.seek(SeekFrom::Current(10));


    for x in buffer {
        println!("{}", x)
    }
    // and more! See the other methods for more details.
}

