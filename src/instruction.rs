mod array;
mod array_repeat;
pub mod at;
mod bin_op;
pub mod block;
mod control_flow;
mod field_access;
pub mod function;
mod ident;
mod import;
pub mod local_variable;
mod r#loop;
mod module;
mod r#mut;
pub mod panic;
pub mod parsing_context;
pub mod pattern;
pub mod postfix_op;
mod prefix_op;
mod reduce;
pub mod r#return;
mod set;
mod slicing;
mod r#struct;
mod tagged;
mod tuple;
pub mod tuple_access;
mod type_filter;
use self::{
    array::Array,
    bin_op::*,
    block::Block,
    control_flow::{If, Match},
    function::Function,
    local_variable::LocalVariables,
    set::{Set, SetElse},
    tuple::Tuple,
};
use crate::{
    Error, ExecError,
    errors::ErrorWithSpan,
    instruction::{
        ident::Ident,
        r#loop::{Break, Continue, r#for::For},
        parsing_context::ParsingContext,
        r#return::Return,
        r#struct::Struct,
    },
    interpreter::Interpreter,
    source_span_from_pest_span,
    variable::{ReturnType, Type, Variable, from_str::variable_from_pair},
};
use derive_more::From;
use r#loop::{Loop, r#while};
use miette::SourceSpan;
use pest::iterators::Pair;
use simplesl_parser::{PRATT_PARSER, Rule, unexpected};
use std::{any::Any, fmt::Debug, sync::Arc};

#[derive(Debug, Clone)]
pub struct InstructionWithSpan {
    pub instruction: Instruction,
    pub span: SourceSpan,
}

impl InstructionWithSpan {
    pub(crate) fn new_expression(
        pair: Pair<'_, Rule>,
        parsing_context: &ParsingContext,
        errors: &mut Vec<ErrorWithSpan>,
    ) -> Self {
        let mut prefix_errors = Vec::new();
        let mut infix_errors = Vec::new();
        let mut postfix_errors = Vec::new();

        let result = PRATT_PARSER
            .map_primary(|pair| Self::create_primary(pair, parsing_context, errors))
            .map_prefix(|op, rhs| Self::create_prefix(op, rhs, parsing_context, &mut prefix_errors))
            .map_infix(|lhs, op, rhs| {
                Self::create_infix(op, lhs, rhs, parsing_context, &mut infix_errors)
            })
            .map_postfix(|lhs, op| {
                Self::create_postfix(op, lhs, parsing_context, &mut postfix_errors)
            })
            .parse(pair.into_inner());
        errors.append(&mut prefix_errors);
        errors.append(&mut infix_errors);
        errors.append(&mut postfix_errors);
        result
    }

    fn create_primary(
        pair: Pair<'_, Rule>,
        parsing_context: &ParsingContext,
        errors: &mut Vec<ErrorWithSpan>,
    ) -> Self {
        let rule = pair.as_rule();
        if rule == Rule::expr {
            return Self::new_expression(pair, parsing_context, errors);
        }
        let span = source_span_from_pest_span(&pair.as_span());
        let instruction: Instruction = match rule {
            Rule::ident => Ident::create_instruction(span, parsing_context, errors),
            Rule::r#true | Rule::r#false | Rule::int | Rule::float | Rule::string => {
                variable_from_pair(pair, errors).into()
            }
            Rule::tuple => Tuple::create_instruction(pair, parsing_context, errors),
            Rule::array => Array::create_instruction(pair, parsing_context, errors),
            Rule::array_repeat => array_repeat::create_instruction(pair, parsing_context, errors),
            Rule::function => Function::create_instruction(pair, parsing_context, None, errors),
            Rule::r#struct => Struct::create_instruction(pair, parsing_context, errors),
            Rule::tagged => tagged::Tagged::create_instruction(pair, parsing_context, errors),
            Rule::r#mod => module::create_instruction(pair, parsing_context, errors),
            rule => unexpected!(rule),
        };
        Self { instruction, span }
    }
}

impl ReturnType for InstructionWithSpan {
    fn return_type(&self) -> Type {
        self.instruction.return_type()
    }
}

pub trait BaseInstruction: Debug + ReturnType + Recreate + Exec + Sync + Send {}

#[derive(Debug, Clone, From)]
pub struct Instruction(Arc<dyn BaseInstruction>);

impl Instruction {
    pub fn new(
        pair: Pair<Rule>,
        parsing_context: &mut ParsingContext,
        errors: &mut Vec<ErrorWithSpan>,
    ) -> Self {
        match pair.as_rule() {
            Rule::set | Rule::set_expr => Set::create(pair, parsing_context, errors).into(),
            Rule::set_else => SetElse::create(pair, parsing_context, errors),
            Rule::block => Block::create_instruction(pair, parsing_context, errors),
            Rule::import => import::create_instruction(pair, parsing_context, errors),
            Rule::r#if => If::create(pair, parsing_context, errors).into(),
            Rule::r#match => Match::create_instruction(pair, parsing_context, errors),
            Rule::r#return => Return::create_instruction(pair, parsing_context, errors),
            Rule::expr => {
                InstructionWithSpan::new_expression(pair, parsing_context, errors).instruction
            }
            Rule::r#loop => Loop::create_instruction(pair, parsing_context, errors),
            Rule::r#while => r#while::create_instruction(pair, parsing_context, errors),
            Rule::r#for => For::create_instruction(pair, parsing_context, errors),
            Rule::r#break => {
                if !parsing_context.local_variables.in_loop {
                    let span = source_span_from_pest_span(&pair.as_span());
                    errors.push(ErrorWithSpan {
                        error: Error::BreakOutsideLoop,
                        span,
                    });
                }
                Break.into()
            }
            Rule::r#continue => {
                if !parsing_context.local_variables.in_loop {
                    let span = source_span_from_pest_span(&pair.as_span());
                    errors.push(ErrorWithSpan {
                        error: Error::ContinueOutsideLoop,
                        span,
                    });
                }
                Continue.into()
            }
            rule => unexpected!(rule),
        }
    }

    fn downcast_ref<T: BaseInstruction + 'static>(&self) -> Option<&T> {
        let value_any = &self.0 as &dyn Any;
        value_any.downcast_ref::<T>()
    }
}

impl Exec for Instruction {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        self.0.exec(interpreter)
    }
}

impl Recreate for Instruction {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        self.0.recreate(local_variables)
    }
}

impl ReturnType for Instruction {
    fn return_type(&self) -> Type {
        self.0.return_type()
    }
}

pub(crate) fn recreate_instructions(
    instructions: &[Instruction],
    local_variables: &mut LocalVariables,
) -> Box<[Instruction]> {
    instructions
        .iter()
        .map(|i| i.recreate(local_variables))
        .collect()
}

pub trait Recreate {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction;
}

pub trait Exec {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult;
}

pub type ExecResult = Result<Variable, ExecStop>;
pub enum ExecStop {
    Break,
    Continue,
    Return(Variable),
    Error(ExecError),
}

impl From<ExecError> for ExecStop {
    fn from(value: ExecError) -> Self {
        Self::Error(value)
    }
}

impl<T: BaseInstruction + 'static> From<T> for Instruction {
    fn from(value: T) -> Self {
        Self(Arc::from(value))
    }
}
