use std::io::{Cursor, Read, Seek, SeekFrom};
use std::str::{from_utf8, from_utf8_mut};

use simple_bytes::*;
use ux::*;

use crate::java::ConstantInfo::MethodType;

pub struct ClassInfo {
    pub magic: u32,

    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<ConstantInfo>,
    pub access_flags: u16,

    pub this_class: u16,
    pub super_class: u16,

    pub interfaces: Vec<u16>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
}


//ClassFile {
//     u32             magic;
//     u16             minor_version;
//     u16             major_version;

//     u16             constant_pool_count;
//     cp_info        constant_pool[constant_pool_count-1];
//     u16             access_flags;
//     u16             this_class;
//     u16             super_class;
//     u16             interfaces_count;
//     u16             interfaces[interfaces_count];
//     u16             fields_count;
//     field_info     fields[fields_count];
//     u16             methods_count;
//     method_info    methods[methods_count];
//     u16             attributes_count;
//     attribute_info attributes[attributes_count];
//}
pub fn read_class_info(mut reader: &Bytes) -> ClassInfo {
    let magic = reader.read_u32();
    let minor_version = reader.read_u16();
    let major_version = reader.read_u16();
    let constant_pool_count = reader.read_u32();

    let mut constant_pool: Vec<ConstantInfo> = Vec::new();
    for i in 0..constant_pool_count {
        constant_pool.push(read_constant_info(reader));
    };

    let access_flags = reader.read_u16();
    let this_class = reader.read_u16();
    let super_class = reader.read_u16();

    let interfaces_count = reader.read_u32();
    let mut interfaces: Vec<u16> = Vec::new();
    for i in 0..interfaces_count {
        interfaces.push(reader.read_u16());
    };

    let field_count = reader.read_u32();
    let mut field: Vec<FieldInfo> = Vec::new();
    for i in 0..field_count {
        field.push(read_field(reader));
    };
}


//field_info {
//     u16             access_flags;
//     u16             name_index;
//     u16             descriptor_index;
//     u16             attributes_count;
//     attribute_info attributes[attributes_count];
// }
pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attribute_info: Vec<AttributeInfo>,
}

pub fn read_field(mut reader: &Bytes) -> FieldInfo {
    let access_flags = reader.read_u16();
    let name_index = reader.read_u16();
    let descriptor_index = reader.read_u16();

    let attributes_count = reader.read_u16();
    let mut attribute_info: Vec<AttributeInfo> = Vec::new();
    for i in 0..attributes_count {
        attribute_info.push(read_attribute_info(reader));
    };
    FieldInfo {
        access_flags,
        name_index,
        descriptor_index,
        attribute_info,
    }
}

//method_info {
//     u16             access_flags;
//     u16             name_index;
//     u16             descriptor_index;
//     u16             attributes_count;
//     attribute_info attributes[attributes_count];
// }
pub struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

pub enum ConstantInfo {
    Class { name_index: u16 },
    Field { class_index: u16, name_and_type_index: u16 },
    Method { class_index: u16, name_and_type_index: u16 },
    Interface { class_index: u16, name_and_type_index: u16 },
    String { string_index: u16 },
    Integer { bytes: u32 },
    Float { bytes: u32 },
    Long { bytes: u8 },
    Double { bytes: u8 },
    NameAndType { name_index: u16, descriptor_index: u16 },
    UTF8 { text: String },
    MethodHandle { reference_kind: u8, reference_index: u16 },
    MethodType { descriptor_index: u16 },
    InvokeDynamic { bootstrap_method_attr_index: u16, name_and_type_index: u16 },
}

pub fn read_constant_info(mut reader: &Bytes) -> ConstantInfo {
    let tag = reader.read_u8();

    match u8::from(tag) {
        7 => ConstantInfo::Class { name_index: reader.read_u16() },
        9 => ConstantInfo::Field {
            class_index: reader.read_u16(),
            name_and_type_index: reader.read_u16(),
        },
        10 => ConstantInfo::Method {
            class_index: reader.read_u16(),
            name_and_type_index: reader.read_u16(),
        },
        11 => ConstantInfo::Interface {
            class_index: reader.read_u16(),
            name_and_type_index: reader.read_u16(),
        },
        8 => ConstantInfo::String { string_index: reader.read_u16() },
        3 => ConstantInfo::Integer { bytes: reader.read_u32() },
        4 => ConstantInfo::Float { bytes: reader.read_u32() },
        5 => ConstantInfo::Long { bytes: reader.read_u8() },
        6 => ConstantInfo::Double { bytes: reader.read_u8() },
        12 => ConstantInfo::NameAndType {
            name_index: reader.read_u16(),
            descriptor_index: reader.read_u16(),
        },
        1 => {
            let length = reader.read_u32();
            let mut bytes: Vec<u8> = Vec::new();

            for i in 0..length {
                bytes.push(reader.read_u8())
            }
            ConstantInfo::UTF8 {
                text: String::from_utf8(bytes).expect("Failed to create string.")
            }
        }
        15 => ConstantInfo::MethodHandle {
            reference_kind: reader.read_u8(),
            reference_index: reader.read_u16(),
        },
        16 => ConstantInfo::MethodType {
            descriptor_index: reader.read_u16()
        },
        18 => ConstantInfo::InvokeDynamic {
            bootstrap_method_attr_index: reader.read_u16(),
            name_and_type_index: reader.read_u16(),
        },
        _ => panic!("crash and burn")
    }
}


//attribute_info {
//     u16 attribute_name_index;
//     u32 attribute_length;
//     u8 info[attribute_length];
// }
pub struct AttributeInfo {
    attribute_name_index: u16,
    info: Vec<u8>,
}


pub fn read_attribute_info(mut reader: &Bytes) -> AttributeInfo {
    let attribute_name_index = reader.read_u16();
    let attribute_length = reader.read_u32();
}
