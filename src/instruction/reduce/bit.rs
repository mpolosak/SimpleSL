use crate::instruction::tuple::Tuple;
use crate::instruction::BinOperation;
use crate::variable::{Type, Variable};
use crate::{self as simplesl, BinOperator, Code, Interpreter};
use crate::{
    instruction::{Instruction, InstructionWithStr},
    unary_operator::UnaryOperator,
    variable::ReturnType,
    Error,
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, int));
}

lazy_static! {
    pub static ref AND: Variable = Code::parse(
        &Interpreter::without_stdlib(),
        "(iter: () -> (bool, int)) -> int {
            return iter $!0 (acc: int, curr: int) -> int {
                return acc & curr;
            }
        }"
    )
    .unwrap()
    .exec()
    .unwrap();
}

lazy_static! {
    pub static ref OR: Variable = Code::parse(
        &Interpreter::without_stdlib(),
        "(iter: () -> (bool, int)) -> int {
            return iter $0 (acc: int, curr: int) -> int {
                return acc | curr;
            }
        }"
    )
    .unwrap()
    .exec()
    .unwrap();
}

pub fn create(iterator: InstructionWithStr, op: UnaryOperator) -> Result<Instruction, Error> {
    let return_type = iterator.return_type();
    if !return_type.matches(&ACCEPTED_TYPE) {
        return Err(Error::IncorectUnaryOperatorOperand {
            ins: iterator.str,
            op,
            expected: ACCEPTED_TYPE.clone(),
            given: return_type,
        });
    }
    let lhs = if op == UnaryOperator::BitAnd {
        AND.clone().into()
    } else {
        OR.clone().into()
    };
    let rhs = Tuple {
        elements: [iterator].into(),
    }
    .into();
    Ok(BinOperation {
        lhs,
        rhs,
        op: BinOperator::FunctionCall,
    }
    .into())
}
