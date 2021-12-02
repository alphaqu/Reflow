use std::fs::File;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::consts::{MethodAccessFlags, print_op};
use crate::java::{AttributeInfo, ClassInfo};

mod consts;
mod java;

fn main() {
    println!("Hello, world!");
    let mut f = File::open("./run/Test.class").expect("hahafunny");
    let mut buffer = Vec::new();
    // read the whole file

    f.read_to_end(&mut buffer).expect("fuckk");

    let start = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");

    for _i in 0..12000 {
        let (_buffer, _class_info) = ClassInfo::parse(&buffer).unwrap();
    }
    let stop = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");

    println!("Read classes in {}ms",stop.as_millis() - start.as_millis() );

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

        for x in x.attribute_info {
            match x {
                java::AttributeInfo::Code { max_stack, max_locals, code, exception_table, attribute_info } => {
                    for x in code {
                        consts::print_op(x.get_op())
                    }
                }
                _ => {}
            }
        }
    }
    // and more! See the other methods for more details.
}
