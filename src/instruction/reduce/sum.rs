use crate::instruction::tuple::Tuple;
use crate::instruction::BinOperation;
use crate::instruction::{Instruction, InstructionWithStr};
use crate::unary_operator::UnaryOperator;
use crate::variable::Type;
use crate::{self as simplesl, BinOperator, Code, Interpreter};
use crate::{
    variable::{ReturnType, Variable},
    Error,
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type =
        var_type!(() -> (bool, int) | () -> (bool, float) | () -> (bool, string));
}

lazy_static! {
    pub static ref INT_SUM: Variable = Code::parse(
        &Interpreter::without_stdlib(),
        "(iter: () -> (bool, int)) -> int {
            return iter $0 (acc: int, curr: int) -> int {
                return acc + curr;
            }
        }"
    )
    .unwrap()
    .exec()
    .unwrap();
}

lazy_static! {
    pub static ref FLOAT_SUM: Variable = Code::parse(
        &Interpreter::without_stdlib(),
        "(iter: () -> (bool, float)) -> float {
            return iter $0.0 (acc: float, curr: float) -> float {
                return acc + curr;
            }
        }"
    )
    .unwrap()
    .exec()
    .unwrap();
}

lazy_static! {
    pub static ref STRING_SUM: Variable = Code::parse(
        &Interpreter::without_stdlib(),
        r#"(iter: () -> (bool, string)) -> string {
            return iter $"" (acc: string, curr: string) -> string {
                return acc + curr;
            }
        }"#
    )
    .unwrap()
    .exec()
    .unwrap();
}

pub fn create(iterator: InstructionWithStr) -> Result<Instruction, Error> {
    let op = UnaryOperator::Sum;
    let return_type = iterator.return_type();
    if !return_type.matches(&ACCEPTED_TYPE) {
        return Err(Error::IncorectUnaryOperatorOperand {
            ins: iterator.str,
            op,
            expected: ACCEPTED_TYPE.clone(),
            given: return_type,
        });
    }
    let lhs = if return_type.matches(&var_type!(() -> (bool, int))) {
        INT_SUM.clone().into()
    } else if return_type.matches(&var_type!(() -> (bool, float))) {
        FLOAT_SUM.clone().into()
    } else {
        STRING_SUM.clone().into()
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
