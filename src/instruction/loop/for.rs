use crate::{
    self as simplesl,
    Error, Interpreter,
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, Recreate,
        local_variable::{LocalVariable, LocalVariables},
    },
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug)]
pub struct For {
    ident: Arc<str>,
    iter: Instruction,
    body: Instruction,
}

impl For {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let ident: Arc<str> = inner.next().unwrap().as_str().into();
        let iter = Instruction::new(inner.next().unwrap(), local_variables)?;
        let Some(iter_element) = iter.return_type().iter_element() else {
            return Err(Error::WrongType(
                "iterator".into(),
                var_type!(() -> (bool, any)),
            ));
        };
        let mut local_variables = local_variables.create_layer();
        local_variables.in_loop = true;
        local_variables.insert(ident.clone(), LocalVariable::Other(iter_element));
        let body = Instruction::new(inner.next().unwrap(), &mut local_variables)?;
        Ok(Self { ident, iter, body }.into())
    }
}

impl BaseInstruction for For {}

impl Exec for For {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let iter = self.iter.exec(interpreter)?.into_function().unwrap();
        let mut interpreter = interpreter.create_layer();
        loop {
            let tuple = iter.exec_with_args(&[])?.into_tuple().unwrap();
            if let Variable::Bool(false) = tuple[0] {
                break;
            }
            interpreter.insert(self.ident.clone(), tuple[1].clone());
            self.body.exec(&mut interpreter)?;
        }
        Ok(Variable::Void)
    }
}

impl Recreate for For {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let iter = self.iter.recreate(local_variables);
        let mut local_variables = local_variables.create_layer();
        local_variables.insert(self.ident.clone(), LocalVariable::Other(iter.return_type()));
        let body = self.body.recreate(&mut local_variables);
        Self {
            ident: self.ident.clone(),
            iter,
            body,
        }
        .into()
    }
}

impl ReturnType for For {
    fn return_type(&self) -> Type {
        Type::Void
    }
}
