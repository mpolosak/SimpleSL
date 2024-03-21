use super::{
    local_variable::{LocalVariable, LocalVariables},
    traits::{ExecResult, MutCreateInstruction},
    tuple::Tuple,
    Exec, Instruction, Recreate,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;
use std::{iter::zip, sync::Arc};

#[derive(Debug)]
pub struct DestructTuple {
    idents: Arc<[Arc<str>]>,
    instruction: Instruction,
}

impl MutCreateInstruction for DestructTuple {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let idents: Arc<[Arc<str>]> = pair.into_inner().map(|pair| pair.as_str().into()).collect();
        let pair = inner.next().unwrap();
        let instruction = Instruction::new(pair, interpreter, local_variables)?;
        let expected = Type::Tuple(std::iter::repeat(Type::Any).take(idents.len()).collect());
        if !instruction.return_type().matches(&expected) {
            return Err(Error::WrongType("instruction".into(), expected));
        }
        let result = Self {
            idents,
            instruction,
        };
        result.insert_local_variables(local_variables);
        Ok(result.into())
    }
}

impl DestructTuple {
    fn insert_local_variables(&self, local_variables: &mut LocalVariables) {
        match &self.instruction {
            Instruction::Variable(Variable::Tuple(elements)) => {
                for (ident, element) in zip(self.idents.iter().cloned(), elements.iter()) {
                    local_variables.insert(ident, LocalVariable::Variable(element.clone()));
                }
            }
            Instruction::Tuple(Tuple { elements }) => {
                for (ident, element) in zip(self.idents.iter().cloned(), elements.iter()) {
                    local_variables.insert(ident, element.into());
                }
            }
            instruction => {
                let types = instruction.return_type().flatten_tuple().unwrap();
                for (ident, var_type) in zip(self.idents.iter().cloned(), types.iter()) {
                    local_variables.insert(ident, var_type.clone().into());
                }
            }
        }
    }
}

impl Exec for DestructTuple {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let result = self.instruction.exec(interpreter)?;
        let Variable::Tuple(elements) = result else {
            panic!()
        };
        for (ident, element) in zip(self.idents.iter(), elements.iter()) {
            interpreter.insert(ident.clone(), element.clone());
        }
        Ok(Variable::Tuple(elements))
    }
}

impl Recreate for DestructTuple {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, ExecError> {
        let instruction = self.instruction.recreate(local_variables, interpreter)?;
        let result = Self {
            idents: self.idents.clone(),
            instruction,
        };
        result.insert_local_variables(local_variables);
        Ok(result.into())
    }
}

impl ReturnType for DestructTuple {
    fn return_type(&self) -> Type {
        self.instruction.return_type()
    }
}
