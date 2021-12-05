use std::collections::VecDeque;

use crate::consts::RETURN;
use crate::java_code::{Instruction, Op};
use crate::java_decomp::ExpressionType::{
    ArrayLengthEx, ArrayLoadEx, CommentEx, ConstantEx, ConvertEx, NumberConstant, OperatorEx,
    PoolConstantEx, ReturnEx, ReturnValueEx,
};
use crate::Code;
use crate::java::ConstantPool;

pub struct ExStack {
    stack: VecDeque<Expression>,
}

impl ExStack {
    pub fn push(&mut self, op: &Op, ex: ExpressionType) {
        self.stack.push_front(Expression { op: op.op, ex });
    }

    pub fn pull(&mut self) -> Expression {
        self.stack.pop_front().unwrap()
    }
}

pub struct ExList {
    list: Vec<Expression>,
}

impl ExList {
    pub fn add(&mut self, op: &Op, ex: ExpressionType) {
        self.list.push(Expression { op: op.op, ex });
    }
}

pub struct Expression {
    op: u8,
    ex: ExpressionType,
}

pub enum ExpressionType {
    ConstantEx {
        op: u8,
    },
    PoolConstantEx {
        pool_pos: u16,
    },
    // short is the biggest number constant. everything else is a PoolConstant
    NumberConstant {
        number: i16,
    },
    OperatorEx {
        left: Expression,
        right: Expression,
    },
    ConvertEx {
        original: Expression,
    },
    IncrementEx {
        var: u16,
        amount: u8,
    },
    LoadVarEx {
        var: u16,
    },
    StoreVarEx {
        var: u16,
        value: Expression,
    },
    ArrayLoadEx {
        array: Expression,
        index: Expression,
    },
    ArrayStoreEx {
        array: Expression,
        index: Expression,
        value: Expression,
    },
    CompareEx {
        left: Expression,
        right: Expression,
    },
    InstanceOfEx {
        value: Expression,
        pool_pos: u16,
    },
    IfEx {
        left: Expression,
        right: Expression,
    },
    If0Ex {
        value: Expression,
    },
    NewEx {
        pool_pos: u16,
    },
    NewPrimArrayEx {
        array_type: u8,
    },
    // needs more impl
    StackEx,
    MonitorEx,
    CastEx {
        value: Expression,
        pool_pos: u16,
    },
    // TODO Switch
    ReturnEx,
    ReturnValueEx {
        value: Expression,
    },
    ThrowEx {
        throwable: Expression,
    },
    GetFieldEx {
        object: Expression,
        pool_pos: u16,
    },
    PutFieldEx {
        object: Expression,
        value: Expression,
        pool_pos: u16,
    },
    GetStaticFieldEx {
        pool_pos: u16,
    },
    PutStaticFieldEx {
        value: Expression,
        pool_pos: u16,
    },
    MethodEx,
    ArrayLengthEx {
        array: Expression,
    },

    // comments for extra info and such
    CommentEx {
        comment: String,
    },
}

impl Expression {
    fn new(op: &Op, ex: ExpressionType) -> Self {
        Expression { op: op.op, ex }
    }

