use super::{
    local_variable::LocalVariables,
    traits::{ExecResult, ExecStop},
    Exec, Instruction, InstructionWithStr, Recreate,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type, Typed, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct At {
    instruction: InstructionWithStr,
    index: InstructionWithStr,
}

impl At {
    pub fn create_instruction(
        instruction: InstructionWithStr,
        index: Pair<Rule>,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error> {
        let pair = index.into_inner().next().unwrap();
        let index = InstructionWithStr::new_expression(pair, local_variables)?;
        let required_instruction_type = Type::String | [Type::Any];
        let instruction_return_type = instruction.return_type();
        if index.return_type() != Type::Int {
            return Err(Error::CannotIndexWith(index.str));
        }
        if !instruction_return_type.matches(&required_instruction_type) {
            return Err(Error::CannotIndexInto(instruction_return_type));
        }
        Ok(Self::create_from_instructions(instruction, index)?)
    }
}

impl At {
    fn create_from_instructions(
        instruction: InstructionWithStr,
        index: InstructionWithStr,
    ) -> Result<Instruction, ExecError> {
        match (instruction, index) {
            (
                InstructionWithStr {
                    instruction: Instruction::Variable(variable),
                    ..
                },
                InstructionWithStr {
                    instruction: Instruction::Variable(index),
                    ..
                },
            ) => Ok(at(variable, index)?.into()),
            (
                _,
                InstructionWithStr {
                    instruction: Instruction::Variable(Variable::Int(value)),
                    ..
                },
            ) if value < 0 => Err(ExecError::NegativeIndex),
            (
                InstructionWithStr {
                    instruction: Instruction::Array(array),
                    ..
                },
                InstructionWithStr {
                    instruction: Instruction::Variable(Variable::Int(value)),
                    ..
                },
            ) if array.instructions.len() <= (value as usize) => Err(ExecError::IndexToBig),
            (instruction, index) => Ok(Self { instruction, index }.into()),
        }
    }
}

impl Exec for At {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let result = self.instruction.exec(interpreter)?;
        let index = self.index.exec(interpreter)?;
        at(result, index).map_err(ExecStop::from)
    }
}

impl Recreate for At {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let instruction = self.instruction.recreate(local_variables)?;
        let index = self.index.recreate(local_variables)?;
        Self::create_from_instructions(instruction, index)
    }
}

impl ReturnType for At {
    fn return_type(&self) -> Type {
        self.instruction.return_type().index_result().unwrap()
    }
}

fn at(variable: Variable, index: Variable) -> Result<Variable, ExecError> {
    let Variable::Int(index) = index else {
        unreachable!("Tried to index with {}", index.as_type())
    };
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
    use crate::{instruction::at::at, variable::Variable, ExecError};
    use std::str::FromStr;

    #[test]
    fn check_at() {
        let array = Variable::from_str("[4, 5.5, \"var\"]").unwrap();
        assert_eq!(at(array.clone(), Variable::Int(0)), Ok(Variable::Int(4)));
        assert_eq!(
            at(array.clone(), Variable::Int(1)),
            Ok(Variable::Float(5.5))
        );
        assert_eq!(
            at(array.clone(), Variable::Int(2)),
            Ok(Variable::String("var".into()))
        );
        assert_eq!(
            at(array.clone(), Variable::Int(-1)),
            Err(ExecError::NegativeIndex)
        );
        assert_eq!(at(array, Variable::Int(3)), Err(ExecError::IndexToBig));
        let string = Variable::String("tex".into());
        assert_eq!(
            at(string.clone(), Variable::Int(0)),
            Ok(Variable::String("t".into()))
        );
        assert_eq!(
            at(string.clone(), Variable::Int(2)),
            Ok(Variable::String("x".into()))
        );
        assert_eq!(
            at(string.clone(), Variable::Int(3)),
            Err(ExecError::IndexToBig)
        );
        assert_eq!(at(string, Variable::Int(-1)), Err(ExecError::NegativeIndex))
    }
}
