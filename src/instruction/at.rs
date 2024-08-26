use super::{
    local_variable::LocalVariables, BinOperation, BinOperator, Instruction, InstructionWithStr,
};
use crate as simplesl;
use crate::{
    variable::{ReturnType, Typed, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::Rule;

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
    Ok(create_from_instructions(
        instruction.instruction,
        index.instruction,
    )?)
}

pub fn create_from_instructions(
    instruction: Instruction,
    index: Instruction,
) -> Result<Instruction, ExecError> {
    match (instruction, index) {
        (Instruction::Variable(variable), Instruction::Variable(index)) => {
            Ok(exec(variable, index)?.into())
        }
        (_, Instruction::Variable(Variable::Int(value))) if value < 0 => {
            Err(ExecError::NegativeIndex)
        }
        (Instruction::Array(array), Instruction::Variable(Variable::Int(value)))
            if array.instructions.len() <= (value as usize) =>
        {
            Err(ExecError::IndexToBig)
        }
        (instruction, index) => Ok(BinOperation {
            lhs: instruction,
            rhs: index,
            op: BinOperator::At,
        }
        .into()),
    }
}

pub fn exec(variable: Variable, index: Variable) -> Result<Variable, ExecError> {
    let index = index.into_int().unwrap();
    if index < 0 {
        return Err(ExecError::NegativeIndex);
    }
    let index = index as usize;
    match variable {
        Variable::String(string) => string
            .get(index..=index)
            .ok_or(ExecError::IndexToBig)
            .map(Variable::from),
        Variable::Array(array) => array.get(index).ok_or(ExecError::IndexToBig).cloned(),
        variable => unreachable!("Tried to index into {}", variable.as_type()),
    }
}

#[cfg(test)]
mod tests {
    use crate as simplesl;
    use crate::{instruction::at::exec as at, ExecError};
    use simplesl_macros::var;

    #[test]
    fn check_at() {
        let array = var!([4, 5.5, "var"]);
        assert_eq!(at(array.clone(), var!(0)), Ok(var!(4)));
        assert_eq!(at(array.clone(), var!(1)), Ok(var!(5.5)));
        assert_eq!(at(array.clone(), var!(2)), Ok(var!("var")));
        assert_eq!(at(array.clone(), var!(-1)), Err(ExecError::NegativeIndex));
        assert_eq!(at(array, var!(3)), Err(ExecError::IndexToBig));
        let string = var!("tex");
        assert_eq!(at(string.clone(), var!(0)), Ok(var!("t")));
        assert_eq!(at(string.clone(), var!(2)), Ok(var!("x")));
        assert_eq!(at(string.clone(), var!(3)), Err(ExecError::IndexToBig));
        assert_eq!(at(string, var!(-1)), Err(ExecError::NegativeIndex))
    }
}
