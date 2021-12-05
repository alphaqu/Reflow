use std::fs::File;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::consts::{MethodAccessFlags, print_op};
use crate::java::{AttributeInfo, ClassInfo};
use crate::java_code::{Code, CodeChunkTarget};

mod consts;
mod java;
mod java_code;
mod java_decomp;
mod java_type;

fn main() {
    println!("Hello, world!");
    let mut f = File::open("./run/Test.class").expect("hahafunny");
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

        for x in x.attribute_info {
            match x {
                java::AttributeInfo::CodeAttribute { code } => {
                    println!();
                    println!();
                    let mut i = 0;
                    for x in code.code_chunks {
                        for x in x.start..x.stop {
                            let op = &code.code[x as usize];
                            println!("{}", op.print());
                        }

                    //   print!("{} [label=\"", i);
                    //   for x in x.start..x.stop {
                    //       let op = &code.code[x as usize];
                    //       print!("{}\n", op.print());
                    //   }
                    //   println!("\"];");
                    //   //print!("{}-{} ", x.start, x.stop);
                    //  match x.target {
                    //      CodeChunkTarget::Basic => {
                    //          println!("{} -> {}", i, i + 1);
                    //      }
                    //      CodeChunkTarget::Return => {
                    //          println!("{}", i);
                    //      }
                    //      CodeChunkTarget::Throw => {

                    //      }
                    //      CodeChunkTarget::Condition { true_chunk_pos } => {
                    //          println!("{} -> {}", i, i + 1);
                    //          println!("{} -> {}", i, true_chunk_pos + 1);
                    //      }
                    //      CodeChunkTarget::Goto { chunk_pos } => {
                    //          println!("{} -> {}", i, chunk_pos + 1);
                    //      }
                    //      CodeChunkTarget::Switch { .. } => {}
                    //  }
                        i += 1;
                    }
                }
                _ => {}
            }
        }
    }
    // and more! See the other methods for more details.
}
