mod check_args;
mod langfunction;
mod nativefunction;
mod param;
pub use self::{
    check_args::check_args,
    langfunction::LangFunction,
    nativefunction::NativeFunction,
    param::{Param, Params},
};
use crate::{
    interpreter::Interpreter,
    variable::{function_type::FunctionType, GetReturnType, GetType, Type, Variable},
    Error,
};
use std::{fmt, iter::zip};

pub trait Function: GetReturnType {
    fn exec(
        &self,
        name: &str,
        interpreter: &mut Interpreter,
        args: &[Variable],
    ) -> Result<Variable, Error> {
        let mut interpreter = interpreter.create_layer();
        let params = self.get_params();
        for (arg, Param { var_type: _, name }) in zip(args, params.iter()) {
            interpreter.insert(name.clone(), arg.clone());
        }
        self.exec_intern(name, &mut interpreter)
    }
    fn check_args_and_exec(
        &self,
        name: &str,
        interpreter: &mut Interpreter,
        args: &[Variable],
    ) -> Result<Variable, Error> {
        let params = self.get_params();
        check_args(
            name,
            params,
            &args.iter().map(Variable::get_type).collect::<Box<[Type]>>(),
        )?;
        self.exec(name, interpreter, args)
    }
    fn exec_intern(&self, name: &str, interpreter: &mut Interpreter) -> Result<Variable, Error>;
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
        let params_types: Box<[Type]> = params
            .iter()
            .map(|Param { name: _, var_type }| var_type.clone())
            .collect();
        let return_type = self.get_return_type();
        FunctionType {
            return_type,
            params: params_types,
        }
        .into()
    }
}
