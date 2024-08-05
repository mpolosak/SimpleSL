use crate as simplesl;
use crate::instruction::traits::ACCEPTED_NUM_TYPE;
use crate::instruction::{BinOperation, BinOperator, Instruction};
use crate::variable::{Array, Variable};
use crate::{
    instruction::InstructionWithStr,
    variable::{ReturnType, Type},
    Error,
};
use match_any::match_any;
use simplesl_macros::var_type;

pub fn create_op(
    lhs: InstructionWithStr,
    rhs: InstructionWithStr,
) -> Result<InstructionWithStr, Error> {
    let str = format!("{} - {}", lhs.str, rhs.str).into();
    let lhs_type = lhs.return_type();
    let rhs_type = rhs.return_type();
    if !can_be_used(lhs_type.clone(), rhs_type.clone()) {
        return Err(Error::CannotDo2(lhs_type, "-", rhs_type));
    }
    let instruction = create_from_instructions(lhs.instruction, rhs.instruction);
    Ok(InstructionWithStr { instruction, str })
}

pub fn can_be_used(lhs: Type, rhs: Type) -> bool {
    var_type!((lhs, rhs)).matches(&ACCEPTED_NUM_TYPE)
}

pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
    match (lhs, rhs) {
        (Instruction::Variable(lhs), Instruction::Variable(rhs)) => exec(lhs, rhs).into(),
        (lhs, rhs) => BinOperation {
            lhs,
            rhs,
            op: BinOperator::Subtract,
        }
        .into(),
    }
}

pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
    match_any! { (lhs, rhs),
        (Variable::Int(lhs), Variable::Int(rhs)) | (Variable::Float(lhs), Variable::Float(rhs))
            => (lhs - rhs).into(),
        (lhs, Variable::Array(array)) => {
            let elements = array
                .iter()
                .cloned()
                .map(|rhs| exec(lhs.clone(), rhs))
                .collect();
            let element_type = array.element_type().clone();
            Array {
                element_type,
                elements,
            }
            .into()
        },
        (Variable::Array(array), rhs) => {
            let elements = array
                .iter()
                .cloned()
                .map(|lhs| exec(lhs, rhs.clone()))
                .collect();
            let element_type = array.element_type().clone();
            Array {
                element_type,
                elements,
            }
            .into()
        },
        (lhs, rhs) => panic!("Tried to do {lhs} - {rhs}")
    }
}
