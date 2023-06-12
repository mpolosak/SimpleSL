mod instruction;
mod langfunction;
mod line;
mod macros;
mod nativefunction;
mod param;
pub use self::{
    instruction::Instruction,
    langfunction::LangFunction,
    line::Line,
    nativefunction::NativeFunction,
    param::{Param, Params},
};
use crate::error::Error;
use crate::intepreter::{Intepreter, VariableMap};
use crate::variable::{Array, Variable};
use crate::variable_type::{GetType, Type};
use std::{fmt, iter::zip};

pub trait Function {
    fn exec(
        &self,
        name: &str,
        intepreter: &mut Intepreter,
        mut args: Array,
    ) -> Result<Variable, Error> {
        let mut args_map = VariableMap::new();
        let params = self.get_params();
        if let Some(param_name) = &params.catch_rest {
            let from = params.standard.len();
            let rest: Array = args.drain(from..).collect();
            args_map.insert(param_name, Variable::Array(rest.into()));
        } else if args.len() != params.standard.len() {
            return Err(Error::WrongNumberOfArguments(
                String::from(name),
                params.standard.len(),
            ));
        }

        for (arg, Param { var_type, name }) in zip(args, &params.standard) {
            if arg.get_type().matches(var_type) {
                args_map.insert(name, arg);
            } else {
                return Err(Error::WrongType(name.clone(), var_type.clone()));
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
    fn get_params(&self) -> &Params;
    fn get_return_type(&self) -> Type;
}

impl fmt::Display for dyn Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params = self.get_params();
        let return_type = self.get_return_type();
        write!(f, "function({params})->{return_type}")
    }
}

impl fmt::Debug for dyn Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params = self.get_params();
        let return_type = self.get_return_type();
        write!(
            f,
            "dyn Function(params: [{params}], return_type: {return_type:?})"
        )
    }
}

impl GetType for dyn Function {
    fn get_type(&self) -> Type {
        let params = self.get_params();
        let param_types = params
            .standard
            .iter()
            .map(|Param { name: _, var_type }| var_type.clone())
            .collect();
        let catch_rest = params.catch_rest.is_some();
        let return_type = self.get_return_type();
        Type::Function(Box::new(return_type), param_types, catch_rest)
    }
}
