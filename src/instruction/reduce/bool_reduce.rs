use crate::instruction::tuple::Tuple;
use crate::instruction::{BinOperation, Instruction, InstructionWithStr};
use crate::unary_operator::UnaryOperator;
use crate::variable::ReturnType;
use crate::{self as simplesl, BinOperator, Code, Error};
use crate::{
    variable::{Type, Variable},
    Interpreter,
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, bool));
}

lazy_static! {
    static ref ALL: Variable = Code::parse(
        &Interpreter::without_stdlib(),
        "(iter: () -> (bool, bool)) -> () -> bool {
            loop {
                (con, value) := iter();
                if !con break;
                if !value return false;
            }
            return true;
        }"
    )
    .unwrap()
    .exec()
    .unwrap();
}

lazy_static! {
    static ref ANY: Variable = Code::parse(
        &Interpreter::without_stdlib(),
        "(iter: () -> (bool, bool)) -> () -> bool {
            loop {
                (con, value) := iter();
                if !con break;
                if value return true;
            }
            return false;
        }"
    )
    .unwrap()
    .exec()
    .unwrap();
}

pub fn create(array: InstructionWithStr, op: UnaryOperator) -> Result<Instruction, Error> {
    let return_type = array.return_type();
    if !return_type.matches(&ACCEPTED_TYPE) {
        return Err(Error::IncorectUnaryOperatorOperand {
            ins: array.str,
            op,
            expected: ACCEPTED_TYPE.clone(),
            given: return_type,
        });
    }
    let function = if op == UnaryOperator::All {
        ALL.clone().into()
    } else {
        ANY.clone().into()
    };
    let rhs = Tuple {
        elements: [function].into(),
    }
    .into();
    Ok(BinOperation {
        lhs: array.instruction,
        rhs,
        op: BinOperator::FunctionCall,
    }
    .into())
}
