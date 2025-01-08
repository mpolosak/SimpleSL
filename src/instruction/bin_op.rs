mod assign;
mod bitwise;
mod filter;
mod logic;
mod map;
mod math;
mod partition;
mod shift;
use super::{
    at, function::call, local_variable::LocalVariables, reduce::Reduce, Exec, ExecResult,
    InstructionWithStr, Recreate,
};
use crate::{self as simplesl, BinOperator};
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
use simplesl_parser::Rule;

lazy_static! {
    pub static ref ACCEPTED_INT_TYPE: Type = var_type!((int, int));
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

lazy_static! {
    pub static ref ACCEPTED_COMP_TYPE: Type = var_type!((int, int) | (float | float));
}

pub fn can_be_used_comp(lhs: Type, rhs: Type) -> bool {
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
        let lhs = self.lhs.exec(interpreter)?;
        if let BinOperator::And = self.op {
            return and::exec(lhs, &self.rhs, interpreter);
        }
        if let BinOperator::Or = self.op {
            return or::exec(lhs, &self.rhs, interpreter);
        }
        let rhs = self.rhs.exec(interpreter)?;
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
            BinOperator::FunctionCall => call::exec(lhs, rhs)?,
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
            BinOperator::Partition => partition::exec(lhs, rhs)?,
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
            BinOperator::Equal
            | BinOperator::NotEqual
            | BinOperator::And
            | BinOperator::Or
            | BinOperator::Greater
            | BinOperator::GreaterOrEqual
            | BinOperator::Lower
            | BinOperator::LowerOrEqual => Type::Bool,
            BinOperator::Subtract
            | BinOperator::Multiply
            | BinOperator::Divide
            | BinOperator::Pow => {
                if var_type!([]).matches(&lhs) {
                    lhs
                } else {
                    rhs
                }
            }
            BinOperator::Filter
            | BinOperator::BitwiseAnd
            | BinOperator::BitwiseOr
            | BinOperator::Xor => lhs,
            BinOperator::Partition => partition::return_type(lhs),
            BinOperator::Map => map::return_type(rhs),
            BinOperator::At => lhs.index_result().unwrap(),
            BinOperator::FunctionCall => lhs.return_type().unwrap(),
            BinOperator::Assign => rhs,
            BinOperator::LShift | BinOperator::RShift | BinOperator::Modulo => Type::Int,
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
                instruction: Reduce::create_instruction(lhs, op, rhs, local_variables)?,
                str,
            });
        }
        let op = BinOperator::from(rule);
        let (lhs, rhs) = (lhs.instruction, rhs.instruction);

        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if !can_be_used(&lhs_type, &rhs_type, op) {
            return Err(Error::CannotDo2(lhs_type, op, rhs_type));
        }

        let instruction = BinOperation { lhs, rhs, op }.into();

        Ok(Self { instruction, str })
    }
}

fn can_be_used(lhs: &Type, rhs: &Type, op: BinOperator) -> bool {
    match op {
        BinOperator::Add => add::can_be_used(lhs, rhs),
        BinOperator::Subtract | BinOperator::Multiply | BinOperator::Divide | BinOperator::Pow => {
            can_be_used_num(lhs.clone(), rhs.clone())
        }
        BinOperator::Lower
        | BinOperator::LowerOrEqual
        | BinOperator::Greater
        | BinOperator::GreaterOrEqual => can_be_used_comp(lhs.clone(), rhs.clone()),
        BinOperator::LShift | BinOperator::RShift | BinOperator::Modulo => {
            can_be_used_int(lhs.clone(), rhs.clone())
        }
        BinOperator::Equal
        | BinOperator::NotEqual
        | BinOperator::At
        | BinOperator::FunctionCall => true,
        BinOperator::And | BinOperator::Or => lhs == &Type::Bool && rhs == &Type::Bool,
        BinOperator::BitwiseAnd | BinOperator::BitwiseOr | BinOperator::Xor => {
            bitwise::can_be_used(lhs.clone(), rhs.clone())
        }
        BinOperator::Filter | BinOperator::Partition => filter::can_be_used(lhs, rhs),
        BinOperator::Map => map::can_be_used(lhs, rhs),
        BinOperator::Assign => {
            assign::can_be_used(lhs.clone(), rhs.clone(), |_, _| true, |_, x| x.clone())
        }
        BinOperator::AssignAdd => {
            assign::can_be_used(lhs.clone(), rhs.clone(), add::can_be_used, |lhs, rhs| {
                add::return_type(lhs.clone(), rhs.clone())
            })
        }
        BinOperator::AssignSubtract
        | BinOperator::AssignMultiply
        | BinOperator::AssignDivide
        | BinOperator::AssignPow => assign::can_be_used(
            lhs.clone(),
            rhs.clone(),
            assign_can_be_used_num,
            return_type,
        ),
        BinOperator::AssignModulo | BinOperator::AssignLShift | BinOperator::AssignRShift => {
            assign::can_be_used(
                lhs.clone(),
                rhs.clone(),
                assign_can_be_used_int,
                return_type,
            )
        }
        BinOperator::AssignBitwiseAnd | BinOperator::AssignBitwiseOr | BinOperator::AssignXor => {
            assign::can_be_used(lhs.clone(), rhs.clone(), bitwise_can_be_used, return_type)
        }
    }
}

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
