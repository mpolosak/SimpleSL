use super::{
    check_args::check_args,
    error_wrong_type, exec_instructions,
    local_variable::LocalVariableMap,
    recreate_instructions,
    traits::{Exec, Recreate},
    Instruction,
};
use crate::{
    error::Error,
    function::Function,
    interpreter::{Interpreter, VariableMap, VariableMapTrait},
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
        variables: &VariableMap,
        args: Box<[Instruction]>,
    ) -> Result<Self, Error> {
        let Variable::Function(function) = variables.try_get(var_name)? else {
            return Err(error_wrong_type(&args, var_name));
        };
        let params = function.get_params();
        check_args(var_name, params, &args)?;
        Ok(Self { function, args })
    }
}
impl Exec for FunctionCall {
    fn exec(
        &self,
        interpreter: &mut Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<Variable, Error> {
        let args = exec_instructions(&self.args, interpreter, local_variables)?;
        self.function.exec("name", interpreter, &args)
    }
}

impl Recreate for FunctionCall {
    fn recreate(
        &self,
        local_variables: &mut LocalVariableMap,
        args: &VariableMap,
    ) -> Result<Instruction, Error> {
        let args = recreate_instructions(&self.args, local_variables, args)?;
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
