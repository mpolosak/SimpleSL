use super::{local_variable::LocalVariables, CreateInstruction, Exec, Instruction, Recreate};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct At {
    instruction: Instruction,
    index: Instruction,
}

impl CreateInstruction for At {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let instruction = Instruction::new(pair, interpreter, local_variables)?;
        let pair = inner.next().unwrap();
        let index = Instruction::new(pair, interpreter, local_variables)?;
        let required_instruction_type = [Type::String, Type::Array(Type::Any.into())].into();
        let instruction_return_type = instruction.get_return_type();
        match (
            instruction_return_type.matches(&required_instruction_type),
            index.get_return_type() == Type::Int,
        ) {
            (true, true) => Self::create_from_instructions(instruction, index),
            (true, false) => Err(Error::WrongType("index".into(), Type::Int)),
            (false, _) => Err(Error::CannotIndexInto(instruction_return_type)),
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

impl GetReturnType for At {
    fn get_return_type(&self) -> Type {
        match self.instruction.get_return_type() {
            Type::String => Type::String,
            Type::Array(elements_type) => *elements_type,
            Type::EmptyArray => Type::Any,
            _ => panic!(),
        }
    }
}

impl From<At> for Instruction {
    fn from(value: At) -> Self {
        Self::At(value.into())
    }
}

fn at(variable: Variable, index: Variable) -> Result<Variable> {
    match (variable, index) {
        (Variable::String(string), Variable::Int(index)) => {
            if index < 0 {
                Err(Error::CannotBeNegative("index"))
            } else if index as usize > string.len() {
                Err(Error::IndexToBig)
            } else {
                let index = index as usize;
                Ok(string.get(index..index).unwrap().into())
            }
        }
        (Variable::Array(array, _), Variable::Int(index)) => {
            if index < 0 {
                return Err(Error::CannotBeNegative("index"));
            }
            let index = index as usize;
            if index < array.len() {
                Ok(array[index].clone())
            } else {
                Err(Error::IndexToBig)
            }
        }
        _ => panic!(),
    }
}
