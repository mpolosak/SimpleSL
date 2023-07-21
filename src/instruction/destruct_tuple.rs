use std::{iter::zip, rc::Rc};

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

#[derive(Clone, Debug)]
pub struct DestructTuple {
    idents: Box<[Rc<str>]>,
    instruction: Instruction,
}

impl CreateInstruction for DestructTuple {
    fn create_instruction(
        pair: Pair<Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let idents: Box<[Rc<str>]> = pair.into_inner().map(|pair| pair.as_str().into()).collect();
        let pair = inner.next().unwrap();
        let instruction = Instruction::new(pair, variables, local_variables)?;
        match instruction.get_return_type() {
            Type::Tuple(types) if types.len() == idents.len() => {
                let result = Self {
                    idents,
                    instruction,
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
        match &self.instruction {
            Instruction::Variable(Variable::Tuple(elements)) => {
                local_variables.extend(zip(self.idents.iter(), elements.iter()).map(
                    |(ident, element)| (ident.clone(), LocalVariable::Variable(element.clone())),
                ));
            }
            Instruction::Tuple(Tuple { elements }) => {
                local_variables.extend(
                    zip(self.idents.iter(), elements.iter())
                        .map(|(ident, element)| (ident.clone(), element.clone().into())),
                );
            }
            instruction => {
                if let Type::Tuple(types) = instruction.get_return_type() {
                    local_variables.extend(
                        zip(self.idents.iter(), types.iter())
                            .map(|(ident, var_type)| (ident.clone(), var_type.clone().into())),
                    );
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
        interpreter.variables.extend(
            zip(self.idents.iter(), elements.iter())
                .map(|(ident, element)| (ident.clone(), element.clone())),
        );
        Ok(Variable::Tuple(elements))
    }
}

impl Recreate for DestructTuple {
    fn recreate(
        &self,
        local_variables: &mut LocalVariableMap,
        args: &VariableMap,
    ) -> Result<Instruction, Error> {
        let instruction = self.instruction.recreate(local_variables, args)?;
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
