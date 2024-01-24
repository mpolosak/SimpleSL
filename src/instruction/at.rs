use super::{local_variable::LocalVariables, traits::BaseInstruction, Exec, Instruction, Recreate};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type, Typed, Variable},
    Error, Result,
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
    ) -> Result<Instruction> {
        let pair = index.into_inner().next().unwrap();
        let index = Instruction::new_expression(pair, interpreter, local_variables)?;
        let required_instruction_type = Type::String | [Type::Any];
        let instruction_return_type = instruction.return_type();
        if index.return_type() != Type::Int {
            Err(Error::WrongType("index".into(), Type::Int))
        } else if !instruction_return_type.matches(&required_instruction_type) {
            Err(Error::CannotIndexInto(instruction_return_type))
        } else {
            Self::create_from_instructions(instruction, index)
        }
    }
}

impl At {
    fn create_from_instructions(
        instruction: Instruction,
        index: Instruction,
    ) -> Result<Instruction> {
        match (instruction, index) {
            (Instruction::Variable(variable), Instruction::Variable(index)) => {
                Ok(at(variable, index)?.into())
            }
            (_, Instruction::Variable(Variable::Int(value))) if value < 0 => {
                Err(Error::CannotBeNegative("index"))
            }
            (instruction, index) => Ok(Self { instruction, index }.into()),
        }
    }
}

impl Exec for At {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let result = self.instruction.exec(interpreter)?;
        let index = self.index.exec(interpreter)?;
        at(result, index)
    }
}

impl Recreate for At {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
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

fn at(variable: Variable, index: Variable) -> Result<Variable> {
    let Variable::Int(index) = index else {
        return Err(Error::WrongType("index".into(), Type::Int));
    };
    if index < 0 {
        return Err(Error::CannotBeNegative("index"));
    }
    let index = index as usize;
    match variable {
        Variable::String(string) => string
            .get(index..index)
            .ok_or(Error::IndexToBig)
            .map(Variable::from),
        Variable::Array(array) => array.get(index).ok_or(Error::IndexToBig).cloned(),
        variable => Err(Error::CannotIndexInto(variable.as_type())),
    }
}
