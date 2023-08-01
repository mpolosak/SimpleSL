use super::{
    check_args::check_args,
    error_wrong_type, exec_instructions,
    local_variable::LocalVariables,
    recreate_instructions,
    traits::{Exec, Recreate},
    Instruction,
};
use crate::{
    error::Error,
    function::Function,
    interpreter::Interpreter,
    variable::{GetReturnType, Type, Variable},
};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct FunctionCall {
    pub function: Rc<dyn Function>,
    pub args: Box<[Instruction]>,
}

impl FunctionCall {
    pub fn new(
        var_name: &str,
        interpreter: &Interpreter,
        args: Box<[Instruction]>,
    ) -> Result<Self, Error> {
        let Variable::Function(function) = interpreter.get_variable(var_name)? else {
            return Err(error_wrong_type(&args, var_name.into()));
        };
        let params = function.get_params();
        check_args(var_name, params, &args)?;
        Ok(Self { function, args })
    }
}
impl Exec for FunctionCall {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable, Error> {
        let args = exec_instructions(&self.args, interpreter)?;
        self.function.exec("name", interpreter, &args)
    }
}

impl Recreate for FunctionCall {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, Error> {
        let args = recreate_instructions(&self.args, local_variables, interpreter)?;
        Ok(Self {
            function: self.function.clone(),
            args,
        }
        .into())
    }
}

impl From<FunctionCall> for Instruction {
    fn from(value: FunctionCall) -> Self {
        Self::FunctionCall(value)
    }
}

impl GetReturnType for FunctionCall {
    fn get_return_type(&self) -> Type {
        self.function.get_return_type()
    }
}
