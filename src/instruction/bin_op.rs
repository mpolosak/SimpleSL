mod assign;
mod bitwise;
mod filter;
mod logic;
mod map;
mod math;
mod shift;
use super::{
    at, local_variable::LocalVariables, reduce::Reduce, return_type::return_type_bool, Exec,
    ExecResult, InstructionWithStr, Recreate,
};
use crate as simplesl;
use crate::op::BinOperator;
use crate::{
    instruction::Instruction,
    variable::{ReturnType, Type},
    Error, ExecError, Interpreter,
};
pub use bitwise::{bitwise_and, bitwise_or, xor};
use lazy_static::lazy_static;
pub use logic::{and, or};
pub use math::{add, multiply, pow, subtract};
use math::{divide, greater, greater_equal, lower, lower_equal, modulo};
use pest::iterators::Pair;
use shift::{lshift, rshift};
use simplesl_macros::var_type;
use simplesl_parser::{unexpected, Rule};

lazy_static! {
    pub static ref ACCEPTED_INT_TYPE: Type = var_type!((int, int | [int]) | ([int], int));
}

pub fn can_be_used_int(lhs: Type, rhs: Type) -> bool {
    var_type!((lhs, rhs)).matches(&ACCEPTED_INT_TYPE)
}

lazy_static! {
    pub static ref ACCEPTED_NUM_TYPE: Type =
        var_type!((int | [int], int) | (int, [int]) | (float | [float], float) | (float, [float]));
}

pub fn can_be_used_num(lhs: Type, rhs: Type) -> bool {
    var_type!((lhs, rhs)).matches(&ACCEPTED_NUM_TYPE)
}

#[derive(Debug)]
pub struct BinOperation {
    pub lhs: Instruction,
    pub rhs: Instruction,
    pub op: BinOperator,
}

impl Exec for BinOperation {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let lhs = interpreter.exec(&self.lhs)?;
        if let BinOperator::And = self.op {
            return and::exec(lhs, &self.rhs, interpreter);
        }
        if let BinOperator::Or = self.op {
            return or::exec(lhs, &self.rhs, interpreter);
        }
        let rhs = interpreter.exec(&self.rhs)?;
        Ok(match self.op {
            BinOperator::Add => add::exec(lhs, rhs),
            BinOperator::Subtract => subtract::exec(lhs, rhs),
            BinOperator::Multiply => multiply::exec(lhs, rhs),
            BinOperator::Divide => divide::exec(lhs, rhs)?,
            BinOperator::Modulo => modulo::exec(lhs, rhs)?,
            BinOperator::Pow => pow::exec(lhs, rhs)?,
            BinOperator::Equal => equal::exec(lhs, rhs),
            BinOperator::NotEqual => not_equal::exec(lhs, rhs),
            BinOperator::Greater => greater::exec(lhs, rhs),
            BinOperator::GreaterOrEqual => greater_equal::exec(lhs, rhs),
            BinOperator::Lower => lower::exec(lhs, rhs),
            BinOperator::LowerOrEqual => lower_equal::exec(lhs, rhs),
            BinOperator::BitwiseAnd => bitwise_and::exec(lhs, rhs),
            BinOperator::BitwiseOr => bitwise_or::exec(lhs, rhs),
            BinOperator::Xor => xor::exec(lhs, rhs),
            BinOperator::LShift => lshift::exec(lhs, rhs)?,
            BinOperator::RShift => rshift::exec(lhs, rhs)?,
            BinOperator::Filter => filter::exec(lhs, rhs)?,
            BinOperator::Map => map::exec(lhs, rhs)?,
            BinOperator::At => at::exec(lhs, rhs)?,
            BinOperator::Assign => assign::exec(lhs, rhs, |_, b| b),
            BinOperator::AssignAdd => assign::exec(lhs, rhs, add::exec),
            BinOperator::AssignSubtract => assign::exec(lhs, rhs, subtract::exec),
            BinOperator::AssignMultiply => assign::exec(lhs, rhs, multiply::exec),
            BinOperator::AssignDivide => assign::try_exec(lhs, rhs, divide::exec)?,
            BinOperator::AssignModulo => assign::try_exec(lhs, rhs, modulo::exec)?,
            BinOperator::AssignLShift => assign::try_exec(lhs, rhs, lshift::exec)?,
            BinOperator::AssignRShift => assign::try_exec(lhs, rhs, rshift::exec)?,
            BinOperator::AssignBitwiseAnd => assign::exec(lhs, rhs, bitwise_and::exec),
            BinOperator::AssignBitwiseOr => assign::exec(lhs, rhs, bitwise_or::exec),
            BinOperator::AssignXor => assign::exec(lhs, rhs, xor::exec),
            BinOperator::AssignPow => assign::try_exec(lhs, rhs, pow::exec)?,
            _ => unreachable!(),
        })
    }
}