    pub fn create(op: &Op, ex_stack: &mut ExStack, ex_list: &mut ExList, const_pool: &ConstantPool) {
        match &op.inst {
            Instruction::Nop => ex_list.add(
                op,
                CommentEx {
                    comment: "nop".to_string(),
                },
            ),
            Instruction::Value => ex_stack.push(op, ConstantEx),
            Instruction::GetArrayLength => {
                let array = ex_stack.pull();
                ex_stack.push(op, ArrayLengthEx { array })
            }
            // TODO stack
            Instruction::Stack => {}
            Instruction::Math => {
                let value2 = ex_stack.pull();
                let value1 = ex_stack.pull();
                ex_stack.push(
                    op,
                    OperatorEx {
                        left: value1,
                        right: value2,
                    },
                )
            }
            Instruction::Conventions => {
                let original = ex_stack.pull();
                ex_stack.push(op, ConvertEx { original })
            }
            Instruction::Return => {
                if op.op == RETURN {
                    ex_list.add(op, ReturnEx)
                } else {
                    let value = ex_stack.pull();
                    ex_list.add(op, ReturnValueEx { value })
                }
            }
            // TODO Throw
            Instruction::Throw { .. } => {}
            Instruction::ConstantPool { pool: pool_pos } => ex_stack.push(
                op,
                PoolConstantEx {
                    pool_pos: pool_pos as u16,
                },
            ),
            Instruction::ConstantPoolWide { pool: pool_pos } => ex_stack.push(
                op,
                PoolConstantEx {
                    pool_pos: *pool_pos,
                },
            ),
            Instruction::PushByte { value } => ex_stack.push(
                op,
                NumberConstant {
                    number: value as i16,
                },
            ),
            Instruction::PushShort { value } => {
                ex_stack.push(op, NumberConstant { number: *value })
            }
            Instruction::Increment { var, amount } => ex_list.add(
                op,
                ExpressionType::IncrementEx {
                    var: *var,
                    amount: *amount,
                },
            ),
            Instruction::Load { var } => ex_stack.push(op, ExpressionType::LoadVarEx { var: *var }),
            Instruction::Store { var } => {
                let value = ex_stack.pull();
                ex_list.push(op, ExpressionType::StoreVarEx { var: *var, value })
            }
            Instruction::ArrayLoad => {
                let index = ex_stack.pull();
                let array = ex_stack.pull();
                ex_stack.push(op, ExpressionType::ArrayLoadEx { array, index })
            }
            Instruction::ArrayStore => {
                let value = ex_stack.pull();
                let index = ex_stack.pull();
                let array = ex_stack.pull();
                ex_stack.push(
                    op,
                    ExpressionType::ArrayStoreEx {
                        array,
                        index,
                        value,
                    },
                )
            }
            Instruction::Comparison => {
                let value2 = ex_stack.pull();
                let value1 = ex_stack.pull();
                ex_stack.push(
                    op,
                    ExpressionType::CompareEx {
                        left: value1,
                        right: value2,
                    },
                )
            }
            Instruction::Instanceof { pool_pos } => {
                let value = ex_stack.pull();
                ex_stack.push(
                    op,
                    ExpressionType::InstanceOfEx {
                        value,
                        pool_pos: *pool_pos,
                    },
                )
            }
            Instruction::Cast { pool_pos } => {
                let value = ex_stack.pull();
                ex_stack.push(
                    op,
                    ExpressionType::CastEx {
                        value,
                        pool_pos: *pool_pos,
                    },
                )
            }
            // TODO compute chunk target
            Instruction::ComparisonJump { jump } => {
                let value2 = ex_stack.pull();
                let value1 = ex_stack.pull();
                ex_list.add(
                    op,
                    ExpressionType::IfEx {
                        left: value1,
                        right: value2,
                    },
                )
            }
            // TODO compute chunk target
            Instruction::ZeroComparisonJump { jump } => {
                let value = ex_stack.pull();
                ex_list.add(op, ExpressionType::If0Ex { value })
            }
            // TODO compute chunk target
            Instruction::Jump { .. } => {}
            // TODO switch
            Instruction::SwitchJump { .. } => {}
            Instruction::New { pool_pos } => ex_stack.push(
                op,
                ExpressionType::NewEx {
                    pool_pos: *pool_pos,
                },
            ),
            Instruction::NewPrimitiveArray { array_type } => ex_stack.push(
                op,
                ExpressionType::NewPrimArrayEx {
                    array_type: *array_type,
                },
            ),
            Instruction::GetField { pool_pos } => {
                let object = ex_stack.pull();
                ex_stack.push(
                    op,
                    ExpressionType::GetFieldEx {
                        object,
                        pool_pos: *pool_pos,
                    },
                )
            }
            Instruction::GetStaticField { pool_pos } => ex_stack.push(
                op,
                ExpressionType::GetStaticFieldEx {
                    pool_pos: *pool_pos,
                },
            ),
            Instruction::PutField { pool_pos } => {
                let value = ex_stack.pull();
                let object = ex_stack.pull();
                ex_list.add(
                    op,
                    ExpressionType::PutFieldEx {
                        object,
                        value,
                        pool_pos: *pool_pos,
                    },
                )
            }
            Instruction::PutStaticField { pool_pos } => {
                let value = ex_stack.pull();
                ex_list.add(
                    op,
                    ExpressionType::PutStaticFieldEx {
                        value,
                        pool_pos: *pool_pos,
                    },
                )
            }
            Instruction::InvokeMethod { pool_pos } => {
                const_pool.get_method_descriptor(pool_pos)
            }
            Instruction::Monitor => {}
        }
    }
}

pub fn compute(code: &Code) {
    let chunks = &code.code_chunks;

    // The stack holds expressions basically like the jvm executes values.
    // Every expression is here until it finds a home at another expression.
    // which later goes to the ex_list.
    let mut ex_stack: VecDeque<ExpressionType> = VecDeque::new();

    // Holds all of the expressions which are final and will be included in the final print.
    let mut ex_list: Vec<ExpressionType> = Vec::new();

    // Iterate through all of the chunks
    for chunk in chunks {
        for i in chunk.start..chunk.stop {}
    }
}
