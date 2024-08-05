use crate::instruction::{Instruction, InstructionWithStr};
use crate::variable::ReturnType;
use crate::{self as simplesl, Error};
use crate::{
    instruction::traits::ExecResult,
    variable::{Array, Type, Variable},
};
use simplesl_macros::var_type;

use super::{BinOperation, BinOperator};

pub fn create_op(lhs: InstructionWithStr, rhs: InstructionWithStr) -> Result<Instruction, Error> {
    let lhs_type = lhs.return_type();
    let rhs_type = rhs.return_type();
    if !can_be_used(&lhs_type, &rhs_type) {
        return Err(Error::CannotDo2(lhs_type, stringify!(op), rhs_type));
    }
    Ok(BinOperation {
        lhs: lhs.instruction,
        rhs: rhs.instruction,
        op: BinOperator::Filter,
    }
    .into())
}

pub fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
    let Some(element_type) = lhs.index_result() else {
        return false;
    };
    let element_type2 = element_type.clone();
    let expected_function = var_type!((element_type)->int | (int,element_type2)->int);
    rhs.matches(&expected_function)
}

pub fn exec(array: Variable, function: Variable) -> ExecResult {
    let (Variable::Array(array), Variable::Function(function)) = (&array, &function) else {
        unreachable!("Tried to do {array} ? {function}")
    };
    let array_iter = array.iter().cloned();
    let elements = if function.params.len() == 1 {
        array_iter
            .filter_map(|element| match function.exec(&[element.clone()]) {
                Ok(Variable::Int(0)) => None,
                Ok(_) => Some(Ok(element)),
                e @ Err(_) => Some(e),
            })
            .collect::<Result<_, _>>()
    } else {
        array_iter
            .enumerate()
            .filter_map(
                |(index, element)| match function.exec(&[index.into(), element.clone()]) {
                    Ok(Variable::Int(0)) => None,
                    Ok(_) => Some(Ok(element)),
                    e @ Err(_) => Some(e),
                },
            )
            .collect::<Result<_, _>>()
    }?;
    let element_type = array.element_type().clone();
    Ok(Array {
        element_type,
        elements,
    }
    .into())
}

#[cfg(test)]
mod tests {
    use crate as simplesl;
    use crate::instruction::bin_op::filter;
    use simplesl_macros::var_type;

    #[test]
    fn can_be_used() {
        assert!(filter::can_be_used(
            &var_type!([int]),
            &var_type!((int)->int)
        ));
        assert!(filter::can_be_used(
            &var_type!([int]),
            &var_type!((int, any)->int)
        ));
        assert!(filter::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((int|float)->int)
        ));
        assert!(filter::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((any, any)->int)
        ));
        assert!(filter::can_be_used(
            &var_type!([int] | [float | string]),
            &var_type!((any, int|float|string)->int)
        ));
        assert!(!filter::can_be_used(
            &var_type!(int),
            &var_type!((any, any)->int)
        ));
        assert!(!filter::can_be_used(
            &var_type!([int] | float),
            &var_type!((any, any)->int)
        ));
        assert!(!filter::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((any, int)->int)
        ));
        assert!(!filter::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((float, any)->int)
        ));
        assert!(!filter::can_be_used(
            &var_type!([int] | [float]),
            &var_type!(string)
        ))
    }
}
