mod langfunction;
mod nativefunction;
mod param;
pub use self::{
    langfunction::LangFunction,
    nativefunction::NativeFunction,
    param::{Param, Params},
};
use crate::{
    error::Error,
    interpreter::Interpreter,
    variable::{function_type::FunctionType, Generics, GetReturnType, GetType, Type, Variable},
};
use std::{fmt, iter::zip, rc::Rc};

pub trait Function: GetReturnType {
    fn exec(
        &self,
        name: &str,
        interpreter: &mut Interpreter,
        args: &[Variable],
    ) -> Result<Variable, Error> {
        let mut interpreter = interpreter.create_layer();
        let params = self.get_params();
        if let Some(param_name) = &params.catch_rest {
            let from = params.standard.len();
            let rest: Rc<[Variable]> = args.get(from..).unwrap_or(&[]).into();
            interpreter.insert(param_name.clone(), Variable::from(rest));
        }

        for (arg, Param { var_type: _, name }) in zip(args, params.standard.iter()) {
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
        let mut interpreter = interpreter.create_layer();
        let params = self.get_params();
        if let Some(param_name) = &params.catch_rest {
            let from = params.standard.len();
            let rest: Rc<[Variable]> = args.get(from..).unwrap_or(&[]).into();
            interpreter.insert(param_name.clone(), Variable::from(rest));
        } else if args.len() != params.standard.len() {
            return Err(Error::WrongNumberOfArguments(
                name.into(),
                params.standard.len(),
            ));
        }

        for (arg, Param { var_type, name }) in zip(args, params.standard.iter()) {
            if arg.get_type().matches(var_type) {
                interpreter.insert(name.clone(), arg.clone());
            } else {
                return Err(Error::WrongType(name.clone(), var_type.clone()));
            }
        }
        self.exec_intern(name, &mut interpreter)
    }
    fn exec_intern(&self, name: &str, interpreter: &mut Interpreter) -> Result<Variable, Error>;
    fn get_params(&self) -> &Params;
    fn get_generics(&self) -> Option<&Generics>;
}

impl fmt::Display for dyn Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params = self.get_params();
        let return_type = self.get_return_type();
        if let Some(generics) = self.get_generics() {
            write!(f, "function{generics}({params})->{return_type}")
        } else {
            write!(f, "function({params})->{return_type}")
        }
    }
}

impl fmt::Debug for dyn Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params = self.get_params();
        let return_type = self.get_return_type();
        let generics = self.get_generics();
        write!(
            f,
            "dyn Function(params: [{params}], return_type: {return_type:?}, generics: {generics:?})"
        )
    }
}

impl GetType for dyn Function {
    fn get_type(&self) -> Type {
        let params = self.get_params();
        let params_types: Box<[Type]> = params
            .standard
            .iter()
            .map(|Param { name: _, var_type }| var_type.clone())
            .collect();
        let catch_rest = params.catch_rest.is_some();
        let return_type = self.get_return_type();
        FunctionType {
            return_type,
            params: params_types,
            catch_rest,
        }
        .into()
    }
}
