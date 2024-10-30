mod body;
mod param;
pub(crate) use self::body::Body;
pub use self::param::{Param, Params};
use crate as simplesl;
use crate::instruction::block::Block;
use crate::instruction::function::call;
use crate::{
    instruction::{ExecStop, InstructionWithStr},
    interpreter::Interpreter,
    join,
    variable::{ReturnType, Type, Typed, Variable},
    Code, Error, ExecError,
};
use simplesl_macros::var_type;
use std::{fmt, iter::zip, sync::Arc};

#[derive(Debug)]
pub struct Function {
    pub(crate) ident: Option<Arc<str>>,
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

    pub fn create_call(self: Arc<Self>, args: Vec<Variable>) -> Result<Code, Error> {
        let ident = self.ident.clone().unwrap_or_else(|| Arc::from("function"));
        let str = format!("{}({})", ident, join(args.iter(), ", ")).into();
        let instructions = call::create_from_variables(ident, self, args)?;
        Ok(Code {
            instructions: Arc::new([InstructionWithStr {
                instruction: Block { instructions }.into(),
                str,
            }]),
        })
    }

    pub(crate) fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable, ExecError> {
        let body = match &self.body {
            Body::Lang(body) => body,
            Body::Native(body) => return (body)(interpreter),
        };
        match interpreter.exec(body) {
            Ok(_) => Ok(Variable::Void),
            Err(ExecStop::Return(var)) => Ok(var),
            Err(ExecStop::Error(error)) => Err(error),
            Err(ExecStop::Break) => unreachable!("Break outside of loop"),
            Err(ExecStop::Continue) => unreachable!("Continue outside of loop"),
        }
    }

    pub(crate) fn exec_with_args(
        self: &Arc<Self>,
        args: &[Variable],
    ) -> Result<Variable, ExecError> {
        let mut interpreter = Interpreter::without_stdlib();
        if let Some(ident) = &self.ident {
            interpreter.insert(ident.clone(), self.clone().into())
        }
        for (arg, Param { var_type: _, name }) in zip(args, self.params.iter()) {
            interpreter.insert(name.clone(), arg.clone());
        }
        self.exec(&mut interpreter)
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

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let return_type = self.return_type();
        write!(f, "({})->{return_type}", self.params)
    }
}
