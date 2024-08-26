use crate as simplesl;
use crate::variable::Type;
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!((bool, bool | [bool]) | ([bool], bool));
}

pub fn can_be_used(lhs: Type, rhs: Type) -> bool {
    var_type!((lhs, rhs)).matches(&ACCEPTED_TYPE)
}

pub mod and {
    use crate::{
        instruction::{BinOperation, BinOperator, Instruction},
        variable::{Array, ReturnType, Variable},
        Error,
    };

    use super::can_be_used;

    pub fn create_op(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if !can_be_used(lhs_type.clone(), rhs_type.clone()) {
            return Err(Error::CannotDo2(lhs_type, "&&", rhs_type));
        }
        Ok(create_from_instructions(lhs, rhs))
    }

    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => exec(lhs, rhs).into(),
            (lhs, rhs) => BinOperation {
                lhs,
                rhs,
                op: BinOperator::And,
            }
            .into(),
        }
    }

    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (result, Variable::Bool(value)) | (Variable::Bool(value), result) if value => result,
            (Variable::Array(array), _) | (_, Variable::Array(array)) => {
                Array::new_repeat(Variable::Bool(false), array.len()).into()
            }
            _ => Variable::Bool(false),
        }
    }
}

pub mod or {
    use crate::{
        instruction::{BinOperation, BinOperator, Instruction},
        variable::{Array, ReturnType, Variable},
        Error,
    };

    use super::can_be_used;

    pub fn create_op(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if !can_be_used(lhs_type.clone(), rhs_type.clone()) {
            return Err(Error::CannotDo2(lhs_type, "||", rhs_type));
        }
        Ok(create_from_instructions(lhs, rhs))
    }

    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => exec(lhs, rhs).into(),
            (lhs, rhs) => BinOperation {
                lhs,
                rhs,
                op: BinOperator::Or,
            }
            .into(),
        }
    }

    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (result, Variable::Bool(value)) | (Variable::Bool(value), result) if !value => result,
            (Variable::Array(array), _) | (_, Variable::Array(array)) => {
                Array::new_repeat(Variable::Bool(true), array.len()).into()
            }
            _ => Variable::Bool(true),
        }
    }
}
