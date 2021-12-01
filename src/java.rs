use nom::bytes::complete::{tag, take};
use nom::combinator::{map, map_opt, map_res};
use nom::error::{make_error, ErrorKind};
use nom::multi::{length_count, length_data};
use nom::number::complete::{be_u16, be_u32, be_u64, be_u8};
use nom::sequence::pair;
use num_traits::FromPrimitive;

use crate::consts::{Opcode, MethodAccessFlags, FieldAccessFlags, ClassAccessFlags};

#[derive(Debug)]
pub struct ClassInfo {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: ClassAccessFlags,

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

// let fields_size = input.get_u16() as usize;
// let mut fields = Vec::with_capacity(fields_size);
// fields.fill_with(|| FieldInfo::parse(input, &constant_pool));

// let methods_size = input.get_u16() as usize;
// let mut methods = Vec::with_capacity(methods_size);
// for i in 0..methods_size {
//     methods.insert(i, get_method(i, &constant_pool));
// }
type IResult<'a, O> = nom::IResult<&'a [u8], O>;

impl ClassInfo {
    pub fn parse<'input>(input: &'input [u8]) -> IResult<'input, Self> {
        let (input, _) = tag(b"\xca\xfe\xba\xbe")(input)?;
        let (input, minor_version) = be_u16(input)?;
        let (input, major_version) = be_u16(input)?;

        let (input, constant_pool) = map(
            length_count(map(be_u16, |num| num - 1), ConstantInfo::parse),
            |v| ConstantPool(v),
        )(input)?;

        let (input, access_flags) = map_opt(be_u16, ClassAccessFlags::from_bits)(input)?;
        let (input, this_class) = be_u16(input)?;
        let (input, super_class) = be_u16(input)?;
        let (input, interfaces) = length_count(be_u16, be_u16)(input)?;

        let (input, fields) =
            length_count(be_u16, |input| FieldInfo::parse(input, &constant_pool))(input)?;
        let (input, methods) =
            length_count(be_u16, |input| MethodInfo::parse(input, &constant_pool))(input)?;
        let (input, attributes) =
            length_count(be_u16, |input| AttributeInfo::parse(input, &constant_pool))(input)?;

        Ok((
            input,
            ClassInfo {
                minor_version,
                major_version,
                constant_pool,
                access_flags,
                this_class,
                super_class,
                interfaces,
                fields,
                methods,
                attributes,
            },
        ))
    }
}

//field_info {
//     u16             access_flags;
//     u16             name_index;
//     u16             descriptor_index;
//     u16             attributes_count;
//     attribute_info attributes[attributes_count];
// }
#[derive(Debug)]
pub struct FieldInfo {
    access_flags: FieldAccessFlags,
    name_index: u16,
    descriptor_index: u16,
    attribute_info: Vec<AttributeInfo>,
}

impl FieldInfo {
    pub fn parse<'input>(
        input: &'input [u8],
        constant_pool: &ConstantPool,
    ) -> IResult<'input, Self> {
        let (input, access_flags) = map_opt(be_u16, FieldAccessFlags::from_bits)(input)?;
        let (input, name_index) = be_u16(input)?;
        let (input, descriptor_index) = be_u16(input)?;
        let (input, attribute_info) =
            length_count(be_u16, |input| AttributeInfo::parse(input, constant_pool))(input)?;

        Ok((
            input,
            Self {
                access_flags,
                name_index,
                descriptor_index,
                attribute_info,
            },
        ))
    }
}

//method_info {
//     u16             access_flags;
//     u16             name_index;
//     u16             descriptor_index;
//     u16             attributes_count;
//     attribute_info attributes[attributes_count];
// }

#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: MethodAccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attribute_info: Vec<AttributeInfo>,
}

impl MethodInfo {
    pub fn parse<'input>(
        input: &'input [u8],
        constant_pool: &ConstantPool,
    ) -> IResult<'input, Self> {
        let (input, access_flags) = map_opt(be_u16, MethodAccessFlags::from_bits)(input)?;
        let (input, name_index) = be_u16(input)?;
        let (input, descriptor_index) = be_u16(input)?;
        let (input, attribute_info) =
            length_count(be_u16, |input| AttributeInfo::parse(input, constant_pool))(input)?;

        Ok((
            input,
            Self {
                access_flags,
                name_index,
                descriptor_index,
                attribute_info,
            },
        ))
    }
}

