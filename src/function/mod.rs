mod instruction;
mod langfunction;
mod line;
mod macros;
mod nativefunction;
mod param;
pub use self::{
    instruction::Instruction, langfunction::LangFunction, line::Line,
    nativefunction::NativeFunction, param::Param,
};
use crate::error::Error;
use crate::intepreter::{Intepreter, VariableMap};
use crate::variable::{Array, Variable};
use crate::variable_type::Type;
use std::{fmt, iter::zip, vec::Vec};

pub trait Function {
    fn exec(
        &self,
        name: &str,
        intepreter: &mut Intepreter,
        mut args: Array,
    ) -> Result<Variable, Error> {
        let mut args_map = VariableMap::new();
        let params = self.get_params();
        match &params.last() {
            Some(Param::CatchRest(param_name)) => {
                let from = params.len() - 1;
                let rest: Array = args.drain(from..).collect();
                args_map.insert(param_name, Variable::Array(rest.into()));
            }
            Some(_) if args.len() != params.len() => {
                return Err(Error::WrongNumberOfArguments(
                    String::from(name),
                    params.len(),
                ));
            }
            None if !args.is_empty() => {
                return Err(Error::WrongNumberOfArguments(String::from(name), 0));
            }
            _ => (),
        }

        for (arg, param) in zip(args, params) {
            if param.get_type().matches(&arg.get_type()) {
                args_map.insert(param.get_name(), arg);
            } else {
                return Err(Error::WrongType(
                    param.get_name().to_owned(),
                    param.get_type(),
                ));
            }
        }
        self.exec_intern(name, intepreter, args_map)
    }
    fn exec_intern(
        &self,
        name: &str,
        intepreter: &mut Intepreter,
        args: VariableMap,
    ) -> Result<Variable, Error>;
    fn get_params(&self) -> &Vec<Param>;
    fn get_return_type(&self) -> Type;
}

impl fmt::Display for dyn Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "function(")?;
        if let [params @ .., last] = &self.get_params()[..] {
            for param in params {
                write!(f, "{param}, ")?;
            }
            write!(f, "{last}")?;
        }
        write!(f, ")")
    }
}
