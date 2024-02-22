use super::{
    local_variable::LocalVariables,
    traits::{BaseInstruction, ExecResult, ExecStop},
    Exec, Instruction, Recreate,
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
    instruction: Instruction,
    index: Instruction,
}

impl At {
    pub fn create_instruction(
        instruction: Instruction,
        index: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error> {
        let pair = index.into_inner().next().unwrap();
        let index = Instruction::new_expression(pair, interpreter, local_variables)?;
        let required_instruction_type = Type::String | [Type::Any];
        let instruction_return_type = instruction.return_type();
        if index.return_type() != Type::Int {
            return Err(Error::WrongType("index".into(), Type::Int));
        }
        if !instruction_return_type.matches(&required_instruction_type) {
            return Err(Error::CannotIndexInto(instruction_return_type));
        }
        Ok(Self::create_from_instructions(instruction, index)?)
    }
}

impl At {
    fn create_from_instructions(
        instruction: Instruction,
        index: Instruction,
    ) -> Result<Instruction, ExecError> {
        match (instruction, index) {
            (Instruction::Variable(variable), Instruction::Variable(index)) => {
                Ok(at(variable, index)?.into())
            }
            (_, Instruction::Variable(Variable::Int(value))) if value < 0 => {
                Err(ExecError::NegativeIndex)
            }
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
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, ExecError> {
        let instruction = self.instruction.recreate(local_variables, interpreter)?;
        let index = self.index.recreate(local_variables, interpreter)?;
        Self::create_from_instructions(instruction, index)
    }
}

impl ReturnType for At {
    fn return_type(&self) -> Type {
        match self.instruction.return_type() {
            Type::String => Type::String,
            Type::Array(elements_type) => elements_type.as_ref().clone(),
            Type::EmptyArray => Type::Any,
            _ => unreachable!(),
        }
    }
}

impl BaseInstruction for At {}

fn at(variable: Variable, index: Variable) -> Result<Variable, ExecError> {
    let Variable::Int(index) = index else {
        unreachable!("Tried to index with negative value")
    };
    if index < 0 {
        return Err(ExecError::NegativeIndex);
    }
    let index = index as usize;
    match variable {
        Variable::String(string) => string
            .get(index..index)
            .ok_or(ExecError::IndexToBig)
            .map(Variable::from),
        Variable::Array(array) => array.get(index).ok_or(ExecError::IndexToBig).cloned(),
        variable => unreachable!("Tried to index into {}", variable.as_type()),
    }
}
