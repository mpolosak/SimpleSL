use super::{local_variable::LocalVariableMap, Exec, Instruction, Recreate};
use crate::{
    error::Error,
    interpreter::{Interpreter, VariableMap},
    parse::Rule,
    variable::Variable,
    variable_type::{GetReturnType, Type},
};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct At {
    instruction: Box<Instruction>,
    index: Box<Instruction>,
}

impl At {
    pub fn new(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariableMap,
        variables: &VariableMap,
    ) -> Result<Self, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let instruction: Box<Instruction> =
            Instruction::new(pair, variables, local_variables)?.into();
        let pair = inner.next().unwrap();
        let index: Box<Instruction> = Instruction::new(pair, variables, local_variables)?.into();
        let required_instruction_type = [Type::String, Type::Array(Type::Any.into())].into();
        match (
            instruction
                .get_return_type()
                .matches(&required_instruction_type),
            index.get_return_type() == Type::Int,
        ) {
            (true, true) => Ok(Self { instruction, index }),
            (true, false) => Err(Error::WrongType("index".into(), Type::Int)),
            (false, _) => Err(Error::WrongType(
                "instruction".into(),
                required_instruction_type,
            )),
        }
    }
}

impl Exec for At {
    fn exec(
        &self,
        interpreter: &mut Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<Variable, Error> {
        let result = self.instruction.exec(interpreter, local_variables)?;
        let index = self.index.exec(interpreter, local_variables)?;
        match (result, index) {
            (Variable::String(string), Variable::Int(index)) => {
                if index < 0 {
                    Err(Error::CannotBeNegative(String::from("index")))
                } else if index as usize > string.len() {
                    Err(Error::IndexToBig)
                } else {
                    let index = index as usize;
                    Ok(string.get(index..index).unwrap().into())
                }
            }
            (Variable::Array(array, _), Variable::Int(index)) => {
                if index < 0 {
                    return Err(Error::CannotBeNegative(String::from("index")));
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
}

impl Recreate for At {
    fn recreate(self, local_variables: &mut LocalVariableMap, args: &VariableMap) -> Instruction {
        let instruction = self.instruction.recreate(local_variables, args).into();
        let index = self.index.recreate(local_variables, args).into();
        Self { instruction, index }.into()
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
        Self::At(value)
    }
}
