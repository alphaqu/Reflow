use std::fs::File;
use std::io::Read;
use bytes::Buf;

mod java;
mod opcodes;

fn main() {
    println!("Hello, world!");
    let mut f = File::open("./run/test.class").expect("hahafunny");
    let mut buffer = Vec::new();
    // read the whole file

    f.read_to_end(&mut buffer).expect("fuckk");

    let reader1 = &mut buffer.as_slice();
    let class_info = java::get_class_info(reader1);
    for x in class_info.methods {
        let i = x.access_flags;
        if (i & opcodes::ACC_PUBLIC) != 0 {
            println!("public");
        } else if (i & opcodes::ACC_PROTECTED) != 0 {
            println!("protected");
        } else if (i & opcodes::ACC_PRIVATE) != 0 {
            println!("private");
        } else {
            println!("package-private");
        }
    }
    // and more! See the other methods for more details.
}

fn printAccess() {}

