use std::sync::Arc;

use crate::{
    instruction::{can_be_used_int, BinOperation, BinOperator, Instruction},
    variable::{Array, ReturnType, Variable},
    Error, ExecError,
};

pub fn create_op(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
    let lhs_type = lhs.return_type();
    let rhs_type = rhs.return_type();
    if !can_be_used_int(lhs_type.clone(), rhs_type.clone()) {
        return Err(Error::CannotDo2(lhs_type, "%", rhs_type));
    }
    Ok(BinOperation {
        lhs,
        rhs,
        op: BinOperator::Modulo,
    }
    .into())
}

pub fn create_from_instructions(
    dividend: Instruction,
    divisor: Instruction,
) -> Result<Instruction, ExecError> {
    match (dividend, divisor) {
        (Instruction::Variable(dividend), Instruction::Variable(divisor)) => {
            Ok(exec(dividend, divisor)?.into())
        }
        (_, Instruction::Variable(Variable::Int(0))) => Err(ExecError::ZeroModulo),
        (Instruction::ArrayRepeat(array), rhs) => Arc::unwrap_or_clone(array)
            .try_map(|lhs| create_from_instructions(lhs, rhs.clone()))
            .map(Instruction::from),
        (lhs, Instruction::ArrayRepeat(array)) => Arc::unwrap_or_clone(array)
            .try_map(|rhs| create_from_instructions(lhs.clone(), rhs))
            .map(Instruction::from),
        (lhs, rhs) => Ok(BinOperation {
            lhs,
            rhs,
            op: BinOperator::Modulo,
        }
        .into()),
    }
}

pub fn exec(dividend: Variable, divisor: Variable) -> Result<Variable, ExecError> {
    match (dividend, divisor) {
        (_, Variable::Int(0)) => Err(ExecError::ZeroModulo),
        (Variable::Int(dividend), Variable::Int(divisor)) => Ok((dividend % divisor).into()),
        (Variable::Array(array), divisor @ Variable::Int(_)) => {
            let elements = array
                .iter()
                .cloned()
                .map(|dividend| exec(dividend, divisor.clone()))
                .collect::<Result<_, _>>()?;
            let element_type = array.element_type().clone();
            Ok(Array {
                element_type,
                elements,
            }
            .into())
        }
        (dividend @ Variable::Int(_), Variable::Array(array)) => {
            let elements = array
                .iter()
                .cloned()
                .map(|divisor| exec(dividend.clone(), divisor))
                .collect::<Result<Arc<_>, _>>()?;
            let element_type = array.element_type().clone();
            Ok(Array {
                element_type,
                elements,
            }
            .into())
        }
        (dividend, divisor) => {
            panic!("Tried to calc {dividend} % {divisor}")
        }
    }
}
