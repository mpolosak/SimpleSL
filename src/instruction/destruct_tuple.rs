use std::iter::zip;

use super::{
    local_variable::{LocalVariable, LocalVariableMap},
    tuple::Tuple,
    CreateInstruction, Exec, Instruction, Recreate,
};
use crate::{
    error::Error,
    interpreter::{Interpreter, VariableMap},
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct DestructTuple {
    idents: Vec<String>,
    instruction: Box<Instruction>,
}

impl CreateInstruction for DestructTuple {
    fn create_instruction(
        pair: Pair<Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let idents: Vec<String> = pair
            .into_inner()
            .map(|pair| pair.as_str().to_owned())
            .collect();
        let pair = inner.next().unwrap();
        let instruction = Instruction::new(pair, variables, local_variables)?;
        match instruction.get_return_type() {
            Type::Tuple(types) if types.len() == idents.len() => {
                let result = Self {
                    idents,
                    instruction: instruction.into(),
                };
                result.insert_local_variables(local_variables);
                Ok(result.into())
            }
            _ => Err(Error::Other("er".into())),
        }
    }
}

impl DestructTuple {
    fn insert_local_variables(&self, local_variables: &mut LocalVariableMap) {
        match self.instruction.as_ref() {
            Instruction::Variable(Variable::Tuple(elements)) => {
                for (ident, element) in zip(&self.idents, elements.iter()) {
                    local_variables.insert(ident.clone(), LocalVariable::Variable(element.clone()));
                }
            }
            Instruction::Tuple(Tuple { elements }) => {
                for (ident, element) in zip(&self.idents, elements) {
                    local_variables.insert(ident.clone(), element.clone().into());
                }
            }
            instruction => {
                if let Type::Tuple(types) = instruction.get_return_type() {
                    for (ident, var_type) in zip(&self.idents, types) {
                        local_variables.insert(ident.clone(), LocalVariable::Other(var_type));
                    }
                } else {
                    panic!()
                }
            }
        }
    }
}

impl Exec for DestructTuple {
    fn exec(
        &self,
        interpreter: &mut Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<Variable, Error> {
        let result = self.instruction.exec(interpreter, local_variables)?;
        let Variable::Tuple(elements) = result else {panic!()};
        for (ident, element) in zip(&self.idents, elements.iter()) {
            interpreter.variables.insert(ident, element.clone())
        }
        Ok(Variable::Tuple(elements))
    }
}

impl Recreate for DestructTuple {
    fn recreate(self, local_variables: &mut LocalVariableMap, args: &VariableMap) -> Instruction {
        let instruction = self.instruction.recreate(local_variables, args).into();
        let result = Self {
            instruction,
            ..self
        };
        result.insert_local_variables(local_variables);
        result.into()
    }
}

impl GetReturnType for DestructTuple {
    fn get_return_type(&self) -> Type {
        self.instruction.get_return_type()
    }
}

impl From<DestructTuple> for Instruction {
    fn from(value: DestructTuple) -> Self {
        Self::DestructTuple(value)
    }
}
