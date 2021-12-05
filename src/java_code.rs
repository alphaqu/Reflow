use std::collections::HashSet;
use std::mem;

use nom::combinator::map;
use nom::error::{ErrorKind, make_error};
use nom::multi::length_count;
use nom::number::complete::be_i32;
use nom::number::streaming::{be_i16, be_i8, be_u16, be_u32, be_u8};
use nom::sequence::pair;

use consts::*;

use crate::{AttributeInfo, consts, java_code};
use crate::java::{AttributeException, ConstantPool, IResult};

pub struct Op {
    pub op: u8,
    pub inst: Instruction,
}

impl Op {
    pub fn parse(input: &[u8]) -> IResult<(Self, u8)> {
        let (input, op) = be_u8(input)?;
        let (input, (inst, length, op)) = Instruction::parse(input, op)?;
        Ok((input, (Op { op, inst }, length + 1))) // instructionType length and op
    }

    pub fn print(&self) -> String {
        match &self.inst {
            Instruction::ComparisonJump { jump } => { format!("{}: {}", consts::print_op(&self.op), jump.get_pos()) }
            Instruction::SwitchJump { jumps } => { format!("{}: ", consts::print_op(&self.op)) }
            Instruction::Jump { jump } => { format!("{}: {}", consts::print_op(&self.op), jump.get_pos()) }
            _ => { format!("{}: ", consts::print_op(&self.op)) }
        }
    }
}

pub enum Instruction {
    // nop
    Nop,
    // aconst_null,
    // iconst_m1,
    // iconst_0, iconst_1, iconst_2, iconst_3, iconst_4, iconst_5,
    // lconst_0, lconst_1,
    // fconst_0, fconst_1, fconst_2,
    // dconst_0, dconst_1,
    Value,
    // arraylength
    GetArrayLength,
    // pop, pop2, dup, dup_x1, dup_x2, dup2, dup2_x1, dup2_x2, swap
    Stack,
    // iadd, ladd, fadd, dadd,
    // isub, lsub, fsub, dsub,
    // imul, lmul, fmul, dmul,
    // idiv, ldiv, fdiv, ddiv,
    // irem, lrem, frem, drem,
    // ineg, lneg, fneg, dneg,
    // ishl, lshl, ishr, lshr,
    // iushr, lushr,
    // iand, land,
    // ior, lor,
    // ixor, lxor,
    Math,
    // i2l, i2f, i2d, l2i, l2f, l2d, f2i, f2l, f2d, d2i, d2l, d2f, i2b, i2c, i2s
    Conventions,
    // ireturn, lreturn, freturn, dreturn, areturn, return
    Return,
    // athrow
    Throw { pool_pos: u16 },
    // ldc
    ConstantPool { pool: u8 },
    // ldc_w, ldc2_w
    ConstantPoolWide { pool: u16 },
    // bipush
    PushByte { value: i8 },
    // sipush
    PushShort { value: i16 },
    // iinc
    Increment { var: u16, amount: u8 },
    // iload, lload, fload, dload, aload,
    // iload_0, iload_1, iload_2, iload_3,
    // lload_0, lload_1, lload_2, lload_3,
    // fload_0, fload_1, fload_2, fload_3,
    // dload_0, dload_1, dload_2, dload_3,
    // aload_0, aload_1, aload_2, aload_3,
    Load { var: u16 },
    // iaload, laload, faload, daload, aaload, baload, caload, saload
    ArrayLoad,
    // istore, lstore, fstore, dstore, astore,
    // istore_0, istore_1, istore_2, istore_3,
    // lstore_0, lstore_1, lstore_2, lstore_3,
    // fstore_0, fstore_1, fstore_2, fstore_3,
    // dstore_0, dstore_1, dstore_2, dstore_3,
    // astore_0, astore_1, astore_2, astore_3,
    Store { var: u16 },
    // iastore, lastore, fastore, dastore, aastore, bastore, castore, sastore
    ArrayStore,
    // lcmp, fcmpl, fcmpg, dcmpl, dcmpg
    Comparison,
    // checkcast
    Cast { pool_pos: u16 },
    // instanceof
    Instanceof { pool_pos: u16 },
    // if_icmpeq, if_icmpne, if_icmplt, if_icmpge, if_icmpgt, if_icmple, if_acmpeq, if_acmpne
    ComparisonJump { jump: JumpValue },
    // ifeq, ifne, iflt, ifge, ifgt, ifle,
    ZeroComparisonJump { jump: JumpValue },
    // tableswitch lookupswitch
    SwitchJump { jumps: Vec<JumpValue> },
    // goto, jsr, goto_w, jsr_w
    Jump { jump: JumpValue },
    // new, anewarray
    New { pool_pos: u16 },
    // newarray
    NewPrimitiveArray { array_type: u8 },
    // getfield
    GetField { pool_pos: u16 },
    // getstatic
    GetStaticField { pool_pos: u16 },
    // putfield
    PutField { pool_pos: u16 },
    // putstatic
    PutStaticField { pool_pos: u16 },
    // invokevirtual, invokespecial, invokestatic, invokeinterface, invokedynamic
    InvokeMethod { pool_pos: u16 },
    // monitorenter, monitorexit
    Monitor,
}