impl Recreate for BinOperation {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let lhs = self.lhs.recreate(local_variables)?;
        if let BinOperator::And = self.op {
            return and::recreate(lhs, &self.rhs, local_variables);
        }
        if let BinOperator::Or = self.op {
            return or::recreate(lhs, &self.rhs, local_variables);
        }
        let rhs = self.rhs.recreate(local_variables)?;
        match self.op {
            BinOperator::Add => Ok(add::create_from_instructions(lhs, rhs)),
            BinOperator::Subtract => Ok(subtract::create_from_instructions(lhs, rhs)),
            BinOperator::Multiply => Ok(multiply::create_from_instructions(lhs, rhs)),
            BinOperator::Divide => divide::create_from_instructions(lhs, rhs),
            BinOperator::Modulo => modulo::create_from_instructions(lhs, rhs),
            BinOperator::Pow => modulo::create_from_instructions(lhs, rhs),
            BinOperator::Equal => Ok(equal::create_from_instructions(lhs, rhs)),
            BinOperator::NotEqual => Ok(not_equal::create_from_instructions(lhs, rhs)),
            BinOperator::Greater => Ok(greater::create_from_instructions(lhs, rhs)),
            BinOperator::GreaterOrEqual => Ok(greater_equal::create_from_instructions(lhs, rhs)),
            BinOperator::Lower => Ok(lower::create_from_instructions(lhs, rhs)),
            BinOperator::LowerOrEqual => Ok(lower_equal::create_from_instructions(lhs, rhs)),
            BinOperator::And => Ok(and::create_from_instructions(lhs, rhs)),
            BinOperator::Or => Ok(or::create_from_instructions(lhs, rhs)),
            BinOperator::BitwiseAnd => Ok(bitwise_and::create_from_instructions(lhs, rhs)),
            BinOperator::BitwiseOr => Ok(bitwise_or::create_from_instructions(lhs, rhs)),
            BinOperator::Xor => Ok(xor::create_from_instructions(lhs, rhs)),
            BinOperator::LShift => lshift::create_from_instructions(lhs, rhs),
            BinOperator::RShift => rshift::create_from_instructions(lhs, rhs),
            BinOperator::At => at::create_from_instructions(lhs, rhs),
            op => Ok(Self { lhs, rhs, op }.into()),
        }
    }
}

impl ReturnType for BinOperation {
    fn return_type(&self) -> Type {
        let lhs = self.lhs.return_type();
        let rhs = self.rhs.return_type();
        match self.op {
            BinOperator::Add => add::return_type(lhs, rhs),
            BinOperator::Equal | BinOperator::NotEqual | BinOperator::And | BinOperator::Or => {
                Type::Bool
            }
            BinOperator::Greater
            | BinOperator::GreaterOrEqual
            | BinOperator::Lower
            | BinOperator::LowerOrEqual => return_type_bool(lhs, rhs),
            BinOperator::BitwiseAnd
            | BinOperator::BitwiseOr
            | BinOperator::Xor
            | BinOperator::LShift
            | BinOperator::RShift
            | BinOperator::Modulo
            | BinOperator::Subtract
            | BinOperator::Multiply
            | BinOperator::Divide
            | BinOperator::Pow => {
                if var_type!([]).matches(&lhs) {
                    lhs
                } else {
                    rhs
                }
            }
            BinOperator::Filter => lhs,
            BinOperator::Map => map::return_type(rhs),
            BinOperator::At => lhs.index_result().unwrap(),
            BinOperator::Assign => rhs,
            _ => lhs.mut_element_type().unwrap(),
        }
    }
}

mod equal {
    use super::{BinOperation, BinOperator};
    use crate::{instruction::Instruction, variable::Variable};

    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        (lhs == rhs).into()
    }

    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => exec(lhs, rhs).into(),
            (lhs, rhs) => BinOperation {
                lhs,
                rhs,
                op: BinOperator::Equal,
            }
            .into(),
        }
    }
}

mod not_equal {
    use super::{BinOperation, BinOperator};
    use crate::{instruction::Instruction, variable::Variable};

    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        (lhs != rhs).into()
    }

    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => exec(lhs, rhs).into(),
            (lhs, rhs) => BinOperation {
                lhs,
                rhs,
                op: BinOperator::NotEqual,
            }
            .into(),
        }
    }
}

