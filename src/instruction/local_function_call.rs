use super::{
    check_args, error_wrong_type, exec_instructions,
    function_call::FunctionCall,
    local_variable::LocalVariableMap,
    recreate_instructions,
    traits::{Exec, Recreate},
    Instruction,
};
use crate::{
    error::Error,
    function::Params,
    interpreter::{Interpreter, VariableMap},
    variable::Variable,
    variable_type::{GetReturnType, Type},
};

#[derive(Clone)]
pub struct LocalFunctionCall {
    ident: String,
    args: Vec<Instruction>,
    return_type: Type,
}

impl LocalFunctionCall {
    pub fn new(
        var_name: &str,
        params: &Params,
        args: Vec<Instruction>,
        return_type: Type,
    ) -> Result<Self, Error> {
        check_args(var_name, params, &args)?;
        Ok(Self {
            ident: String::from(var_name),
            args,
            return_type,
        })
    }
}

impl Exec for LocalFunctionCall {
    fn exec(
        &self,
        interpreter: &mut Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<Variable, Error> {
        let args = exec_instructions(&self.args, interpreter, local_variables)?;
        let Variable::Function(function) = local_variables.get(&self.ident).unwrap() else {
            return Err(error_wrong_type(&self.args, &self.ident));
        };
        function.exec(&self.ident, interpreter, args)
    }
}

impl Recreate for LocalFunctionCall {
    fn recreate(self, local_variables: &mut LocalVariableMap, args: &VariableMap) -> Instruction {
        let instructions = recreate_instructions(self.args, local_variables, args);
        if local_variables.contains_key(&self.ident) {
            Self {
                ident: self.ident,
                args: instructions,
                return_type: self.return_type,
            }
            .into()
        } else {
            let Variable::Function(function) = args.get(&self.ident).unwrap() else {
                        panic!()
                    };
            FunctionCall {
                function,
                args: instructions,
            }
            .into()
        }
    }
}

impl From<LocalFunctionCall> for Instruction {
    fn from(value: LocalFunctionCall) -> Self {
        Self::LocalFunctionCall(value)
    }
}

impl GetReturnType for LocalFunctionCall {
    fn get_return_type(&self) -> Type {
        self.return_type.clone()
    }
}