#[derive(Debug)]
pub enum ConstantInfo {
    Class {
        name_index: u16,
    },
    Field {
        class_index: u16,
        name_and_type_index: u16,
    },
    Method {
        class_index: u16,
        name_and_type_index: u16,
    },
    Interface {
        class_index: u16,
        name_and_type_index: u16,
    },
    String {
        string_index: u16,
    },
    Integer {
        bytes: u32,
    },
    Float {
        bytes: u32,
    },
    Long {
        bytes: u64,
    },
    Double {
        bytes: u64,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    UTF8 {
        text: String,
    },
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
}

impl ConstantInfo {
    pub fn parse<'input>(input: &'input [u8]) -> IResult<'input, Self> {
        let (input, variant) = be_u8(input)?;
        match variant {
            7 => map(be_u16, |name_index| ConstantInfo::Class { name_index })(input),
            9 => map(
                pair(be_u16, be_u16),
                |(class_index, name_and_type_index)| ConstantInfo::Field {
                    class_index,
                    name_and_type_index,
                },
            )(input),
            10 => map(
                pair(be_u16, be_u16),
                |(class_index, name_and_type_index)| ConstantInfo::Method {
                    class_index,
                    name_and_type_index,
                },
            )(input),
            11 => map(
                pair(be_u16, be_u16),
                |(class_index, name_and_type_index)| ConstantInfo::Interface {
                    class_index,
                    name_and_type_index,
                },
            )(input),
            8 => map(be_u16, |string_index| ConstantInfo::String { string_index })(input),
            3 => map(be_u32, |bytes| ConstantInfo::Integer { bytes })(input),
            4 => map(be_u32, |bytes| ConstantInfo::Float { bytes })(input),
            5 => map(be_u64, |bytes| ConstantInfo::Long { bytes })(input),
            6 => map(be_u64, |bytes| ConstantInfo::Double { bytes })(input),
            12 => map(pair(be_u16, be_u16), |(name_index, descriptor_index)| {
                ConstantInfo::NameAndType {
                    name_index,
                    descriptor_index,
                }
            })(input),
            1 => map_res(
                length_data(be_u16),
                //FIXME(leocth): Java uses MUTF-8, which Rust does *not* expect. https://en.wikipedia.org/wiki/UTF-8#Modified_UTF-8
                |data: &[u8]| {
                    String::from_utf8(data.into()).map(|text| ConstantInfo::UTF8 { text })
                },
            )(input),
            15 => map(pair(be_u8, be_u16), |(reference_kind, reference_index)| {
                ConstantInfo::MethodHandle {
                    reference_kind,
                    reference_index,
                }
            })(input),
            16 => map(be_u16, |descriptor_index| ConstantInfo::MethodType {
                descriptor_index,
            })(input),
            18 => map(
                pair(be_u16, be_u16),
                |(bootstrap_method_attr_index, name_and_type_index)| ConstantInfo::InvokeDynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                },
            )(input),
            _ => return Err(nom::Err::Error(make_error(input, ErrorKind::Alt))),
        }
    }
}