pub struct JumpValue {
    union: JumpUnion,
}

impl JumpValue {
    pub fn new(offset: i32) -> JumpValue {
        JumpValue {
            union: JumpUnion {
                jump_offset: offset,
            },
        }
    }

    pub fn get_pos(&self) -> u32 {
        unsafe {
            self.union.jump_pos
        }
    }
}

pub union JumpUnion {
    jump_pos: u32,
    jump_offset: i32,
}

impl JumpUnion {
    pub fn apply(&mut self, op_byte: u32, op_byte_to_op: &Vec<u32>) {
        unsafe {
            self.jump_pos = *op_byte_to_op.get(((op_byte as i64) + (self.jump_offset as i64)) as usize).unwrap();
        }
    }
}

impl Instruction {
    pub fn parse(input: &[u8], op: u8) -> IResult<(Self, u8, u8)> {
        match op {
            // nop
            NOP => Ok((input, (Instruction::Value, 0, op))),
            // Constant
            ACONST_NULL | ICONST_M1 | ICONST_0 | ICONST_1 | ICONST_2 | ICONST_3
            | ICONST_4 | ICONST_5 | LCONST_0 | LCONST_1 | FCONST_0 | FCONST_1 | FCONST_2
            | DCONST_0 | DCONST_1 => Ok((input, (Instruction::Value, 0, op))),
            ARRAYLENGTH => Ok((input, (Instruction::Value, 0, op))),
            // Stack
            POP | POP2 | DUP | DUP_X1 | DUP_X2 | DUP2 | DUP2_X1 | DUP2_X2 | SWAP => {
                Ok((input, (Instruction::Stack, 0, op)))
            }
            // Math
            IADD | LADD | FADD | DADD | ISUB | LSUB | FSUB | DSUB | IMUL | LMUL | FMUL | DMUL
            | IDIV | LDIV | FDIV | DDIV | IREM | LREM | FREM | DREM | INEG | LNEG | FNEG | DNEG
            | ISHL | LSHL | ISHR | LSHR | IUSHR | LUSHR | IAND | LAND | IOR | LOR | IXOR | LXOR => {
                Ok((input, (Instruction::Math, 0, op)))
            }
            // Conversions
            I2L | I2F | I2D | L2I | L2F | L2D | F2I | F2L | F2D | D2I | D2L | D2F | I2B | I2C
            | I2S => Ok((input, (Instruction::Conventions, 0, op))),
            // Return
            IRETURN | LRETURN | FRETURN | DRETURN | ARETURN | RETURN => {
                Ok((input, (Instruction::Return, 0, op)))
            }
            ATHROW => map(be_u16, |pool_pos| (Instruction::Throw { pool_pos }, 2, op))(input),
            // Constant Pool related
            LDC => map(be_u8, |pool| (Instruction::ConstantPool { pool }, 1, op))(input),
            LDC_W | LDC2_W => map(be_u16, |pool| {
                (Instruction::ConstantPoolWide { pool }, 2, op)
            })(input),
            // Push
            BIPUSH => map(be_i8, |value| (Instruction::PushByte { value }, 1, op))(input),
            SIPUSH => map(be_i16, |value| (Instruction::PushShort { value }, 2, op))(input),
            // Increment
            // TODO wide instruction
            IINC => map(pair(be_u8, be_u8), |(var, amount)| {
                (
                    Instruction::Increment {
                        var: var as u16,
                        amount,
                    },
                    2,
                    op,
                )
            })(input),
            // Load
            // TODO wide instruction
            ILOAD | LLOAD | FLOAD | DLOAD | ALOAD => {
                map(be_u8, |var| (Instruction::Load { var: var as u16 }, 1, op))(input)
            }
            IALOAD | LALOAD | FALOAD | DALOAD | AALOAD | BALOAD | CALOAD | SALOAD => {
                Ok((input, (Instruction::ArrayLoad, 0, op)))
            }
            // TODO op instruction should be LOAD not LOAD_<x>
            ILOAD_0 | ILOAD_1 | ILOAD_2 | ILOAD_3 | LLOAD_0 | LLOAD_1 | LLOAD_2 | LLOAD_3
            | FLOAD_0 | FLOAD_1 | FLOAD_2 | FLOAD_3 | DLOAD_0 | DLOAD_1 | DLOAD_2 | DLOAD_3
            | ALOAD_0 | ALOAD_1 | ALOAD_2 | ALOAD_3 => Ok((input, {
                let opcode = op - ILOAD_0;
                (
                    Instruction::Load {
                        var: (opcode & 0x3) as u16,
                    },
                    0,
                    ILOAD + (opcode >> 2),
                )
            })),
            // Store
            // TODO wide instruction
            ISTORE | LSTORE | FSTORE | DSTORE | ASTORE => {
                map(be_u8, |var| (Instruction::Store { var: var as u16 }, 1, op))(input)
            }
            IASTORE | LASTORE | FASTORE | DASTORE | AASTORE | BASTORE | CASTORE | SASTORE => {
                Ok((input, (Instruction::ArrayStore, 0, op)))
            }
            // TODO op instruction should be STORE not STORE_<x>
            ISTORE_0 | ISTORE_1 | ISTORE_2 | ISTORE_3 | LSTORE_0 | LSTORE_1 | LSTORE_2
            | LSTORE_3 | FSTORE_0 | FSTORE_1 | FSTORE_2 | FSTORE_3 | DSTORE_0 | DSTORE_1
            | DSTORE_2 | DSTORE_3 | ASTORE_0 | ASTORE_1 | ASTORE_2 | ASTORE_3 => Ok((input, {
                let opcode = op - ISTORE_0;
                (
                    Instruction::Store {
                        var: (opcode & 0x3) as u16,
                    },
                    0,
                    ISTORE + (opcode >> 2),
                )
            })),
            // Comparisons
            LCMP | FCMPL | FCMPG | DCMPL | DCMPG => Ok((input, (Instruction::Comparison, 0, op))),
            CHECKCAST => map(be_u16, |pool_pos| {
                (Instruction::Cast { pool_pos }, 2, op)
            })(input),
            INSTANCEOF => map(be_u16, |pool_pos| {
                (Instruction::Instanceof { pool_pos }, 2, op)
            })(input),
            // Jumps
            IF_ICMPEQ | IF_ICMPNE | IF_ICMPLT | IF_ICMPGE | IF_ICMPGT | IF_ICMPLE | IF_ACMPEQ
            | IF_ACMPNE => map(be_i16, |jump_offset| {
                (
                    Instruction::ComparisonJump { jump: JumpValue::new(jump_offset as i32) },
                    2,
                    op,
                )
            })(input),

            IFEQ | IFNE | IFLT | IFGE | IFGT | IFLE => map(be_i16, |jump_offset| {
                (
                    Instruction::ZeroComparisonJump { jump: JumpValue::new(jump_offset as i32) },
                    2,
                    op,
                )
            })(input),
            // TODO Switch

            // Jump
            GOTO | JSR => map(be_i16, |jump_offset| {
                (
                    Instruction::Jump { jump: JumpValue::new(jump_offset as i32) },
                    2,
                    op,
                )
            })(input),
            GOTO_W | JSR_W => map(be_i32, |jump_offset| {
                (
                    Instruction::Jump { jump: JumpValue::new(jump_offset) },
                    4,
                    op,
                )
            })(input),
            // New
            NEW | ANEWARRAY => {
                map(be_u16, |pool_pos| (Instruction::New { pool_pos }, 2, op))(input)
            }
            NEWARRAY => map(be_u8, |array_type| {
                (Instruction::NewPrimitiveArray { array_type }, 1, op)
            })(input),
            // Get
            GETFIELD => map(be_u16, |pool_pos| (Instruction::GetField { pool_pos }, 2, op))(input),
            GETSTATIC => map(be_u16, |pool_pos| (Instruction::GetStaticField { pool_pos }, 2, op))(input),
            // Put
            PUTFIELD => map(be_u16, |pool_pos| (Instruction::PutField { pool_pos }, 2, op))(input),
            PUTSTATIC => map(be_u16, |pool_pos| (Instruction::PutStaticField { pool_pos }, 2, op))(input),
            // Invoke
            INVOKEVIRTUAL | INVOKESPECIAL | INVOKESTATIC | INVOKEINTERFACE | INVOKEDYNAMIC => {
                map(be_u16, |pool_pos| (Instruction::InvokeMethod { pool_pos }, 2, op))(input)
            }
            MONITORENTER | MONITOREXIT => Ok((input, (Instruction::Monitor, 0, op))),
            _ => Err(nom::Err::Error(make_error(input, ErrorKind::Fail))),
        }
    }
}

pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<Op>,
    pub code_chunks: Vec<CodeChunk>,
    pub exception_table: Vec<AttributeException>,
    pub attribute_info: Vec<AttributeInfo>,
}

pub struct CodeChunk {
    pub start: u32,
    pub stop: u32,
    pub source: Vec<u32>,
    pub target: CodeChunkTarget,
}

#[derive(Debug)]
pub enum CodeChunkTarget {
    Basic,
    Return,
    Throw,
    Condition { true_chunk_pos: u32 },
    Goto { chunk_pos: u32 },
    Switch { targets: Vec<u32> },
}

impl Code {
    pub fn add_source(code: &mut Vec<Op>, from: u32, to: u32) {
        let option: &mut Op = code.get_mut(to as usize).unwrap();
        option.source.push(from);
    }

    pub fn parse<'a>(input: &'a [u8], constant_pool: &ConstantPool) -> IResult<'a, Self> {
        println!("Code");
        let (input, max_stack) = be_u16(input)?;
        let (input, max_locals) = be_u16(input)?;
        let (input, code_length) = be_u32(input)?;
        let mut input = input;

        let mut op_byte_to_op: Vec<u32> = Vec::with_capacity(code_length as usize);
        let mut op_byte_ops: Vec<(u32, Op)> = Vec::new(); //


        let mut op_byte_pos: usize = 0;
        let mut op_pos: u32 = 0;

        // Read code and create opbyte to op vec
        while op_byte_pos < code_length as usize {
            let (input2, (op, op_byte_length)) = Op::parse(input)?;
            op_byte_ops.push((op_byte_pos as u32, op));
            for _i in 0..op_byte_length {
                op_byte_to_op.push(op_pos);
            }
            input = input2;
            op_byte_pos += op_byte_length as usize;
            op_pos += 1;
        }

        // Apply all jumps, as jumps are relative to byte location not op location,
        // This also adds stuff to the split vec which is all of the spots which it should split the code on.
        let mut splits: Vec<u32> = Vec::new();
        let mut code: Vec<Op> = Vec::with_capacity(op_byte_ops.len());
        op_pos = 0;
        for (op_byte, mut op) in op_byte_ops {
            match &mut op.inst {
                Instruction::ComparisonJump { jump } => {
                    jump.union.apply(op_byte, &op_byte_to_op);
                    let next_op = op_pos + 1;
                    let jump_op = jump.get_pos();
                    splits.push(next_op);
                    splits.push(jump_op);
                }
                Instruction::ZeroComparisonJump { jump } => {
                    jump.union.apply(op_byte, &op_byte_to_op);
                    let next_op = op_pos + 1;
                    let jump_op = jump.get_pos();
                    splits.push(next_op);
                    splits.push(jump_op);
                }
                Instruction::Jump { jump } => {
                    jump.union.apply(op_byte, &op_byte_to_op);
                    let next_op = op_pos + 1;
                    let jump_op = jump.get_pos();
                    splits.push(next_op);
                    splits.push(jump_op);
                }
                Instruction::Return => {
                    let next_op = op_pos + 1;
                    splits.push(next_op);
                }
                _ => {}
            };

            code.insert(op_pos as usize, op);
            op_pos += 1;
        }

        // sorts and deduplicate all of the split locations so we don't split twice as that might lead to issues
        splits.sort();
        splits.dedup();
        let mut code_chunks: Vec<CodeChunk> = Vec::with_capacity(splits.len());

        // op_byte_to_op now becomes op_to_chunk to save memory and computation
        let mut op_to_chunk = op_byte_to_op;

        // create chunks and fill op_to_chunk
        let mut last_split = 0;
        for i in 0..splits.len() {
            let split = splits[i];
            let chunk = CodeChunk {
                start: last_split,
                stop: split,
                source: Vec::new(),
                target: CodeChunkTarget::Basic,
            };
            code_chunks.push(chunk);
            op_to_chunk.insert(split as usize, i as u32);
            last_split = split as u32;
        }

        // calculate targets
        for chunk in 0..code_chunks.len() {
            let i = (&(code_chunks[chunk].stop - 1));

            match &code[*i as usize].inst {
                Instruction::ComparisonJump { jump } => {
                    let jump_chunk_pos = op_to_chunk[jump.get_pos() as usize];
                    &mut code_chunks[jump_chunk_pos as usize].source.push(chunk as u32);
                    &mut code_chunks[(chunk + 1) as usize].source.push(chunk as u32);
                    code_chunks[chunk].target = CodeChunkTarget::Condition { true_chunk_pos: jump_chunk_pos };
                }
                Instruction::ZeroComparisonJump { jump } => {
                    let jump_chunk_pos = op_to_chunk[jump.get_pos() as usize];
                    &mut code_chunks[jump_chunk_pos as usize].source.push(chunk as u32);
                    &mut code_chunks[(chunk + 1) as usize].source.push(chunk as u32);
                    code_chunks[chunk].target = CodeChunkTarget::Condition { true_chunk_pos: jump_chunk_pos };
                }
                // TODO Switch
                Instruction::Jump { jump } => {
                    let jump_chunk_pos = op_to_chunk[jump.get_pos() as usize];
                    &mut code_chunks[jump_chunk_pos as usize].source.push(chunk as u32);
                    code_chunks[chunk].target = CodeChunkTarget::Goto {
                        chunk_pos: jump_chunk_pos,
                    };
                }
                Instruction::Return => {
                    code_chunks[chunk].target = CodeChunkTarget::Return;
                }
                Instruction::Throw { .. } => {
                    code_chunks[chunk].target = CodeChunkTarget::Throw;
                }
                _ => {
                    &mut code_chunks[(chunk + 1) as usize].source.push(chunk as u32);
                    code_chunks[chunk].target = CodeChunkTarget::Basic;
                }
            };
        }
        let (input, exception_table) = length_count(be_u16, |input| AttributeException::parse(input))(input)?;
        let (input, attribute_info) = length_count(be_u16, |input| AttributeInfo::parse(input, &constant_pool))(input)?;
        Ok((
            input,
            Code {
                max_stack,
                max_locals,
                code,
                code_chunks,
                exception_table,
                attribute_info,
            },
        ))
    }
}
