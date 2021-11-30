use std::io::{Cursor, Read, Seek, SeekFrom};
use std::str::{from_utf8, from_utf8_mut};

use bitreader::BitReader;
use ux::*;

use crate::java::ConstantInfo::MethodType;

pub struct ClassInfo {
    pub magic: u4,

    pub minor_version: u2,
    pub major_version: u2,
    pub constant_pool: Vec<ConstantInfo>,
    pub access_flags: u2,

    pub this_class: u2,
    pub super_class: u2,

    pub interfaces: Vec<u2>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
}


//ClassFile {
//     u4             magic;
//     u2             minor_version;
//     u2             major_version;

//     u2             constant_pool_count;
//     cp_info        constant_pool[constant_pool_count-1];
//     u2             access_flags;
//     u2             this_class;
//     u2             super_class;
//     u2             interfaces_count;
//     u2             interfaces[interfaces_count];
//     u2             fields_count;
//     field_info     fields[fields_count];
//     u2             methods_count;
//     method_info    methods[methods_count];
//     u2             attributes_count;
//     attribute_info attributes[attributes_count];
//}
pub fn read_class_info(mut reader: &BitReader) -> ClassInfo {
    let magic = read_u4(reader, "magic");
    let minor_version = read_u2(reader, "minor_version");
    let major_version = read_u2(reader, "major_version");
    let constant_pool_count = read_u(reader, 2, "constant_pool_count");

    let mut constant_pool: Vec<ConstantInfo> = Vec::new();
    for i in 0..constant_pool_count {
        constant_pool.push(read_constant_info(reader));
    };

    let access_flags = read_u2(reader, "access_flags");
    let this_class = read_u2(reader, "this_class");
    let super_class = read_u2(reader, "super_class");

    let interfaces_count = read_u(reader, 2, "interfaces_count");
    let mut interfaces: Vec<u2> = Vec::new();
    for i in 0..interfaces_count {
        interfaces.push(read_u2(reader, "interface"));
    };

    let field_count = read_u(reader, 2, "field_count");
    let mut field: Vec<FieldInfo> = Vec::new();
    for i in 0..field_count {
        field.push(read_u2(reader, "interface"));
    };


}


//field_info {
//     u2             access_flags;
//     u2             name_index;
//     u2             descriptor_index;
//     u2             attributes_count;
//     attribute_info attributes[attributes_count];
// }
pub struct FieldInfo {
    access_flags: u2,
    name_index: u2,
    descriptor_index: u2,
    attributes: Vec<AttributeInfo>,
}
pub fn read_field(mut reader: &BitReader) -> FieldInfo {
    let access_flags = read_u2(reader, "field access_flags");
    let name_index = read_u2(reader, "field name_index");
    let descriptor_index = read_u2(reader, "field descriptor_index");

    let attributes_count = read_u(reader, 2, "field attributes_count");
    let mut attribute_info: Vec<AttributeInfo> = Vec::new();
    for i in 0..attributes_count {
        attribute_info.push(read_u2(reader, "field attribute_info"));
    };
    FieldInfo {
        access_flags,
        name_index,
        descriptor_index,
        attributes_count: read_u2(reader, "field attributes_count"),
        attributes: vec![]
    }
}
//method_info {
//     u2             access_flags;
//     u2             name_index;
//     u2             descriptor_index;
//     u2             attributes_count;
//     attribute_info attributes[attributes_count];
// }
pub struct MethodInfo {
    access_flags: u2,
    name_index: u2,
    descriptor_index: u2,
    attributes: Vec<AttributeInfo>,
}

pub enum ConstantInfo {
    Class { name_index: u2 },
    Field { class_index: u2, name_and_type_index: u2 },
    Method { class_index: u2, name_and_type_index: u2 },
    Interface { class_index: u2, name_and_type_index: u2 },
    String { string_index: u2 },
    Integer { bytes: u4 },
    Float { bytes: u4 },
    Long { bytes: u8 },
    Double { bytes: u8 },
    NameAndType { name_index: u2, descriptor_index: u2 },
    UTF8 { text: String },
    MethodHandle { reference_kind: u1, reference_index: u2 },
    MethodType { descriptor_index: u2 },
    InvokeDynamic { bootstrap_method_attr_index: u2, name_and_type_index: u2 },
}

