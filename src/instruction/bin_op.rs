mod bitwise;
mod filter;
mod logic;
mod map;
mod math;
mod shift;
use super::{
    local_variable::LocalVariables,
    reduce::Reduce,
    return_type::{return_type_float, return_type_int},
    Exec, ExecResult, InstructionWithStr, Recreate,
};
use crate::{
    instruction::{traits::CanBeUsed, Instruction},
    variable::{ReturnType, Type},
    Error, ExecError, Interpreter, ToResult,
};
pub use bitwise::{bitwise_and, bitwise_or, xor};
use duplicate::duplicate_item;
pub use math::{add, multiply, pow, subtract};
use math::{divide, greater, greater_equal, lower, lower_equal, modulo};
use pest::iterators::Pair;
use shift::{lshift, rshift};
use simplesl_parser::{unexpected, Rule};

#[derive(Debug)]
pub struct BinOperation {
    pub lhs: Instruction,
    pub rhs: Instruction,
    pub op: BinOperator,
}

#[derive(Debug, Clone, Copy)]
pub enum BinOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Pow,
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Lower,
    LowerOrEqual,
    BitwiseAnd,
    BitwiseOr,
    Xor,
    LShift,
    RShift,
    Filter,
}

#[duplicate_item(T; [Map]; [And]; [Or];
)]
#[derive(Debug)]
pub struct T {
    pub lhs: Instruction,
    pub rhs: Instruction,
}

impl Exec for BinOperation {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        match self.op {
            BinOperator::Add => Ok(add::exec(lhs, rhs)),
            BinOperator::Subtract => Ok(subtract::exec(lhs, rhs)),
            BinOperator::Multiply => Ok(multiply::exec(lhs, rhs)),
            BinOperator::Divide => Ok(divide::exec(lhs, rhs)?),
            BinOperator::Modulo => Ok(modulo::exec(lhs, rhs)?),
            BinOperator::Pow => Ok(pow::exec(lhs, rhs)?),
            BinOperator::Equal => Ok(equal::exec(lhs, rhs)),
            BinOperator::NotEqual => Ok(not_equal::exec(lhs, rhs)),
            BinOperator::Greater => Ok(greater::exec(lhs, rhs)),
            BinOperator::GreaterOrEqual => Ok(greater_equal::exec(lhs, rhs)),
            BinOperator::Lower => Ok(lower::exec(lhs, rhs)),
            BinOperator::LowerOrEqual => Ok(lower_equal::exec(lhs, rhs)),
            BinOperator::BitwiseAnd => Ok(bitwise_and::exec(lhs, rhs)),
            BinOperator::BitwiseOr => Ok(bitwise_or::exec(lhs, rhs)),
            BinOperator::Xor => Ok(xor::exec(lhs, rhs)),
            BinOperator::LShift => Ok(lshift::exec(lhs, rhs)?),
            BinOperator::RShift => Ok(rshift::exec(lhs, rhs)?),
            BinOperator::Filter => Ok(filter::exec(lhs, rhs)?),
        }
    }
}

impl Recreate for BinOperation {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let lhs = self.lhs.recreate(local_variables)?;
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
            BinOperator::BitwiseAnd => Ok(bitwise_and::create_from_instructions(lhs, rhs)),
            BinOperator::BitwiseOr => Ok(bitwise_or::create_from_instructions(lhs, rhs)),
            BinOperator::Xor => Ok(xor::create_from_instructions(lhs, rhs)),
            BinOperator::LShift => lshift::create_from_instructions(lhs, rhs),
            BinOperator::RShift => rshift::create_from_instructions(lhs, rhs),
            op @ BinOperator::Filter => Ok(Self { lhs, rhs, op }.into()),
        }
    }
}

impl ReturnType for BinOperation {
    fn return_type(&self) -> Type {
        let lhs = self.lhs.return_type();
        let rhs = self.rhs.return_type();
        match self.op {
            BinOperator::Add => add::return_type(lhs, rhs),
            BinOperator::Subtract
            | BinOperator::Multiply
            | BinOperator::Divide
            | BinOperator::Pow => return_type_float(lhs, rhs),
            BinOperator::Equal | BinOperator::NotEqual => Type::Int,
            BinOperator::Greater
            | BinOperator::GreaterOrEqual
            | BinOperator::Lower
            | BinOperator::LowerOrEqual
            | BinOperator::BitwiseAnd
            | BinOperator::BitwiseOr
            | BinOperator::Xor
            | BinOperator::LShift
            | BinOperator::RShift
            | BinOperator::Modulo => return_type_int(lhs, rhs),
            BinOperator::Filter => self.lhs.return_type(),
        }
    }
}

