use std::fs::File;
use std::io::Read;

use crate::consts::MethodAccessFlags;
use crate::java::ClassInfo;

mod consts;
mod java;

fn main() {
    println!("Hello, world!");
    let mut f = File::open("./run/test.class").expect("hahafunny");
    let mut buffer = Vec::new();
    // read the whole file

    f.read_to_end(&mut buffer).expect("fuckk");

    let (_buffer, class_info) = ClassInfo::parse(&buffer).unwrap();

    for x in class_info.methods {
        let i = x.access_flags;
        if i.contains(MethodAccessFlags::PUBLIC) {
            println!("public");
        } else if i.contains(MethodAccessFlags::PROTECTED) {
            println!("protected");
        } else if i.contains(MethodAccessFlags::PRIVATE) {
            println!("private");
        } else {
            println!("package-private");
        }
    }
    // and more! See the other methods for more details.
}
