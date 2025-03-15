use super::{BinOperation, Instruction, InstructionWithStr, local_variable::LocalVariables};
use crate::{
    self as simplesl, BinOperator, Error, ExecError,
    stdlib::add_string::len,
    variable::{ReturnType, Typed, Variable},
};
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::Rule;
use std::ops::Range;

pub fn create(
    instruction: InstructionWithStr,
    index: Pair<Rule>,
    local_variables: &LocalVariables,
) -> Result<Instruction, Error> {
    let pair = index.into_inner().next().unwrap();
    let index = InstructionWithStr::new_expression(pair, local_variables)?;
    let required_instruction_type = var_type!(string | [any]);
    let instruction_return_type = instruction.return_type();
    if index.return_type() != var_type!(int) {
        return Err(Error::CannotIndexWith(index.str));
    }
    if !instruction_return_type.matches(&required_instruction_type) {
        return Err(Error::CannotIndexInto(instruction_return_type));
    }
    Ok(BinOperation {
        lhs: instruction.instruction,
        rhs: index.instruction,
        op: BinOperator::At,
    }
    .into())
}

pub fn create_from_instructions(
    instruction: Instruction,
    index: Instruction,
) -> Result<Instruction, ExecError> {
    match (instruction, index) {
        (Instruction::Variable(variable), Instruction::Variable(index)) => {
            Ok(exec(variable, index)?.into())
        }
        (Instruction::Array(array), Instruction::Variable(Variable::Int(value)))
            if !range(array.instructions.len()).contains(&value) =>
        {
            Err(ExecError::IndexOutOfBounds)
        }
        (instruction, index) => Ok(BinOperation {
            lhs: instruction,
            rhs: index,
            op: BinOperator::At,
        }
        .into()),
    }
}

fn range(value: usize) -> Range<i64> {
    let value = value as i64;
    -value..value
}

pub fn exec(variable: Variable, index: Variable) -> Result<Variable, ExecError> {
    let index = index.into_int().unwrap();
    let index = if index >= 0 {
        index as usize
    } else {
        let index = len(&variable) as i64 + index;
        if index < 0 {
            return Err(ExecError::IndexOutOfBounds);
        }
        index as usize
    };
    match variable {
        Variable::String(string) => string
            .chars()
            .nth(index)
            .ok_or(ExecError::IndexOutOfBounds)
            .map(|ch| ch.to_string().into()),
        Variable::Array(array) => array.get(index).ok_or(ExecError::IndexOutOfBounds).cloned(),
        variable => unreachable!("Tried to index into {}", variable.as_type()),
    }
}

#[cfg(test)]
mod tests {
    use crate as simplesl;
    use crate::{ExecError, instruction::at::exec as at};
    use simplesl_macros::var;

    #[test]
    fn check_at() {
        let array = var!([4, 5.5, "var"]);
        assert_eq!(at(array.clone(), var!(0)), Ok(var!(4)));
        assert_eq!(at(array.clone(), var!(1)), Ok(var!(5.5)));
        assert_eq!(at(array.clone(), var!(2)), Ok(var!("var")));
        assert_eq!(at(array.clone(), var!(-1)), Ok(var!("var")));
        assert_eq!(at(array, var!(3)), Err(ExecError::IndexOutOfBounds));
        let string = var!("tex");
        assert_eq!(at(string.clone(), var!(0)), Ok(var!("t")));
        assert_eq!(at(string.clone(), var!(2)), Ok(var!("x")));
        assert_eq!(
            at(string.clone(), var!(3)),
            Err(ExecError::IndexOutOfBounds)
        );
        assert_eq!(at(string, var!(-1)), Ok(var!("x")))
    }
}
