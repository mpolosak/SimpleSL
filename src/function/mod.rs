mod langfunction;

mod nativefunction;
mod param;
pub use self::{
    langfunction::LangFunction,
    nativefunction::NativeFunction,
    param::{Param, Params},
};
use crate::interpreter::{Interpreter, VariableMap};
use crate::variable::{Array, Variable};
use crate::variable_type::{GetType, Type};
use crate::{error::Error, variable_type::GetReturnType};
use std::{fmt, iter::zip};

pub trait Function: GetReturnType {
    fn exec(
        &self,
        name: &str,
        intepreter: &mut Interpreter,
        mut args: Array,
    ) -> Result<Variable, Error> {
        let mut args_map = VariableMap::new();
        let params = self.get_params();
        if let Some(param_name) = &params.catch_rest {
            let from = params.standard.len();
            let rest: Array = args.drain(from..).collect();
            args_map.insert(param_name, Variable::Array(rest.into()));
        }

        for (arg, Param { var_type: _, name }) in zip(args, &params.standard) {
            args_map.insert(name, arg);
        }
        self.exec_intern(name, intepreter, args_map)
    }
    fn check_args_and_exec(
        &self,
        name: &str,
        intepreter: &mut Interpreter,
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
        intepreter: &mut Interpreter,
        args: VariableMap,
    ) -> Result<Variable, Error>;
    fn get_params(&self) -> &Params;
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
        let params_types: Vec<Type> = params
            .standard
            .iter()
            .map(|Param { name: _, var_type }| var_type.clone())
            .collect();
        let catch_rest = params.catch_rest.is_some();
        let return_type = self.get_return_type().into();
        Type::Function {
            return_type,
            params: params_types,
            catch_rest,
        }
    }
}