pub fn read_constant_info(mut reader: &BitReader) -> ConstantInfo {
    let tag = read_u(reader, 1, "Constant Tag");

    match u8::from(tag) {
        7 => ConstantInfo::Class { name_index: read_u2(reader, "ConstantClass name_index") },
        9 => ConstantInfo::Field {
            class_index: read_u2(reader, "ConstantField class_index"),
            name_and_type_index: read_u2(reader, "ConstantField name_and_type_index"),
        },
        10 => ConstantInfo::Method {
            class_index: read_u2(reader, "ConstantMethod class_index"),
            name_and_type_index: read_u2(reader, "ConstantMethod name_and_type_index"),
        },
        11 => ConstantInfo::Interface {
            class_index: read_u2(reader, "ConstantInterface class_index"),
            name_and_type_index: read_u2(reader, "ConstantInterface name_and_type_index"),
        },
        8 => ConstantInfo::String {
            string_index: read_u2(reader, "ConstantString string_index")
        },
        3 => ConstantInfo::Integer { bytes: read_u4(reader, "ConstantInteger bytes") },
        4 => ConstantInfo::Float { bytes: read_u4(reader, "ConstantFloat bytes") },
        5 => ConstantInfo::Long { bytes: read_u8(reader, "ConstantLong bytes") },
        6 => ConstantInfo::Double { bytes: read_u8(reader, "ConstantDouble bytes") },
        12 => ConstantInfo::NameAndType {
            name_index: read_u2(reader, "ConstantNameAndType name_index"),
            descriptor_index: read_u2(reader, "ConstantNameAndType descriptor_index"),
        },
        1 => {
            let length = read_u(reader, 2, "ConstantUTF8 length");
            let mut bytes: Vec<u8> = Vec::new();

            for i in 0..length {
                bytes.push(reader.read_u8(8).expect("ConstantUTF8 stringByte"))
            }
            ConstantInfo::UTF8 {
                text: String::from_utf8(bytes).expect("Failed to create string.")
            }
        }
        15 => ConstantInfo::MethodHandle {
            reference_kind: read_u1(reader, "ConstantMethodHandle reference_kind"),
            reference_index: read_u2(reader, "ConstantMethodHandle reference_index"),
        },
        16 => ConstantInfo::MethodType {
            descriptor_index: read_u2(reader, "ConstantMethodType descriptor_index")
        },
        18 => ConstantInfo::InvokeDynamic {
            bootstrap_method_attr_index: read_u2(reader, "ConstantInvokeDynamic bootstrap_method_attr_index"),
            name_and_type_index: read_u2(reader, "ConstantInvokeDynamic name_and_type_index"),
        },
        _ => panic!("crash and burn")
    }
}

pub fn read_u(mut reader: &BitReader, bit_count: u8, reading: &str) -> u8 {
    reader.read_u8(bit_count).expect(&*("Failed to read ".to_owned() + reading))
}

pub fn read_u1(mut reader: &BitReader, reading: &str) -> u1 {
    u1::new(read_u(reader, 1, reading))
}

pub fn read_u2(mut reader: &BitReader, reading: &str) -> u2 {
    u2::new(read_u(reader, 2, reading))
}

pub fn read_u4(mut reader: &BitReader, reading: &str) -> u4 {
    u4::new(read_u(reader, 4, reading))
}

pub fn read_u8(mut reader: &BitReader, reading: &str) -> u8 {
    read_u(reader, 8, reading)
}

//attribute_info {
//     u2 attribute_name_index;
//     u4 attribute_length;
//     u1 info[attribute_length];
// }
pub struct AttributeInfo {
    attribute_name_index: u2,
    info: Vec<u1>,
}
