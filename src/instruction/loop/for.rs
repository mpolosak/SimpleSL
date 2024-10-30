use crate::instruction::local_variable::LocalVariable;
use crate::instruction::{Exec, ExecResult, ExecStop, Recreate};
use crate::variable::{Type, Variable};
use crate::{self as simplesl, ExecError, Interpreter};
use crate::{
    instruction::{local_variable::LocalVariables, Instruction, InstructionWithStr},
    variable::ReturnType,
    Error,
};
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug)]
pub struct For {
    index: Arc<str>,
    ident: Arc<str>,
    array: InstructionWithStr,
    instruction: InstructionWithStr,
}

impl For {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let rule = pair.as_rule();
        let mut inner = pair.into_inner();
        let index: Arc<str> = if rule == Rule::for_with_index {
            inner.next().unwrap().as_str()
        } else {
            "$i"
        }
        .into();
        let ident: Arc<str> = inner.next().unwrap().as_str().into();
        let array = InstructionWithStr::new_expression(inner.next().unwrap(), local_variables)?;
        let Some(element_type) = array.return_type().element_type() else {
            return Err(Error::WrongType("array".into(), var_type!([any])));
        };
        let mut local_variables = local_variables.create_layer();
        local_variables.in_loop = true;
        local_variables.insert(ident.clone(), LocalVariable::Other(element_type));
        local_variables.insert(index.clone(), LocalVariable::Other(Type::Int));
        let instruction =
            InstructionWithStr::new_expression(inner.next().unwrap(), &local_variables)?;
        Ok(Self {
            index,
            ident,
            array,
            instruction,
        }
        .into())
    }
}

impl Exec for For {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let array = self.array.exec(interpreter)?.into_array().unwrap();
        let mut interpreter = interpreter.create_layer();
        for (index, element) in array.iter().cloned().enumerate() {
            interpreter.insert(self.index.clone(), index.into());
            interpreter.insert(self.ident.clone(), element);
            match self.instruction.exec(&mut interpreter) {
                Ok(_) | Err(ExecStop::Continue) => (),
                Err(ExecStop::Break) => break,
                e => return e,
            }
        }
        Ok(Variable::Void)
    }
}

impl Recreate for For {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let array = self.array.recreate(local_variables)?;
        let mut local_variables = local_variables.create_layer();
        local_variables.insert(
            self.ident.clone(),
            LocalVariable::Other(array.return_type().element_type().unwrap()),
        );
        local_variables.insert(self.index.clone(), LocalVariable::Other(Type::Int));
        let instruction = self.instruction.recreate(&mut local_variables)?;
        Ok(Self {
            index: self.index.clone(),
            ident: self.ident.clone(),
            array,
            instruction,
        }
        .into())
    }
}

impl From<For> for Instruction {
    fn from(value: For) -> Self {
        Self::For(value.into())
    }
}
