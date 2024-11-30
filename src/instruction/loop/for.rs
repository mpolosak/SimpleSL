use crate::instruction::control_flow::if_else::return_type;
use crate::instruction::local_variable::LocalVariable;
use crate::instruction::{Exec, ExecResult, ExecStop, Recreate};
use crate::variable::{Type, Variable};
use crate::{self as simplesl, ExecError, Interpreter};
use crate::{
    instruction::{local_variable::LocalVariables, Instruction, InstructionWithStr},
    Error,
};
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::Rule;
use std::sync::Arc;

use super::recreate_instructions;

#[derive(Debug)]
pub struct For {
    index: Arc<str>,
    ident: Arc<str>,
    instructions: Arc<[InstructionWithStr]>,
}

impl For {
    pub fn create(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
        instructions: &mut Vec<InstructionWithStr>,
    ) -> Result<(), Error> {
        let rule = pair.as_rule();
        let str = pair.as_str().into();
        let mut inner = pair.into_inner();
        let index: Arc<str> = if rule == Rule::for_with_index {
            inner.next().unwrap().as_str()
        } else {
            "$i"
        }
        .into();
        let ident: Arc<str> = inner.next().unwrap().as_str().into();
        let pair = inner.next().unwrap();
        InstructionWithStr::create(pair, local_variables, instructions)?;
        let return_type = return_type(instructions);
        let Some(element_type) = return_type.element_type() else {
            return Err(Error::WrongType("array".into(), var_type!([any])));
        };
        local_variables.new_layer();
        let in_loop = local_variables.in_loop;
        local_variables.in_loop = true;
        local_variables.new_layer();
        local_variables.insert(ident.clone(), LocalVariable::Other(element_type));
        local_variables.insert(index.clone(), LocalVariable::Other(Type::Int));
        let mut loop_instructions = Vec::<InstructionWithStr>::new();
        let pair = inner.next().unwrap();
        InstructionWithStr::create(pair, local_variables, &mut loop_instructions)?;
        local_variables.drop_layer();
        local_variables.in_loop = in_loop;
        let instruction = Self {
            index,
            ident,
            instructions: loop_instructions.into(),
        }
        .into();
        instructions.push(InstructionWithStr { instruction, str });
        Ok(())
    }
}

impl Exec for For {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let array = interpreter.result().unwrap().clone().into_array().unwrap();

        for (index, element) in array.iter().cloned().enumerate() {
            interpreter.push_layer();
            interpreter.insert(self.index.clone(), index.into());
            interpreter.insert(self.ident.clone(), element);
            let result = interpreter.exec_all(&self.instructions);
            interpreter.pop_layer();
            match result {
                Ok(_) | Err(ExecStop::Continue) => (),
                Err(ExecStop::Break) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(Variable::Void)
    }
}

impl Recreate for For {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        local_variables.new_layer();
        local_variables.insert(self.ident.clone(), LocalVariable::Other(Type::Any));
        local_variables.insert(self.index.clone(), LocalVariable::Other(Type::Int));
        let instructions = recreate_instructions(&self.instructions, local_variables)?;
        local_variables.drop_layer();
        Ok(Self {
            index: self.index.clone(),
            ident: self.ident.clone(),
            instructions,
        }
        .into())
    }
}

impl From<For> for Instruction {
    fn from(value: For) -> Self {
        Self::For(value.into())
    }
}
