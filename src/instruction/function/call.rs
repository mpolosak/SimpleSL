use crate::{
    Error, Interpreter,
    function::{Function as VarFunc, Param, Params},
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, InstructionWithStr, Recreate,
        local_variable::LocalVariables, tuple::Tuple,
    },
    variable::{ReturnType, Type, Typed, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::{iter::zip, sync::Arc};

#[derive(Debug)]
pub struct FunctionCall {
    pub function: Instruction,
    pub args: Instruction,
}

impl FunctionCall {
    pub fn create_instruction(
        function: InstructionWithStr,
        args: Pair<Rule>,
        local_variables: &LocalVariables,
        errors: &mut Vec<Error>
    ) -> Instruction {
        let args = args
            .into_inner()
            .map(|pair| InstructionWithStr::new_expression(pair, local_variables, errors))
            .collect::<Box<_>>();

        if let Some(Variable::Function(function2)) = function.instruction.downcast_ref::<Variable>()
        {
            check_args_with_params(&function.str, &function2.params, &args, errors);
        } else {
            let f_type = function.return_type();
            if !f_type.is_function() {
                errors.push(Error::NotAFunction(function.str.clone()));
            }
            let Some(params) = f_type.params() else {
                errors.push(Error::CannotDetermineParams(function.str));
                let args = Tuple { elements: [].into() }.into();
                return Self {
                    function: function.instruction,
                    args,
                }
                .into()
            };
            let params = params
                .iter()
                .enumerate()
                .map(|(i, var_type)| Param {
                    name: format!("#{i}").into(),
                    var_type: var_type.clone(),
                })
                .collect();
            check_args_with_params(&function.str, &Params(params), &args, errors);
        }
        let args = args.into_iter().map(|arg| arg.instruction).collect();
        let args = Tuple { elements: args }.into();
        Self {
            function: function.instruction,
            args,
        }
        .into()
    }
    pub fn create_from_variables(
        function: Arc<VarFunc>,
        args: Arc<[Variable]>,
    ) -> Result<Instruction, Error> {
        if function.params.len() != args.len() {
            return Err(Error::WrongNumberOfArguments(
                function
                    .ident
                    .clone()
                    .unwrap_or_else(|| Arc::from("function")),
                function.params.len(),
            ));
        }
        for (arg, param) in zip(args.iter(), function.params.iter()) {
            let arg_type = arg.as_type();
            if !arg_type.matches(&param.var_type) {
                return Err(Error::WrongArgument {
                    function: function
                        .ident
                        .clone()
                        .unwrap_or_else(|| Arc::from("function")),
                    param: param.clone(),
                    given: arg.to_string().into(),
                    given_type: arg_type,
                });
            }
        }
        let function = Variable::Function(function.clone()).into();
        let args = Variable::Tuple(args).into();
        Ok(FunctionCall { function, args }.into())
    }
}

impl BaseInstruction for FunctionCall {}

impl Exec for FunctionCall {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let function = self.function.exec(interpreter)?.into_function().unwrap();
        let args = self.args.exec(interpreter)?.into_tuple().unwrap();
        Ok(function.exec(&args)?)
    }
}

impl Recreate for FunctionCall {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let function = self.function.recreate(local_variables);
        let args = self.args.recreate(local_variables);
        Self { function, args }.into()
    }
}

impl ReturnType for FunctionCall {
    fn return_type(&self) -> Type {
        self.function.return_type().return_type().unwrap()
    }
}

fn check_args_with_params(
    ident: &Arc<str>,
    params: &Params,
    args: &[InstructionWithStr],
    errors: &mut Vec<Error>
) {
    if params.len() != args.len() {
        errors.push(Error::WrongNumberOfArguments(ident.clone(), params.len()));
    }
    for (arg, param) in zip(args, params.iter()) {
        let arg_type = arg.return_type();
        if !arg_type.matches(&param.var_type) {
            errors.push(Error::WrongArgument {
                function: ident.clone(),
                param: param.clone(),
                given: arg.str.clone(),
                given_type: arg_type,
            });
        }
    }
}
