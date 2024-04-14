use crate::{
    function::{check_args, Params},
    instruction::{
        traits::{ExecResult, ExecStop},
        InstructionWithStr,
    },
    interpreter::Interpreter,
    variable::{FunctionType, ReturnType, Type, Variable},
    Error, ExecError,
};
use crate::{
    instruction::{
        function::AnonymousFunction,
        local_variable::{LocalVariable, LocalVariables},
        recreate_instructions,
        traits::{Exec, Recreate},
        Instruction,
    },
    parse::Rule,
};
use pest::iterators::Pair;
use std::sync::Arc;

#[derive(Debug)]
pub struct FunctionCall {
    pub function: InstructionWithStr,
    pub args: Arc<[InstructionWithStr]>,
}

impl FunctionCall {
    pub fn create_instruction(
        function: InstructionWithStr,
        args: Pair<Rule>,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error> {
        let args = args
            .into_inner()
            .map(|pair| InstructionWithStr::new_expression(pair, local_variables))
            .collect::<Result<Arc<_>, Error>>()?;
        let f_type = function.return_type();
        if !f_type.is_function() {
            return Err(Error::NotAFunction(function.str));
        }
        match &function.instruction {
            Instruction::Variable(Variable::Function(function2)) => {
                Self::check_args_with_params(&function.str, &function2.params, &args)?;
            }
            Instruction::LocalVariable(ident, LocalVariable::Function(params, _)) => {
                Self::check_args_with_params(ident, params, &args)?;
            }
            Instruction::AnonymousFunction(AnonymousFunction { params, .. }) => {
                Self::check_args_with_params(&function.str, params, &args)?;
            }
            _ => {
                Self::check_args_with_type(&function.str, &f_type, &args)?;
            }
        };
        Ok(Self { function, args }.into())
    }
    fn check_args_with_params(
        ident: &str,
        params: &Params,
        args: &[InstructionWithStr],
    ) -> Result<(), Error> {
        check_args(
            ident,
            params,
            &args
                .iter()
                .map(ReturnType::return_type)
                .collect::<Box<[Type]>>(),
        )
    }
    fn check_args_with_type(
        pair_str: &str,
        var_type: &Type,
        args: &[InstructionWithStr],
    ) -> Result<(), Error> {
        let params = args
            .iter()
            .map(ReturnType::return_type)
            .collect::<Box<[Type]>>();
        let expected = FunctionType {
            params,
            return_type: Type::Any,
        }
        .into();
        if !var_type.matches(&expected) {
            return Err(Error::WrongType(pair_str.into(), expected));
        }
        Ok(())
    }
}

impl Exec for FunctionCall {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let args = interpreter.exec(&self.args)?;
        let Variable::Function(function) = self.function.exec(interpreter)? else {
            panic!("Tried to call not function")
        };
        function.exec(&args).map_err(ExecStop::from)
    }
}

impl Recreate for FunctionCall {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let function = self.function.recreate(local_variables)?;
        let args = recreate_instructions(&self.args, local_variables)?;
        Ok(Self { function, args }.into())
    }
}

impl ReturnType for FunctionCall {
    fn return_type(&self) -> Type {
        let Type::Function(function_type) = self.function.return_type() else {
            unreachable!();
        };
        function_type.return_type.clone()
    }
}
