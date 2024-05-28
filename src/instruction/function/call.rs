use crate::instruction::{
    function::AnonymousFunction,
    local_variable::{LocalVariable, LocalVariables},
    recreate_instructions,
    traits::{Exec, Recreate},
    Instruction,
};
use crate::{
    function::{Function, Param, Params},
    instruction::{
        traits::{ExecResult, ExecStop},
        InstructionWithStr,
    },
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::{iter::zip, sync::Arc};

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
                let f_type = function.return_type();
                if !f_type.is_function() {
                    return Err(Error::NotAFunction(function.str));
                }
                let Some(params) = f_type.params() else {
                    return Err(Error::CannotDetermineParams(function.str));
                };
                let params = params
                    .iter()
                    .enumerate()
                    .map(|(i, var_type)| Param {
                        name: format!("#{i}").into(),
                        var_type: var_type.clone(),
                    })
                    .collect();
                Self::check_args_with_params(&function.str, &Params(params), &args)?;
            }
        };
        Ok(Self { function, args }.into())
    }
    pub fn create_from_variables(
        ident: Arc<str>,
        function: Arc<Function>,
        args: Vec<Variable>,
    ) -> Result<Instruction, Error> {
        let args: Arc<[InstructionWithStr]> =
            args.into_iter().map(InstructionWithStr::from).collect();
        Self::check_args_with_params(&ident, &function.params, &args)?;
        Ok(FunctionCall {
            function: Variable::Function(function).into(),
            args,
        }
        .into())
    }
    fn check_args_with_params(
        ident: &Arc<str>,
        params: &Params,
        args: &[InstructionWithStr],
    ) -> Result<(), Error> {
        if params.len() != args.len() {
            return Err(Error::WrongNumberOfArguments(ident.clone(), params.len()));
        }
        for (arg, param) in zip(args, params.iter()) {
            let arg_type = arg.return_type();
            if !arg_type.matches(&param.var_type) {
                return Err(Error::WrongArgument {
                    function: ident.clone(),
                    param: param.clone(),
                    given: arg.str.clone(),
                    given_type: arg_type,
                });
            }
        }
        Ok(())
    }
}

impl Exec for FunctionCall {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let args = interpreter.exec(&self.args)?;
        let function = self.function.exec(interpreter)?.into_function().unwrap();
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
