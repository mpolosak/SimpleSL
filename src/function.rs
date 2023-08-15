mod body;
mod check_args;
mod param;
pub use self::{
    body::Body,
    check_args::check_args,
    param::{Param, Params},
};
use crate::{
    instruction::Exec,
    interpreter::Interpreter,
    variable::{function_type::FunctionType, GetReturnType, GetType, Type, Variable},
    Result,
};
use std::{fmt, iter::zip, rc::Rc};

#[derive(Debug)]
pub struct Function {
    pub ident: Option<Rc<str>>,
    pub params: Params,
    pub body: Body,
    pub return_type: Type,
}

impl Function {
    pub fn exec(
        self: &Rc<Self>,
        interpreter: &mut Interpreter,
        args: &[Variable],
    ) -> Result<Variable> {
        let mut interpreter = interpreter.create_layer();
        if let Some(ident) = &self.ident {
            interpreter.insert(ident.clone(), self.clone().into())
        }
        for (arg, Param { var_type: _, name }) in zip(args, self.params.iter()) {
            interpreter.insert(name.clone(), arg.clone());
        }
        self.body.exec(&mut interpreter)
    }
}

impl GetType for Function {
    fn get_type(&self) -> Type {
        let params: Box<[Type]> = self
            .params
            .iter()
            .map(|Param { name: _, var_type }| var_type.clone())
            .collect();
        let return_type = self.return_type.clone();
        FunctionType {
            return_type,
            params,
        }
        .into()
    }
}

impl GetReturnType for Function {
    fn get_return_type(&self) -> Type {
        self.return_type.clone()
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let return_type = self.get_return_type();
        write!(f, "function({})->{return_type}", self.params)
    }
}
