use std::{iter::zip, rc::Rc};

use super::{
    local_variable::{LocalVariable, LocalVariables},
    tuple::Tuple,
    CreateInstruction, Exec, Instruction, Recreate,
};
use crate::{
    error::Error,
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct DestructTuple {
    idents: Rc<[Rc<str>]>,
    instruction: Instruction,
}

impl CreateInstruction for DestructTuple {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let idents: Rc<[Rc<str>]> = pair.into_inner().map(|pair| pair.as_str().into()).collect();
        let pair = inner.next().unwrap();
        let instruction = Instruction::new(pair, interpreter, local_variables)?;
        match instruction.get_return_type() {
            Type::Tuple(types) if types.len() == idents.len() => {
                let result = Self {
                    idents,
                    instruction,
                };
                result.insert_local_variables(local_variables);
                Ok(result.into())
            }
            _ => Err(Error::Other("er")),
        }
    }
}

impl DestructTuple {
    fn insert_local_variables(&self, local_variables: &mut LocalVariables) {
        match &self.instruction {
            Instruction::Variable(Variable::Tuple(elements)) => {
                for (ident, element) in zip(self.idents.iter().cloned(), elements.iter()) {
                    local_variables.insert(ident, LocalVariable::Variable(element.clone()))
                }
            }
            Instruction::Tuple(Tuple { elements }) => {
                for (ident, element) in zip(self.idents.iter().cloned(), elements.iter()) {
                    local_variables.insert(ident, element.into())
                }
            }
            instruction => {
                if let Type::Tuple(types) = instruction.get_return_type() {
                    for (ident, var_type) in zip(self.idents.iter().cloned(), types.iter()) {
                        local_variables.insert(ident, var_type.clone().into())
                    }
                } else {
                    panic!()
                }
            }
        }
    }
}

impl Exec for DestructTuple {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable, Error> {
        let result = self.instruction.exec(interpreter)?;
        let Variable::Tuple(elements) = result else {panic!()};
        for (ident, element) in zip(self.idents.iter(), elements.iter()) {
            interpreter.insert(ident.clone(), element.clone())
        }
        Ok(Variable::Tuple(elements))
    }
}

impl Recreate for DestructTuple {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, Error> {
        let instruction = self.instruction.recreate(local_variables, interpreter)?;
        let result = Self {
            idents: self.idents.clone(),
            instruction,
        };
        result.insert_local_variables(local_variables);
        Ok(result.into())
    }
}

impl GetReturnType for DestructTuple {
    fn get_return_type(&self) -> Type {
        self.instruction.get_return_type()
    }
}

impl From<DestructTuple> for Instruction {
    fn from(value: DestructTuple) -> Self {
        Self::DestructTuple(value.into())
    }
}
