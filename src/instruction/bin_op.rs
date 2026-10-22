mod assign;
mod bitwise;
mod concat;
mod filter;
mod logic;
mod map;
mod math;
mod partition;
mod shift;
use super::{
    Exec, ExecResult, Instruction, InstructionWithStr, Recreate, local_variable::LocalVariables,
    reduce,
};
use crate::{
    self as simplesl, BinOperator, Error, Interpreter,
    instruction::{
        BaseInstruction,
        bin_op::{
            add::Add,
            assign::{Assign, add::AssignAdd},
            logic::{And, Or},
        },
    },
    variable::{ReturnType, Type, Variable},
};
pub use bitwise::{bitwise_and, bitwise_or, xor};
pub use math::{add, multiply, pow, subtract};
use math::{divide, greater, greater_equal, lower, lower_equal, modulo};
use pest::iterators::Pair;
use shift::{lshift, rshift};
use simplesl_macros::var_type;
use simplesl_parser::Rule;

pub fn can_be_used_int(lhs: &Type, rhs: &Type) -> bool {
    lhs == &Type::Int && rhs == &Type::Int
}

pub fn can_be_used_num(lhs: &Type, rhs: &Type) -> bool {
    (lhs == &Type::Int || lhs == &Type::Float) && lhs == rhs
}

#[derive(Debug)]
pub struct BinOperation {
    pub lhs: Instruction,
    pub rhs: Instruction,
    pub op: BinOperator,
}

impl BaseInstruction for BinOperation {}

impl Exec for BinOperation {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok(match self.op {
            BinOperator::Subtract => lhs - rhs,
            BinOperator::Multiply => lhs * rhs,
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
            BinOperator::AssignSubtract => assign::exec(lhs, rhs, |lhs, rhs| lhs - rhs),
            BinOperator::AssignMultiply => assign::exec(lhs, rhs, |lhs, rhs| lhs * rhs),
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
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let lhs = self.lhs.recreate(local_variables);
        let rhs = self.rhs.recreate(local_variables);
        match self.op {
            BinOperator::Subtract => subtract::create_from_instructions(lhs, rhs),
            BinOperator::Multiply => multiply::create_from_instructions(lhs, rhs),
            BinOperator::Equal => equal::create_from_instructions(lhs, rhs),
            BinOperator::NotEqual => not_equal::create_from_instructions(lhs, rhs),
            BinOperator::Greater => greater::create_from_instructions(lhs, rhs),
            BinOperator::GreaterOrEqual => greater_equal::create_from_instructions(lhs, rhs),
            BinOperator::Lower => lower::create_from_instructions(lhs, rhs),
            BinOperator::LowerOrEqual => lower_equal::create_from_instructions(lhs, rhs),
            BinOperator::BitwiseAnd => bitwise_and::create_from_instructions(lhs, rhs),
            BinOperator::BitwiseOr => bitwise_or::create_from_instructions(lhs, rhs),
            BinOperator::Xor => xor::create_from_instructions(lhs, rhs),
            op => Self { lhs, rhs, op }.into(),
        }
    }
}

impl ReturnType for BinOperation {
    fn return_type(&self) -> Type {
        let lhs = self.lhs.return_type();
        let rhs = self.rhs.return_type();
        match self.op {
            BinOperator::Equal
            | BinOperator::NotEqual
            | BinOperator::Greater
            | BinOperator::GreaterOrEqual
            | BinOperator::Lower
            | BinOperator::LowerOrEqual => Type::Bool,
            BinOperator::Subtract
            | BinOperator::Multiply
            | BinOperator::Divide
            | BinOperator::Pow
            | BinOperator::Filter
            | BinOperator::BitwiseAnd
            | BinOperator::BitwiseOr
            | BinOperator::Xor => lhs,
            BinOperator::Partition => partition::return_type(lhs),
            BinOperator::Map => map::return_type(rhs),
            BinOperator::LShift | BinOperator::RShift | BinOperator::Modulo => Type::Int,
            _ => lhs.mut_element_type().unwrap(),
        }
    }
}

mod equal {
    use super::{BinOperator, create_from_instructions_with_exec};
    use crate::{instruction::Instruction, variable::Variable};

    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        (lhs == rhs).into()
    }

    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        create_from_instructions_with_exec(lhs, rhs, BinOperator::Equal, exec)
    }
}