impl InstructionWithStr {
    pub fn create_infix(
        op: Pair<'_, Rule>,
        lhs: Self,
        rhs: Self,
        local_variables: &LocalVariables<'_>,
    ) -> Result<Self, Error> {
        let str = format!("{} {} {}", lhs.str, op.as_str(), rhs.str).into();
        let rule = op.as_rule();
        if rule == Rule::reduce {
            return Ok(Self {
                instruction: Reduce::create_instruction(
                    lhs,
                    op,
                    rhs,
                    &mut local_variables.clone(),
                )?,
                str,
            });
        }
        let (lhs, rhs) = (lhs.instruction, rhs.instruction);
        fn return_type(lhs: &Type, rhs: &Type) -> Type {
            if var_type!([]).matches(lhs) {
                lhs.clone()
            } else {
                rhs.clone()
            }
        }
        fn assign_can_be_used_num(a: &Type, b: &Type) -> bool {
            can_be_used_num(a.clone(), b.clone())
        }
        fn assign_can_be_used_int(a: &Type, b: &Type) -> bool {
            can_be_used_int(a.clone(), b.clone())
        }
        fn bitwise_can_be_used(a: &Type, b: &Type) -> bool {
            bitwise::can_be_used(a.clone(), b.clone())
        }
        let instruction = match rule {
            Rule::pow => pow::create_op(lhs, rhs),
            Rule::multiply => multiply::create_op(lhs, rhs),
            Rule::add => add::create_op(lhs, rhs),
            Rule::subtract => subtract::create_op(lhs, rhs),
            Rule::divide => divide::create_op(lhs, rhs),
            Rule::modulo => modulo::create_op(lhs, rhs),
            Rule::equal => Ok(BinOperation {
                lhs,
                rhs,
                op: BinOperator::Equal,
            }
            .into()),
            Rule::not_equal => Ok(BinOperation {
                lhs,
                rhs,
                op: BinOperator::NotEqual,
            }
            .into()),
            Rule::lower => lower::create_op(lhs, rhs),
            Rule::lower_equal => lower_equal::create_op(lhs, rhs),
            Rule::greater => greater::create_op(lhs, rhs),
            Rule::greater_equal => greater_equal::create_op(lhs, rhs),
            Rule::map => map::create_op(lhs, rhs),
            Rule::filter => filter::create_op(lhs, rhs),
            Rule::bitwise_and => bitwise_and::create_op(lhs, rhs),
            Rule::bitwise_or => bitwise_or::create_op(lhs, rhs),
            Rule::xor => xor::create_op(lhs, rhs),
            Rule::rshift => rshift::create_op(lhs, rhs),
            Rule::lshift => lshift::create_op(lhs, rhs),
            Rule::and => and::create_op(lhs, rhs),
            Rule::or => or::create_op(lhs, rhs),
            Rule::assign => {
                assign::create_op(lhs, rhs, BinOperator::Assign, |_, _| true, |_, x| x.clone())
            }
            Rule::assign_add => assign::create_op(
                lhs,
                rhs,
                BinOperator::AssignAdd,
                add::can_be_used,
                |lhs, rhs| add::return_type(lhs.clone(), rhs.clone()),
            ),
            Rule::assign_subtract => assign::create_op(
                lhs,
                rhs,
                BinOperator::AssignSubtract,
                assign_can_be_used_num,
                return_type,
            ),
            Rule::assing_multiply => assign::create_op(
                lhs,
                rhs,
                BinOperator::AssignMultiply,
                assign_can_be_used_num,
                return_type,
            ),
            Rule::assign_divide => assign::create_op(
                lhs,
                rhs,
                BinOperator::AssignDivide,
                assign_can_be_used_num,
                return_type,
            ),
            Rule::assign_modulo => assign::create_op(
                lhs,
                rhs,
                BinOperator::AssignModulo,
                assign_can_be_used_int,
                return_type,
            ),
            Rule::assign_lshift => assign::create_op(
                lhs,
                rhs,
                BinOperator::AssignLShift,
                assign_can_be_used_int,
                return_type,
            ),
            Rule::assign_rshift => assign::create_op(
                lhs,
                rhs,
                BinOperator::AssignRShift,
                assign_can_be_used_int,
                return_type,
            ),
            Rule::assign_bitwise_and => assign::create_op(
                lhs,
                rhs,
                BinOperator::AssignBitwiseAnd,
                bitwise_can_be_used,
                return_type,
            ),
            Rule::assign_bitwise_or => assign::create_op(
                lhs,
                rhs,
                BinOperator::AssignBitwiseOr,
                bitwise_can_be_used,
                return_type,
            ),
            Rule::assign_xor => assign::create_op(
                lhs,
                rhs,
                BinOperator::AssignXor,
                assign_can_be_used_int,
                return_type,
            ),
            Rule::assign_pow => assign::create_op(
                lhs,
                rhs,
                BinOperator::AssignPow,
                assign_can_be_used_num,
                return_type,
            ),
            rule => unexpected!(rule),
        }?;
        Ok(Self { instruction, str })
    }
}