impl From<BinOperation> for Instruction {
    fn from(value: BinOperation) -> Self {
        Self::BinOperation(value.into())
    }
}

#[duplicate_item(T op;[Map] [@]; [And] [&&]; [Or] [||])]
impl T {
    pub fn create_op(
        lhs: InstructionWithStr,
        rhs: InstructionWithStr,
    ) -> Result<InstructionWithStr, Error> {
        let str = format!("{} {} {}", lhs.str, stringify!(op), rhs.str).into();
        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if !Self::can_be_used(&lhs_type, &rhs_type) {
            return Err(Error::CannotDo2(lhs_type, stringify!(op), rhs_type));
        }
        let instruction: Result<Instruction, Error> =
            Self::create_from_instructions(lhs.instruction, rhs.instruction).to_result();
        Ok(InstructionWithStr {
            instruction: instruction?,
            str,
        })
    }
}

impl Map {
    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        Self { lhs, rhs }.into()
    }
}

mod equal {
    use super::{BinOperation, BinOperator};
    use crate::{
        instruction::{Instruction, InstructionWithStr},
        variable::Variable,
        Error,
    };

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

    pub fn create_op(
        lhs: InstructionWithStr,
        rhs: InstructionWithStr,
    ) -> Result<InstructionWithStr, Error> {
        let str = format!("{} == {}", lhs.str, rhs.str).into();
        let instruction = create_from_instructions(lhs.instruction, rhs.instruction);
        Ok(InstructionWithStr { instruction, str })
    }
}

mod not_equal {
    use super::{BinOperation, BinOperator};
    use crate::{
        instruction::{Instruction, InstructionWithStr},
        variable::Variable,
        Error,
    };

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

    pub fn create_op(
        lhs: InstructionWithStr,
        rhs: InstructionWithStr,
    ) -> Result<InstructionWithStr, Error> {
        let str = format!("{} != {}", lhs.str, rhs.str).into();
        let instruction = create_from_instructions(lhs.instruction, rhs.instruction);
        Ok(InstructionWithStr { instruction, str })
    }
}

impl InstructionWithStr {
    pub fn create_infix(
        op: Pair<'_, Rule>,
        lhs: Self,
        rhs: Self,
        local_variables: &LocalVariables<'_>,
    ) -> Result<Self, Error> {
        match op.as_rule() {
            Rule::pow => pow::create_op(lhs, rhs),
            Rule::multiply => multiply::create_op(lhs, rhs),
            Rule::add => add::create_op(lhs, rhs),
            Rule::subtract => subtract::create_op(lhs, rhs),
            Rule::divide => divide::create_op(lhs, rhs),
            Rule::modulo => modulo::create_op(lhs, rhs),
            Rule::equal => equal::create_op(lhs, rhs),
            Rule::not_equal => not_equal::create_op(lhs, rhs),
            Rule::lower => lower::create_op(lhs, rhs),
            Rule::lower_equal => lower_equal::create_op(lhs, rhs),
            Rule::greater => greater::create_op(lhs, rhs),
            Rule::greater_equal => greater_equal::create_op(lhs, rhs),
            Rule::map => Map::create_op(lhs, rhs),
            Rule::filter => filter::create_op(lhs, rhs),
            Rule::bitwise_and => bitwise_and::create_op(lhs, rhs),
            Rule::bitwise_or => bitwise_or::create_op(lhs, rhs),
            Rule::xor => xor::create_op(lhs, rhs),
            Rule::rshift => rshift::create_op(lhs, rhs),
            Rule::lshift => lshift::create_op(lhs, rhs),
            Rule::and => And::create_op(lhs, rhs),
            Rule::or => Or::create_op(lhs, rhs),
            Rule::reduce => {
                let str = format!("{} {} {}", lhs.str, op.as_str(), rhs.str).into();
                let instruction = Reduce::create_instruction(lhs, op, rhs, local_variables)?;
                Ok(InstructionWithStr { instruction, str })
            }
            rule => unexpected!(rule),
        }
    }
}
