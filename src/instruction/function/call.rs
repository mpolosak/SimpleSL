use crate::instruction::{
    function::Function,
    local_variable::{LocalVariable, LocalVariables},
    Instruction, Recreate,
};
use crate::{
    function::{self, Param, Params},
    instruction::{Exec, ExecResult, ExecStop, InstructionWithStr},
    variable::{ReturnType, Type, Typed, Variable},
    Error, Interpreter,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::{iter::zip, sync::Arc};

use super::recreate_instructions;

#[derive(Debug)]
pub struct FunctionCall {
    function: InstructionWithStr,
    args: Arc<[InstructionWithStr]>,
}

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
            check_args_with_params(&function.str, &function2.params, &args)?;
        }
        Instruction::LocalVariable(ident, LocalVariable::Function(params, _)) => {
            check_args_with_params(ident, params, &args)?;
        }
        Instruction::Function(Function { params, .. }) => {
            check_args_with_params(&function.str, params, &args)?;
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
            check_args_with_params(&function.str, &Params(params), &args)?;
        }
    };
    Ok(FunctionCall { function, args }.into())
}

pub fn create_from_variables(
    ident: Arc<str>,
    function: Arc<function::Function>,
    args: Vec<Variable>,
    instructions: &mut Vec<InstructionWithStr>,
) -> Result<(), Error> {
    if function.params.len() != args.len() {
        return Err(Error::WrongNumberOfArguments(
            ident.clone(),
            function.params.len(),
        ));
    }
    for (arg, param) in zip(args.iter(), function.params.iter()) {
        let arg_type = arg.as_type();
        if !arg_type.matches(&param.var_type) {
            return Err(Error::WrongArgument {
                function: ident.clone(),
                param: param.clone(),
                given: arg.to_string().into(),
                given_type: arg_type,
            });
        }
    }

    let instruction = Variable::Function(function.clone()).into();
    instructions.push(instruction);

    let ident = function.ident.clone().unwrap_or_else(|| "$".into());
    let str = format!("set {ident}").into();
    let instruction = InstructionWithStr {
        instruction: Instruction::Set(ident.clone()),
        str,
    };
    instructions.push(instruction);

    let args = args.into_iter().map(InstructionWithStr::from);
    for (param, arg) in zip(function.params.iter(), args) {
        instructions.push(arg);
        let ident = param.name.clone();
        let str = format!("set {ident}").into();
        let instruction = InstructionWithStr {
            instruction: Instruction::Set(ident),
            str,
        };
        instructions.push(instruction);
    }

    let instruction = Variable::Function(function).into();
    instructions.push(instruction);

    let instruction = InstructionWithStr {
        instruction: Instruction::Call,
        str: "()".into(),
    };
    instructions.push(instruction);

    Ok(())
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

impl Exec for FunctionCall {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let function = self.function.exec(interpreter)?.into_function().unwrap();
        let args = self
            .args
            .iter()
            .map(|instruction| instruction.exec(interpreter))
            .collect::<Result<Arc<[Variable]>, ExecStop>>()?;
        function.exec_with_args(&args).map_err(ExecStop::from)
    }
}

impl Recreate for FunctionCall {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, crate::ExecError> {
        let function = self.function.recreate(local_variables)?;
        let args = recreate_instructions(&self.args, local_variables)?;
        Ok(Self { function, args }.into())
    }
}

impl ReturnType for FunctionCall {
    fn return_type(&self) -> Type {
        self.function.return_type().return_type().unwrap()
    }
}
