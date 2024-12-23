use crate::{
    self as simplesl,
    instruction::{tuple::Tuple, BinOperation, Instruction, InstructionWithStr},
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type, Variable},
    BinOperator, Code, Error, Interpreter,
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, int) | () -> (bool, float));
}

lazy_static! {
    pub static ref INT_PRODUCT: Variable = Code::parse(
        &Interpreter::without_stdlib(),
        "(iter: () -> (bool, int)) -> int {
            return iter $1 (acc: int, curr: int) -> int {
                return acc * curr;
            }
        }"
    )
    .unwrap()
    .exec()
    .unwrap();
}

lazy_static! {
    pub static ref FLOAT_PRODUCT: Variable = Code::parse(
        &Interpreter::without_stdlib(),
        "(iter: () -> (bool, float)) -> float {
            return iter $1.0 (acc: float, curr: float) -> float {
                return acc * curr;
            }
        }"
    )
    .unwrap()
    .exec()
    .unwrap();
}

pub fn create(iterator: InstructionWithStr) -> Result<Instruction, Error> {
    let op = UnaryOperator::Product;
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
        INT_PRODUCT.clone().into()
    } else {
        FLOAT_PRODUCT.clone().into()
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
