mod body;
mod check_args;
mod param;
pub(crate) use self::body::Body;
pub use self::{
    check_args::check_args,
    param::{Param, Params},
};
use crate::{
    instruction::{ExecStop, FunctionCall, Instruction},
    interpreter::Interpreter,
    variable::{FunctionType, ReturnType, Type, Typed, Variable},
    Code, Error, ExecError,
};
use std::{fmt, iter::zip, rc::Rc};

#[derive(Debug)]
pub struct Function {
    pub(crate) ident: Option<Rc<str>>,
    pub(crate) params: Params,
    pub(crate) body: Body,
    pub(crate) return_type: Type,
}

impl Function {
    pub fn new(
        params: Params,
        body: fn(&mut Interpreter) -> Result<Variable, ExecError>,
        return_type: Type,
    ) -> Self {
        Self {
            ident: None,
            params,
            body: Body::Native(body),
            return_type,
        }
    }

    pub fn create_call(self: Rc<Self>, args: Vec<Variable>) -> Result<Code, Error> {
        let types = args.iter().map(Typed::as_type).collect::<Box<[Type]>>();
        let args = args.into_iter().map(Instruction::from).collect();
        check_args("function", &self.params, &types)?;
        Ok(Code {
            instructions: Rc::new([FunctionCall {
                function: Variable::Function(self).into(),
                args,
            }
            .into()]),
        })
    }

    pub(crate) fn exec(self: &Rc<Self>, args: &[Variable]) -> Result<Variable, ExecError> {
        let mut interpreter = Interpreter::without_stdlib();
        if let Some(ident) = &self.ident {
            interpreter.insert(ident.clone(), self.clone().into())
        }
        for (arg, Param { var_type: _, name }) in zip(args, self.params.iter()) {
            interpreter.insert(name.clone(), arg.clone());
        }
        let body = match &self.body {
            Body::Lang(body) => body,
            Body::Native(body) => return (body)(&mut interpreter),
        };
        match interpreter.exec(body) {
            Ok(_) => Ok(Variable::Void),
            Err(ExecStop::Return(var)) => Ok(var),
            Err(ExecStop::Error(error)) => Err(error),
        }
    }
}

impl Typed for Function {
    fn as_type(&self) -> Type {
        let params: Box<[Type]> = self
            .params
            .iter()
            .map(|Param { name: _, var_type }| var_type.clone())
            .collect();
        let return_type = self.return_type.clone();
        FunctionType {
            params,
            return_type,
        }
        .into()
    }
}

impl ReturnType for Function {
    fn return_type(&self) -> Type {
        self.return_type.clone()
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let return_type = self.return_type();
        write!(f, "({})->{return_type}", self.params)
    }
}
