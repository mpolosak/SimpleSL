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
pub mod pattern;
pub mod postfix_op;
mod prefix_op;
mod reduce;
pub mod r#return;
mod set;
mod slicing;
mod r#struct;
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
    instruction::{
        ident::Ident,
        r#loop::{Break, Continue, r#for::For},
        r#return::Return,
        r#struct::Struct,
    },
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable, from_str::variable_from_pair},
};
use derive_more::From;
use r#loop::{Loop, r#while};
use pest::iterators::Pair;
use simplesl_parser::{PRATT_PARSER, Rule, unexpected};
use std::{any::Any, fmt::Debug, sync::Arc};

#[derive(Debug, Clone)]
pub struct InstructionWithStr {
    pub instruction: Instruction,
    pub str: Arc<str>,
}

impl InstructionWithStr {
    pub(crate) fn new_expression(
        pair: Pair<Rule>,
        local_variables: &LocalVariables,
        errors: &mut Vec<Error>,
    ) -> Self {
        let mut prefix_errors = Vec::new();
        let mut infix_errors = Vec::new();
        let mut postfix_errors = Vec::new();

        let result = PRATT_PARSER
            .map_primary(|pair| Self::create_primary(pair, local_variables, errors))
            .map_prefix(|op, rhs| Self::create_prefix(op, rhs, &mut prefix_errors))
            .map_infix(|lhs, op, rhs| {
                Self::create_infix(op, lhs, rhs, local_variables, &mut infix_errors)
            })
            .map_postfix(|lhs, op| {
                Self::create_postfix(op, lhs, local_variables, &mut postfix_errors)
            })
            .parse(pair.into_inner());
        errors.append(&mut prefix_errors);
        errors.append(&mut infix_errors);
        errors.append(&mut postfix_errors);
        result
    }

    fn create_primary(
        pair: Pair<'_, Rule>,
        local_variables: &LocalVariables<'_>,
        errors: &mut Vec<Error>,
    ) -> Self {
        let rule = pair.as_rule();
        if rule == Rule::expr {
            return Self::new_expression(pair, local_variables, errors);
        }
        let str: Arc<str> = pair.as_str().into();
        let instruction: Instruction = match rule {
            Rule::ident => Ident::create_instruction(&str, local_variables, errors),
            Rule::r#true | Rule::r#false | Rule::int | Rule::float | Rule::void | Rule::string => {
                variable_from_pair(pair, errors).into()
            }
            Rule::tuple => Tuple::create_instruction(pair, local_variables, errors),
            Rule::array => Array::create_instruction(pair, local_variables, errors),
            Rule::array_repeat => array_repeat::create_instruction(pair, local_variables, errors),
            Rule::function => Function::create_instruction(pair, local_variables, None, errors),
            Rule::r#struct => Struct::create_instruction(pair, local_variables, errors),
            Rule::r#mod => module::create_instruction(pair, local_variables, errors),
            rule => unexpected!(rule),
        };
        Self { instruction, str }
    }
}

impl Exec for InstructionWithStr {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        self.instruction.exec(interpreter)
    }
}

impl ReturnType for InstructionWithStr {
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
        local_variables: &mut LocalVariables,
        errors: &mut Vec<Error>,
    ) -> Self {
        match pair.as_rule() {
            Rule::set | Rule::set_expr => Set::create(pair, local_variables, errors).into(),
            Rule::set_else => SetElse::create(pair, local_variables, errors),
            Rule::block => Block::create_instruction(pair, local_variables, errors),
            Rule::import => import::create_instruction(pair, local_variables, errors),
            Rule::r#if => If::create(pair, local_variables, errors).into(),
            Rule::r#match => Match::create_instruction(pair, local_variables, errors),
            Rule::r#return => Return::create_instruction(pair, local_variables, errors),
            Rule::expr => {
                InstructionWithStr::new_expression(pair, local_variables, errors).instruction
            }
            Rule::r#loop => Loop::create_instruction(pair, local_variables, errors),
            Rule::r#while => r#while::create_instruction(pair, local_variables, errors),
            Rule::r#for => For::create_instruction(pair, local_variables, errors),
            Rule::r#break => {
                if !local_variables.in_loop {
                    errors.push(Error::BreakOutsideLoop);
                }
                Break.into()
            }
            Rule::r#continue => {
                if !local_variables.in_loop {
                    errors.push(Error::ContinueOutsideLoop);
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
