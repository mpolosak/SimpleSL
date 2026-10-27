mod body;
mod param;
pub(crate) use self::body::Body;
pub use self::param::{Param, Params};
use crate::{
    self as simplesl, Code, Error, ExecError, Interpreter,
    instruction::{Exec, ExecStop, function::call::FunctionCall, r#return::Return},
    variable::{FunctionType, ReturnType, Type, Typed, Variable},
};
use derive_more::Display;
use simplesl_macros::var_type;
use std::{iter::zip, sync::Arc};

#[derive(Clone, Debug, Display)]
#[display("({params})->{return_type}")]
pub struct Function {
    pub(crate) ident: Option<Arc<str>>,
    pub(crate) params: Params,
    pub(crate) body: Body,
    pub(crate) return_type: Type,
}

impl Function {
    pub fn new(
        params: Params,
        body: fn(&[Variable]) -> Result<Variable, ExecError>,
        return_type: Type,
    ) -> Self {
        Self {
            ident: None,
            params,
            body: Body::Native(body),
            return_type,
        }
    }

    pub fn of_type(fn_type: &FunctionType) -> Option<Self> {
        let params = fn_type
            .params
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, var_type)| Param {
                name: format!("p{i}").into(),
                var_type,
            })
            .collect();
        let returned = Variable::of_type(&fn_type.return_type)?;
        Some(Self {
            ident: None,
            params,
            body: Body::Lang([Return(returned.into()).into()].into()),
            return_type: fn_type.return_type(),
        })
    }

    pub fn create_call(self: Arc<Self>, args: Arc<[Variable]>) -> Result<Code, Error> {
        let instruction = FunctionCall::create_from_variables(self, args)?;
        Ok(Code {
            instructions: [instruction].into(),
        })
    }

    pub(crate) fn exec(
        self: &Arc<Self>,
        args: &[Variable],
    ) -> Result<Variable, ExecError> {
        let body = match &self.body {
            Body::Lang(body) => body,
            Body::Native(body) => return (body)(args),
        };
        let mut interpreter = Interpreter::without_stdlib();
        if let Some(ident) = &self.ident {
            interpreter.insert(ident.clone(), self.clone().into())
        }
        for (arg, Param { var_type: _, name }) in zip(args, self.params.iter()) {
            interpreter.insert(name.clone(), arg.clone());
        }
        for instruction in body.iter() {
            match instruction.exec(&mut interpreter) {
                Ok(_) => {}
                Err(ExecStop::Return(var)) => return Ok(var),
                Err(ExecStop::Error(error)) => return Err(error),
                Err(ExecStop::Break) => unreachable!("Break outside of loop"),
                Err(ExecStop::Continue) => unreachable!("Continue outside of loop"),
            }
        }
        Ok(Variable::Void)
    }
}

impl Typed for Function {
    fn as_type(&self) -> Type {
        let params = self
            .params
            .iter()
            .map(|Param { name: _, var_type }| var_type.clone())
            .collect();
        let return_type = self.return_type.clone();
        var_type!(params -> return_type)
    }
}

impl ReturnType for Function {
    fn return_type(&self) -> Type {
        self.return_type.clone()
    }
}
