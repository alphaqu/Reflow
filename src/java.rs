use bytes::Buf;

use crate::java::Instruction::BASIC;
use crate::opcodes;

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
pub fn get_class_info(reader: &mut &[u8]) -> ClassInfo {
    let magic = reader.get_u32();
    let minor_version = reader.get_u16();
    let major_version = reader.get_u16();

    let constant_pool_size = (reader.get_u16() - 1) as usize;
    let mut constant_pool: Vec<ConstantInfo> = Vec::with_capacity(constant_pool_size);
    for i in 0..constant_pool_size {
        constant_pool.insert(i, get_constant_info(i, reader));
    }

    let access_flags = reader.get_u16();
    let this_class = reader.get_u16();
    let super_class = reader.get_u16();

    let interfaces_size = reader.get_u16() as usize;
    let mut interfaces: Vec<u16> = Vec::with_capacity(interfaces_size);
    for i in 0..interfaces_size {
        interfaces.insert(i, reader.get_u16());
    }

    let fields_size = reader.get_u16() as usize;
    let mut fields: Vec<FieldInfo> = Vec::with_capacity(fields_size);
    for i in 0..fields_size {
        fields.insert(i, get_field(reader, &constant_pool));
    }

    let methods_size = reader.get_u16() as usize;
    let mut methods: Vec<MethodInfo> = Vec::with_capacity(methods_size);
    for i in 0..methods_size {
        methods.insert(i, get_method(reader, &constant_pool));
    }

    let attributes_size = reader.get_u16() as usize;
    let mut attributes: Vec<AttributeInfo> = Vec::with_capacity(attributes_size);
    for i in 0..attributes_size {
        attributes.insert(i, get_attribute_info(reader, &constant_pool));
    }

    ClassInfo {
        magic,
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
    }
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

pub fn get_field(reader: &mut &[u8], constant_pool: &Vec<ConstantInfo>) -> FieldInfo {
    let access_flags = reader.get_u16();
    let name_index = reader.get_u16();
    let descriptor_index = reader.get_u16();

    let attribute_info_size = reader.get_u16();
    let mut attribute_info: Vec<AttributeInfo> = Vec::with_capacity(attribute_info_size as usize);
    for _i in 0..attribute_info_size {
        attribute_info.push(get_attribute_info(reader, constant_pool));
    }
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
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attribute_info: Vec<AttributeInfo>,
}

pub fn get_method(reader: &mut &[u8], constant_pool: &Vec<ConstantInfo>) -> MethodInfo {
    let access_flags = reader.get_u16();
    let name_index = reader.get_u16();
    let descriptor_index = reader.get_u16();

    let attribute_info_size = reader.get_u16();
    let mut attribute_info: Vec<AttributeInfo> = Vec::with_capacity(attribute_info_size as usize);
    for _i in 0..attribute_info_size {
        attribute_info.push(get_attribute_info(reader, constant_pool));
    }
    MethodInfo {
        access_flags,
        name_index,
        descriptor_index,
        attribute_info,
    }
}

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
        text: str,
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

pub fn get_constant_info(i: usize, reader: &mut &[u8]) -> ConstantInfo {
    return match reader.get_u8() {
        7 => ConstantInfo::Class {
            name_index: reader.get_u16(),
        },
        9 => ConstantInfo::Field {
            class_index: reader.get_u16(),
            name_and_type_index: reader.get_u16(),
        },
        10 => ConstantInfo::Method {
            class_index: reader.get_u16(),
            name_and_type_index: reader.get_u16(),
        },
        11 => ConstantInfo::Interface {
            class_index: reader.get_u16(),
            name_and_type_index: reader.get_u16(),
        },
        8 => ConstantInfo::String {
            string_index: reader.get_u16(),
        },
        3 => ConstantInfo::Integer {
            bytes: reader.get_u32(),
        },
        4 => ConstantInfo::Float {
            bytes: reader.get_u32(),
        },
        5 => ConstantInfo::Long {
            bytes: reader.get_u64(),
        },
        6 => ConstantInfo::Double {
            bytes: reader.get_u64(),
        },
        12 => ConstantInfo::NameAndType {
            name_index: reader.get_u16(),
            descriptor_index: reader.get_u16(),
        },
        1 => {
            let bytes_size = reader.get_u16();
            let mut bytes: Vec<u8> = Vec::with_capacity(bytes_size as usize);
            for _i in 0..bytes_size {
                bytes.push(reader.get_u8())
            }
            let string = String::from_utf8(bytes).expect("Failed to create string.");
            println!("{}. {}", i, string);
            ConstantInfo::UTF8 { text: *string }
        }
        15 => ConstantInfo::MethodHandle {
            reference_kind: reader.get_u8(),
            reference_index: reader.get_u16(),
        },
        16 => ConstantInfo::MethodType {
            descriptor_index: reader.get_u16(),
        },
        18 => ConstantInfo::InvokeDynamic {
            bootstrap_method_attr_index: reader.get_u16(),
            name_and_type_index: reader.get_u16(),
        },
        _ => panic!("crash and burn"),
    };
}

pub struct AttributeException {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

pub struct AttributeClass {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: u16,
}

pub struct AttributeLineNumber {
    start_pc: u16,
    line_number: u16,
}

pub struct AttributeLocalVariable {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

pub struct AttributeLocalVariableType {
    start_pc: u16,
    length: u16,
    name_index: u16,
    signature_index: u16,
    index: u16,
}

pub struct AttributeBootstrapMethod {
    bootstrap_method_ref: u16,
    bootstrap_arguments: Vec<u16>,
}

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

pub fn get_attribute_info(reader: &mut &[u8], constant_pool: &Vec<ConstantInfo>) -> AttributeInfo {
    let attribute_name_index = reader.get_u16();
    let x = get_constant(attribute_name_index, constant_pool);
    let length = reader.get_u32(); // length
    match x {
        ConstantInfo::UTF8 { text } => {
            match text {
                "ConstantValue" => AttributeInfo::ConstantValue {
                    constant_index: reader.get_u16(),
                },
                "Code" => {
                    let max_stack = reader.get_u16();
                    let max_locals = reader.get_u16();
                    let code_length = reader.get_u32();
                    AttributeInfo::Code {
                        max_stack,
                        max_locals,
                        code: vec![],
                        exception_table: vec![],
                        attribute_info: vec![],
                    }
                }
                "StackMapTable" => {}
                "Exceptions" => {}
                "InnerClasses" => {}
                "EnclosingMethod" => {}
                "Synthetic" => {}
                "Signature" => {}
                "SourceFile" => {}
                "SourceDebugExtension" => {}
                "LineNumberTable" => {}
                "LocalVariableTable" => {}
                "LocalVariableTypeTable" => {}
                "Deprecated" => {}
                "RuntimeVisibleAnnotations" => {}
                "RuntimeInvisibleAnnotations" => {}
                "RuntimeVisibleParameterAnnotations" => {}
                "RuntimeInvisibleParameterAnnotations" => {}
                "AnnotationDefault" => {}
                "BootstrapMethods" => {}
                &_ => {}
            }
            AttributeInfo::AnnotationDefault
        }
        _ => {
            for i in 0..length {
                reader.get_u8();
            }
            AttributeInfo::AnnotationDefault
        }
    }
}

pub enum Instruction {
    BASIC {
        op: u8,
    },
    TYPED {
        op: u8,
        type_index: u16,
    },
    VAR {
        op: u8,
        var: u32,
    },
    JUMP {
        op: u8,
        location: u32,
    },
    INT {
        op: u8,
        int: i32,
    },
    FIELD {
        op: u8,
        owner: u16,
        name: u16,
        descriptor: u16,
    },
    METHOD {
        op: u8,
        owner: u16,
        name: u16,
        descriptor: u16,
    },
}

pub fn get_instruction(reader: &mut &[u8]) -> Instruction {
    let op = reader.get_u8();
    match op {
        opcodes::NOP
        | opcodes::ACONST_NULL
        | opcodes::ICONST_M1
        | opcodes::ICONST_0
        | opcodes::ICONST_1
        | opcodes::ICONST_2
        | opcodes::ICONST_3
        | opcodes::ICONST_4
        | opcodes::ICONST_5
        | opcodes::LCONST_0
        | opcodes::LCONST_1
        | opcodes::FCONST_0
        | opcodes::FCONST_1
        | opcodes::FCONST_2
        | opcodes::DCONST_0
        | opcodes::DCONST_1
        | opcodes::IALOAD
        | opcodes::LALOAD
        | opcodes::FALOAD
        | opcodes::DALOAD
        | opcodes::AALOAD
        | opcodes::BALOAD
        | opcodes::CALOAD
        | opcodes::SALOAD
        | opcodes::IASTORE
        | opcodes::LASTORE
        | opcodes::FASTORE
        | opcodes::DASTORE
        | opcodes::AASTORE
        | opcodes::BASTORE
        | opcodes::CASTORE
        | opcodes::SASTORE
        | opcodes::POP
        | opcodes::POP2
        | opcodes::DUP
        | opcodes::DUP_X1
        | opcodes::DUP_X2
        | opcodes::DUP2
        | opcodes::DUP2_X1
        | opcodes::DUP2_X2
        | opcodes::SWAP
        | opcodes::IADD
        | opcodes::LADD
        | opcodes::FADD
        | opcodes::DADD
        | opcodes::ISUB
        | opcodes::LSUB
        | opcodes::FSUB
        | opcodes::DSUB
        | opcodes::IMUL
        | opcodes::LMUL
        | opcodes::FMUL
        | opcodes::DMUL
        | opcodes::IDIV
        | opcodes::LDIV
        | opcodes::FDIV
        | opcodes::DDIV
        | opcodes::IREM
        | opcodes::LREM
        | opcodes::FREM
        | opcodes::DREM
        | opcodes::INEG
        | opcodes::LNEG
        | opcodes::FNEG
        | opcodes::DNEG
        | opcodes::ISHL
        | opcodes::LSHL
        | opcodes::ISHR
        | opcodes::LSHR
        | opcodes::IUSHR
        | opcodes::LUSHR
        | opcodes::IAND
        | opcodes::LAND
        | opcodes::IOR
        | opcodes::LOR
        | opcodes::IXOR
        | opcodes::LXOR
        | opcodes::I2L
        | opcodes::I2F
        | opcodes::I2D
        | opcodes::L2I
        | opcodes::L2F
        | opcodes::L2D
        | opcodes::F2I
        | opcodes::F2L
        | opcodes::F2D
        | opcodes::D2I
        | opcodes::D2L
        | opcodes::D2F
        | opcodes::I2B
        | opcodes::I2C
        | opcodes::I2S
        | opcodes::LCMP
        | opcodes::FCMPL
        | opcodes::FCMPG
        | opcodes::DCMPL
        | opcodes::DCMPG
        | opcodes::IRETURN
        | opcodes::LRETURN
        | opcodes::FRETURN
        | opcodes::DRETURN
        | opcodes::ARETURN
        | opcodes::RETURN
        | opcodes::ARRAYLENGTH
        | opcodes::ATHROW
        | opcodes::MONITORENTER
        | opcodes::MONITOREXIT => Instruction::BASIC { op },
        _ => {}
    }
}

pub fn get_constant(index: u16, constant_pool: &Vec<ConstantInfo>) -> &ConstantInfo {
    constant_pool
        .get(attribute_name_index as usize - 1)
        .expect("Could not get constant. at " + index)
}
