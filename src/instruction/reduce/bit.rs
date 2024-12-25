use crate::instruction::tuple::Tuple;
use crate::instruction::BinOperation;
use crate::stdlib::operators::{AND, OR};
use crate::variable::Type;
use crate::{self as simplesl, BinOperator};
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
