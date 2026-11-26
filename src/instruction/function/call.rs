use crate::{
    Error, Interpreter,
    errors::{
        CannotDetermineParams, ErrorWithRange, NotAFunction, WrongArgument, WrongNumberOfArguments,
    },
    function::{Function as VarFunc, Param, Params},
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, InstructionWithRange, Recreate,
        local_variable::LocalVariables, panic::Panic, parsing_context::ParsingContext,
        tuple::Tuple,
    },
    range_from_2_ranges, range_from_pest_span,
    variable::{ReturnType, Type, Typed, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::{iter::zip, ops::Range, sync::Arc};

#[derive(Debug)]
pub struct FunctionCall {
    pub function: Instruction,
    pub args: Instruction,
}

impl FunctionCall {
    pub fn create_instruction(
        function: InstructionWithRange,
        args: Pair<Rule>,
        parsing_context: &ParsingContext,
        errors: &mut Vec<ErrorWithRange>,
    ) -> Instruction {
        let args_span = range_from_pest_span(&args.as_span());
        let span = range_from_2_ranges(&function.range, &args_span);
        let args = args
            .into_inner()
            .map(|pair| InstructionWithRange::new_expression(pair, parsing_context, errors))
            .collect::<Box<_>>();

        let ident = parsing_context.slice(function.range.clone()).into();
        if let Some(Variable::Function(function2)) = function.instruction.downcast_ref::<Variable>()
        {
            check_args_with_params(
                span,
                &ident,
                &function2.params,
                &args,
                parsing_context,
                errors,
            );
        } else {
            let f_type = function.return_type();
            if !f_type.is_function() {
                let error = NotAFunction { ins: ident.clone() }.into();
                let range = function.range;
                errors.push(ErrorWithRange { error, range });
                return Panic.into();
            }
            let Some(params) = f_type.params() else {
                let error = CannotDetermineParams { ident }.into();
                let span = function.range;
                errors.push(ErrorWithRange { error, range: span });
                let args = Tuple {
                    elements: [].into(),
                }
                .into();
                return Self {
                    function: function.instruction,
                    args,
                }
                .into();
            };
            let params = params
                .iter()
                .enumerate()
                .map(|(i, var_type)| Param {
                    name: format!("#{i}").into(),
                    var_type: var_type.clone(),
                })
                .collect();
            check_args_with_params(
                span,
                &ident,
                &Params(params),
                &args,
                parsing_context,
                errors,
            );
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
            return Err(WrongNumberOfArguments {
                function: function
                    .ident
                    .clone()
                    .unwrap_or_else(|| Arc::from("function")),
                expected_arguments: function.params.len(),
            }
            .into());
        }
        for (arg, param) in zip(args.iter(), function.params.iter()) {
            let arg_type = arg.as_type();
            if !arg_type.matches(&param.var_type) {
                return Err(WrongArgument {
                    function: function
                        .ident
                        .clone()
                        .unwrap_or_else(|| Arc::from("function")),
                    param: param.clone(),
                    given: arg.to_string().into(),
                    given_type: arg_type,
                }
                .into());
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
        self.function
            .return_type()
            .return_type()
            .unwrap_or(Type::Never)
    }
}

fn check_args_with_params(
    range: Range<usize>,
    ident: &Arc<str>,
    params: &Params,
    args: &[InstructionWithRange],
    parsing_context: &ParsingContext,
    errors: &mut Vec<ErrorWithRange>,
) {
    if params.len() != args.len() {
        let error = WrongNumberOfArguments {
            function: ident.clone(),
            expected_arguments: params.len(),
        }
        .into();
        errors.push(ErrorWithRange { error, range });
    }
    for (arg, param) in zip(args, params.iter()) {
        let arg_type = arg.return_type();
        if !arg_type.matches(&param.var_type) {
            let range = arg.range.clone();
            let given = parsing_context.slice(range.clone()).into();
            let error = WrongArgument {
                function: ident.clone(),
                param: param.clone(),
                given,
                given_type: arg_type,
            }
            .into();
            errors.push(ErrorWithRange { error, range });
        }
    }
}