#[derive(Debug)]
pub struct AttributeException {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

#[derive(Debug)]
pub struct AttributeClass {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: u16,
}

#[derive(Debug)]
pub struct AttributeLineNumber {
    start_pc: u16,
    line_number: u16,
}

#[derive(Debug)]
pub struct AttributeLocalVariable {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

#[derive(Debug)]
pub struct AttributeLocalVariableType {
    start_pc: u16,
    length: u16,
    name_index: u16,
    signature_index: u16,
    index: u16,
}

#[derive(Debug)]
pub struct AttributeBootstrapMethod {
    bootstrap_method_ref: u16,
    bootstrap_arguments: Vec<u16>,
}

#[derive(Debug)]
pub enum AttributeInfo {
    ConstantValue {
        constant_index: u16,
    },
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<AttributeException>,
        attribute_info: Vec<AttributeInfo>,
    },
    // TODO do this
    StackMapTable,
    Exceptions {
        exception_index_table: Vec<u16>,
    },
    InnerClasses {
        classes: Vec<AttributeClass>,
    },
    EnclosingMethod {
        class_index: u16,
        method_index: u16,
    },
    Synthetic,
    Signature {
        signature_index: u16,
    },
    SourceFile {
        source_file_index: u16,
    },
    SourceDebugExtension {
        debug_extension: Vec<u8>,
    },
    LineNumberTable {
        line_number_table: Vec<AttributeLineNumber>,
    },
    LocalVariableTable {
        local_variable_table: Vec<AttributeLocalVariable>,
    },
    LocalVariableTypeTable {
        local_variable_type_table: Vec<AttributeLocalVariableType>,
    },
    Deprecated,
    // TODO do this
    RuntimeInvisibleAnnotations,
    // TODO do this
    RuntimeVisibleParameterAnnotations,
    // TODO do this
    RuntimeInvisibleParameterAnnotations,
    // TODO do this
    AnnotationDefault,
    // TODO do this
    RuntimeVisibleAnnotations,
    BootstrapMethods {
        bootstrap_methods: Vec<AttributeBootstrapMethod>,
    },
}

impl AttributeInfo {
    pub fn parse<'input>(
        input: &'input [u8],
        constant_pool: &ConstantPool,
    ) -> IResult<'input, Self> {
        let (input, info) = map_opt(be_u16, |index| constant_pool.get(index))(input)?;
        let (input, length) = be_u32(input)?;

        match info {
            // ConstantInfo::UTF8 { text } => match text.as_str() {
            //     "ConstantValue" => map(be_u16, |constant_index| AttributeInfo::ConstantValue {
            //         constant_index,
            //     })(input),
            //     "Code" => map(
            //         tuple((be_u16, be_u16, be_u32)),
            //         |(max_stack, max_locals, _code_length)| AttributeInfo::Code {
            //             max_stack,
            //             max_locals,
            //             code: vec![],
            //             exception_table: vec![],
            //             attribute_info: vec![],
            //         },
            //     )(input),
            //     "StackMapTable" => todo!(),
            //     "Exceptions" => todo!(),
            //     "InnerClasses" => todo!(),
            //     "EnclosingMethod" => todo!(),
            //     "Synthetic" => todo!(),
            //     "Signature" => todo!(),
            //     "SourceFile" => todo!(),
            //     "SourceDebugExtension" => todo!(),
            //     "LineNumberTable" => todo!(),
            //     "LocalVariableTable" => todo!(),
            //     "LocalVariableTypeTable" => todo!(),
            //     "Deprecated" => todo!(),
            //     "RuntimeVisibleAnnotations" => todo!(),
            //     "RuntimeInvisibleAnnotations" => todo!(),
            //     "RuntimeVisibleParameterAnnotations" => todo!(),
            //     "RuntimeInvisibleParameterAnnotations" => todo!(),
            //     "AnnotationDefault" => todo!(),
            //     "BootstrapMethods" => todo!(),
            //     _ => Ok((input, AttributeInfo::AnnotationDefault)),
            // },
            // discard the remaining bytes
            _ => map(take(length), |_| AttributeInfo::AnnotationDefault)(input),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Basic {
        op: Opcode,
    },
    Typed {
        op: Opcode,
        type_index: u16,
    },
    Var {
        op: Opcode,
        var: u32,
    },
    Jump {
        op: Opcode,
        location: u32,
    },
    Int {
        op: Opcode,
        int: i32,
    },
    Field {
        op: Opcode,
        owner: u16,
        name: u16,
        descriptor: u16,
    },
    Method {
        op: Opcode,
        owner: u16,
        name: u16,
        descriptor: u16,
    },
}
impl Instruction {
    pub fn parse<'input>(input: &'input [u8]) -> IResult<'input, Self> {
        map_opt(be_u8, |op| {
            FromPrimitive::from_u8(op).map(|op| Instruction::Basic { op })
        })(input)
    }
}

#[derive(Debug)]
pub struct ConstantPool(Vec<ConstantInfo>);
impl ConstantPool {
    pub fn get(&self, index: u16) -> Option<&ConstantInfo> {
        assert!(index >= 1);

        self.0.get(index as usize - 1)
    }
}
