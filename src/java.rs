#![allow(dead_code)]

use nom::bytes::complete::{tag, take};
use nom::combinator::{map, map_opt, map_res};
use nom::error::{make_error, ErrorKind};
use nom::multi::{length_count, length_data, length_value};
use nom::number::complete::{be_u16, be_u32, be_u64, be_u8};
use nom::sequence::{pair, tuple};

use crate::consts::{self, ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use crate::java::AttributeInfo::Code;

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

type IResult<'a, O> = nom::IResult<&'a [u8], O>;

impl ClassInfo {
    pub fn parse(input: &[u8]) -> IResult<Self> {
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
    pub fn parse<'a>(input: &'a [u8], constant_pool: &ConstantPool) -> IResult<'a, Self> {
        let (input, access_flags) = map_opt(be_u16, FieldAccessFlags::from_bits)(input)?;
        let (input, name_index) = be_u16(input)?;
        let (input, descriptor_index) = be_u16(input)?;
        let (input, attribute_info) =
            length_count(be_u16, |input| AttributeInfo::parse(input, &constant_pool))(input)?;

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
    pub fn parse<'a>(input: &'a [u8], constant_pool: &ConstantPool) -> IResult<'a, Self> {
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
    pub fn parse(input: &[u8]) -> IResult<Self> {
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

impl AttributeException {
    pub fn parse(input: &[u8]) -> IResult<Self> {
        map(
            tuple((be_u16, be_u16, be_u16, be_u16)),
            |(start_pc, end_pc, handler_pc, catch_type)| AttributeException {
                start_pc,
                end_pc,
                handler_pc,
                catch_type,
            },
        )(input)
    }
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
        code: Vec<Instruction>,
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
    pub fn parse<'a>(input: &'a [u8], constant_pool: &ConstantPool) -> IResult<'a, Self> {
        let (input, info) = map_opt(be_u16, |index| constant_pool.get(index))(input)?;
        let (input, length) = be_u32(input)?;

        match info {
            ConstantInfo::UTF8 { text } => match text.as_str() {
                "ConstantValue" => map(be_u16, |constant_index| AttributeInfo::ConstantValue {
                    constant_index,
                })(input),
                "Code" => map(
                    tuple((
                        be_u16,
                        be_u16,
                        (|input| {
                            let (input, table_length) = be_u32(input)?;
                            let mut input = input;
                            let mut pos: usize = 0;
                            let mut out = Vec::new();
                            while pos < table_length as usize {
                                let (input2, value) = Instruction::parse(input)?;
                                out.push(value.0);
                                pos += value.1;
                                input = input2;
                            }
                            Ok((input, out))
                        }),
                        length_count(be_u16, |input| AttributeException::parse(input)),
                        length_count(be_u16, |input| AttributeInfo::parse(input, &constant_pool)),
                    )),
                    |(max_stack, max_locals, code, exception_table, attribute_info)| {
                        AttributeInfo::Code {
                            max_stack,
                            max_locals,
                            code,
                            exception_table,
                            attribute_info,
                        }
                    },
                )(input),
                //"StackMapTable" => todo!(),
                //"Exceptions" => todo!(),
                //"InnerClasses" => todo!(),
                //"EnclosingMethod" => todo!(),
                //"Synthetic" => todo!(),
                //"Signature" => todo!(),
                //"SourceFile" => todo!(),
                //"SourceDebugExtension" => todo!(),
                //"LineNumberTable" => todo!(),
                //"LocalVariableTable" => todo!(),
                //"LocalVariableTypeTable" => todo!(),
                //"Deprecated" => todo!(),
                //"RuntimeVisibleAnnotations" => todo!(),
                //"RuntimeInvisibleAnnotations" => todo!(),
                //"RuntimeVisibleParameterAnnotations" => todo!(),
                //"RuntimeInvisibleParameterAnnotations" => todo!(),
                //"AnnotationDefault" => todo!(),
                //"BootstrapMethods" => todo!(),
                _ => map(take(length), |_| AttributeInfo::AnnotationDefault)(input),
            },
            //discard the remaining bytes
            _ => map(take(length), |_| AttributeInfo::AnnotationDefault)(input),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Basic { op: u8 },
    Typed { op: u8, type_index: u16 },
    Var { op: u8, var: u16 },
    Jump { op: u8, location: u32 },
    Int { op: u8, int: i32 },
    Inc { op: u8, amount: u16 },
    Constant { op: u8, constant: u16 },
    Field { op: u8, field_index: u16 },
    Method { op: u8, method_index: u16 },
}

impl Instruction {
    pub fn get_op(&self) -> u8 {
        match self {
            Instruction::Basic { op } => *op,
            Instruction::Typed { op, type_index } => *op,
            Instruction::Var { op, var } => *op,
            Instruction::Jump { op, location } => *op,
            Instruction::Int { op, int } => *op,
            Instruction::Inc { op, amount } => *op,
            Instruction::Constant { op, constant } => *op,
            Instruction::Field { op, field_index } => *op,
            Instruction::Method { op, method_index } => *op,
        }
    }

    pub fn parse(input: &[u8]) -> IResult<(Self, usize)> {
        let (input, op) = be_u8(input)?;
        match op {
            consts::NOP
            | consts::ACONST_NULL
            | consts::ICONST_M1
            | consts::ICONST_0
            | consts::ICONST_1
            | consts::ICONST_2
            | consts::ICONST_3
            | consts::ICONST_4
            | consts::ICONST_5
            | consts::LCONST_0
            | consts::LCONST_1
            | consts::FCONST_0
            | consts::FCONST_1
            | consts::FCONST_2
            | consts::DCONST_0
            | consts::DCONST_1
            | consts::IALOAD
            | consts::LALOAD
            | consts::FALOAD
            | consts::DALOAD
            | consts::AALOAD
            | consts::BALOAD
            | consts::CALOAD
            | consts::SALOAD
            | consts::IASTORE
            | consts::LASTORE
            | consts::FASTORE
            | consts::DASTORE
            | consts::AASTORE
            | consts::BASTORE
            | consts::CASTORE
            | consts::SASTORE
            | consts::POP
            | consts::POP2
            | consts::DUP
            | consts::DUP_X1
            | consts::DUP_X2
            | consts::DUP2
            | consts::DUP2_X1
            | consts::DUP2_X2
            | consts::SWAP
            | consts::IADD
            | consts::LADD
            | consts::FADD
            | consts::DADD
            | consts::ISUB
            | consts::LSUB
            | consts::FSUB
            | consts::DSUB
            | consts::IMUL
            | consts::LMUL
            | consts::FMUL
            | consts::DMUL
            | consts::IDIV
            | consts::LDIV
            | consts::FDIV
            | consts::DDIV
            | consts::IREM
            | consts::LREM
            | consts::FREM
            | consts::DREM
            | consts::INEG
            | consts::LNEG
            | consts::FNEG
            | consts::DNEG
            | consts::ISHL
            | consts::LSHL
            | consts::ISHR
            | consts::LSHR
            | consts::IUSHR
            | consts::LUSHR
            | consts::IAND
            | consts::LAND
            | consts::IOR
            | consts::LOR
            | consts::IXOR
            | consts::LXOR
            | consts::I2L
            | consts::I2F
            | consts::I2D
            | consts::L2I
            | consts::L2F
            | consts::L2D
            | consts::F2I
            | consts::F2L
            | consts::F2D
            | consts::D2I
            | consts::D2L
            | consts::D2F
            | consts::I2B
            | consts::I2C
            | consts::I2S
            | consts::LCMP
            | consts::FCMPL
            | consts::FCMPG
            | consts::DCMPL
            | consts::DCMPG
            | consts::IRETURN
            | consts::LRETURN
            | consts::FRETURN
            | consts::DRETURN
            | consts::ARETURN
            | consts::RETURN
            | consts::ARRAYLENGTH
            | consts::ATHROW
            | consts::MONITORENTER
            | consts::MONITOREXIT => Ok((input, (Instruction::Basic { op }, 1))),
            consts::IFEQ
            | consts::IFNE
            | consts::IFLT
            | consts::IFGE
            | consts::IFGT
            | consts::IFLE
            | consts::IF_ICMPEQ
            | consts::IF_ICMPNE
            | consts::IF_ICMPLT
            | consts::IF_ICMPGE
            | consts::IF_ICMPGT
            | consts::IF_ICMPLE
            | consts::IF_ACMPEQ
            | consts::IF_ACMPNE
            | consts::GOTO
            | consts::JSR
            | consts::IFNULL
            | consts::IFNONNULL => map(be_u16, |val| {
                (
                    Instruction::Jump {
                        op,
                        location: val as u32,
                    },
                    3,
                )
            })(input),
            consts::ILOAD_0
            | consts::ILOAD_1
            | consts::ILOAD_2
            | consts::ILOAD_3
            | consts::LLOAD_0
            | consts::LLOAD_1
            | consts::LLOAD_2
            | consts::LLOAD_3
            | consts::FLOAD_0
            | consts::FLOAD_1
            | consts::FLOAD_2
            | consts::FLOAD_3
            | consts::DLOAD_0
            | consts::DLOAD_1
            | consts::DLOAD_2
            | consts::DLOAD_3
            | consts::ALOAD_0
            | consts::ALOAD_1
            | consts::ALOAD_2
            | consts::ALOAD_3 => {
                let opcode = op - consts::ILOAD_0;
                Ok((
                    input,
                    (
                        Instruction::Var {
                            op: consts::ILOAD + (opcode >> 2),
                            var: (opcode & 0x3) as u16,
                        },
                        1,
                    ),
                ))
            }
            consts::GOTO_W | consts::JSR_W => {
                map(be_u32, |val| (Instruction::Jump { op, location: val }, 3))(input)
            }
            consts::ISTORE_0
            | consts::ISTORE_1
            | consts::ISTORE_2
            | consts::ISTORE_3
            | consts::LSTORE_0
            | consts::LSTORE_1
            | consts::LSTORE_2
            | consts::LSTORE_3
            | consts::FSTORE_0
            | consts::FSTORE_1
            | consts::FSTORE_2
            | consts::FSTORE_3
            | consts::DSTORE_0
            | consts::DSTORE_1
            | consts::DSTORE_2
            | consts::DSTORE_3
            | consts::ASTORE_0
            | consts::ASTORE_1
            | consts::ASTORE_2
            | consts::ASTORE_3 => {
                let opcode = op - consts::ISTORE_0;
                Ok((
                    input,
                    (
                        Instruction::Var {
                            op: consts::ISTORE + (opcode >> 2),
                            var: (opcode & 0x3) as u16,
                        },
                        1,
                    ),
                ))
            }
            consts::ILOAD
            | consts::LLOAD
            | consts::FLOAD
            | consts::DLOAD
            | consts::ALOAD
            | consts::ISTORE
            | consts::LSTORE
            | consts::FSTORE
            | consts::DSTORE
            | consts::ASTORE
            | consts::BIPUSH
            | consts::NEWARRAY
            | consts::RET => map(be_u8, |val| {
                (
                    Instruction::Var {
                        op,
                        var: val as u16,
                    },
                    2,
                )
            })(input),
            consts::SIPUSH => map(be_u16, |val| (Instruction::Var { op, var: val }, 3))(input),
            consts::LDC => map(be_u8, |val| {
                (
                    Instruction::Constant {
                        op,
                        constant: val as u16,
                    },
                    2,
                )
            })(input),
            consts::LDC_W | consts::LDC2_W => map(be_u16, |val| {
                (Instruction::Constant { op, constant: val }, 3)
            })(input),

            consts::GETSTATIC | consts::PUTSTATIC | consts::GETFIELD | consts::PUTFIELD => {
                map(be_u16, |val| {
                    (
                        Instruction::Field {
                            op,
                            field_index: val,
                        },
                        3,
                    )
                })(input)
            }

            consts::INVOKEVIRTUAL | consts::INVOKESPECIAL | consts::INVOKESTATIC => {
                map(be_u16, |val| {
                    (
                        Instruction::Method {
                            op,
                            method_index: val,
                        },
                        3,
                    )
                })(input)
            }
            consts::INVOKEINTERFACE => map(pair(be_u16, be_u16), |(val, things)| {
                (
                    Instruction::Method {
                        op,
                        method_index: val,
                    },
                    3,
                )
            })(input),
            consts::NEW | consts::ANEWARRAY | consts::CHECKCAST | consts::INSTANCEOF => {
                map(be_u16, |val| {
                    (
                        Instruction::Typed {
                            op,
                            type_index: val,
                        },
                        3,
                    )
                })(input)
            }
            consts::IINC => map(be_u16, |amount| (Instruction::Inc { op, amount }, 3))(input),
            _ => panic!("Opcode {} is not supported.", op),
        }
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