mod not_equal {
    use super::{BinOperator, create_from_instructions_with_exec};
    use crate::{instruction::Instruction, variable::Variable};

    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        (lhs != rhs).into()
    }

    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        create_from_instructions_with_exec(lhs, rhs, BinOperator::NotEqual, exec)
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
        let instruction = Instruction::create_infix(op, lhs, rhs, local_variables)?;
        Ok(Self { instruction, str })
    }
}

impl Instruction {
    fn create_infix(
        op: Pair<'_, Rule>,
        lhs: InstructionWithStr,
        rhs: InstructionWithStr,
        local_variables: &LocalVariables<'_>,
    ) -> Result<Self, Error> {
        let rule = op.as_rule();
        if rule == Rule::reduce {
            return reduce::create_instruction(lhs, op, rhs, local_variables);
        }
        let op = BinOperator::from(rule);
        let (lhs, rhs) = (lhs.instruction, rhs.instruction);

        match op {
            BinOperator::Add => {
                return Add::create_instruction(lhs, rhs);
            }
            BinOperator::Assign => {
                return Assign::create_instruction(lhs, rhs);
            }
            BinOperator::AssignAdd => {
                return AssignAdd::create_instruction(lhs, rhs);
            }
            _ => (),
        }

        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if !can_be_used(&lhs_type, &rhs_type, op) {
            return Err(Error::CannotDo2(lhs_type, op, rhs_type));
        }
        Ok(match op {
            BinOperator::And => And { lhs, rhs }.into(),
            BinOperator::Or => Or { lhs, rhs }.into(),
            _ => BinOperation { lhs, rhs, op }.into(),
        })
    }
}

fn can_be_used(lhs: &Type, rhs: &Type, op: BinOperator) -> bool {
    match op {
        BinOperator::Subtract
        | BinOperator::Multiply
        | BinOperator::Divide
        | BinOperator::Pow
        | BinOperator::Lower
        | BinOperator::LowerOrEqual
        | BinOperator::Greater
        | BinOperator::GreaterOrEqual => can_be_used_num(lhs, rhs),
        BinOperator::LShift | BinOperator::RShift | BinOperator::Modulo => {
            can_be_used_int(lhs, rhs)
        }
        BinOperator::Equal | BinOperator::NotEqual => true,
        BinOperator::And | BinOperator::Or => lhs == &Type::Bool && rhs == &Type::Bool,
        BinOperator::BitwiseAnd | BinOperator::BitwiseOr | BinOperator::Xor => {
            bitwise::can_be_used(lhs, rhs)
        }
        BinOperator::Filter | BinOperator::Partition => filter::can_be_used(lhs, rhs),
        BinOperator::Map => map::can_be_used(lhs, rhs),
        BinOperator::AssignSubtract
        | BinOperator::AssignMultiply
        | BinOperator::AssignDivide
        | BinOperator::AssignPow => assign::can_be_used(lhs, rhs, can_be_used_num, return_type),
        BinOperator::AssignModulo | BinOperator::AssignLShift | BinOperator::AssignRShift => {
            assign::can_be_used(lhs, rhs, can_be_used_int, return_type)
        }
        BinOperator::AssignBitwiseAnd | BinOperator::AssignBitwiseOr | BinOperator::AssignXor => {
            assign::can_be_used(lhs, rhs, bitwise::can_be_used, return_type)
        }
        _ => unreachable!(),
    }
}

fn return_type(lhs: &Type, rhs: &Type) -> Type {
    if var_type!([]).matches(lhs) {
        lhs.clone()
    } else {
        rhs.clone()
    }
}

pub fn create_from_instructions_with_exec<T: FnOnce(Variable, Variable) -> Variable>(
    lhs: Instruction,
    rhs: Instruction,
    op: BinOperator,
    exec: T,
) -> Instruction {
    let (Some(lhs), Some(rhs)) = (
        lhs.downcast_ref::<Variable>(),
        rhs.downcast_ref::<Variable>(),
    ) else {
        return BinOperation { lhs, rhs, op }.into();
    };
    exec(lhs.clone(), rhs.clone()).into()
}
