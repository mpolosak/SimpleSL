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
    set::Set,
    tuple::Tuple,
};
use crate::{
    Error, ExecError,
    instruction::{ident::Ident, r#return::Return, r#struct::Struct},
    interpreter::Interpreter,
    variable::{ReturnType, Type, Typed, Variable},
};
use derive_more::From;
use r#loop::{Loop, r#for, r#while};
use pest::iterators::Pair;
use simplesl_parser::{PRATT_PARSER, Rule, unexpected};
use std::{fmt::Debug, sync::Arc};

#[derive(Debug, Clone)]
pub struct InstructionWithStr {
    pub instruction: Instruction,
    pub str: Arc<str>,
}

impl InstructionWithStr {
    pub(crate) fn new_expression(
        pair: Pair<Rule>,
        local_variables: &LocalVariables,
    ) -> Result<Self, Error> {
        PRATT_PARSER
            .map_primary(|pair| Self::create_primary(pair, local_variables))
            .map_prefix(|op, rhs| Self::create_prefix(op, rhs?))
            .map_infix(|lhs, op, rhs| Self::create_infix(op, lhs?, rhs?, local_variables))
            .map_postfix(|lhs, op| Self::create_postfix(op, lhs?, local_variables))
            .parse(pair.into_inner())
    }

    fn create_primary(
        pair: Pair<'_, Rule>,
        local_variables: &LocalVariables<'_>,
    ) -> Result<Self, Error> {
        let rule = pair.as_rule();
        if rule == Rule::expr {
            return Self::new_expression(pair, local_variables);
        }
        let str: Arc<str> = pair.as_str().into();
        let instruction = match rule {
            Rule::ident => Ident::create_instruction(&str, local_variables),
            Rule::r#true | Rule::r#false | Rule::int | Rule::float | Rule::string | Rule::void => {
                Variable::try_from(pair).map(Instruction::from)
            }
            Rule::tuple => Tuple::create_instruction(pair, local_variables),
            Rule::array => Array::create_instruction(pair, local_variables),
            Rule::array_repeat => array_repeat::create_instruction(pair, local_variables),
            Rule::function => Function::create_instruction(pair, local_variables, None),
            Rule::r#struct => Struct::create_instruction(pair, local_variables),
            Rule::r#mod => module::create_instruction(pair, local_variables),
            rule => unexpected!(rule),
        }?;
        Ok(Self { instruction, str })
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

impl From<Variable> for InstructionWithStr {
    fn from(value: Variable) -> Self {
        let str = value.to_string().into();
        Self {
            instruction: Instruction::from(value),
            str,
        }
    }
}

pub trait BaseInstruction: Debug + ReturnType + Recreate + Exec + Sync + Send {}

#[derive(Debug, Clone, From)]
pub enum Instruction {
    Break,
    Continue,
    #[from]
    Variable(Variable),
    Base(Arc<dyn BaseInstruction>),
}

impl Instruction {
    pub fn new(pair: Pair<Rule>, local_variables: &mut LocalVariables) -> Result<Self, Error> {
        match pair.as_rule() {
            Rule::set | Rule::set_expr => Set::create(pair, local_variables).map(Self::from),
            Rule::block => Block::create_instruction(pair, local_variables),
            Rule::import => import::create_instruction(pair, local_variables),
            Rule::r#if => If::create(pair, local_variables).map(Self::from),
            Rule::r#match => Match::create_instruction(pair, local_variables),
            Rule::r#return => Return::create_instruction(pair, local_variables),
            Rule::expr => {
                InstructionWithStr::new_expression(pair, local_variables).map(|iws| iws.instruction)
            }
            Rule::r#loop => Loop::create_instruction(pair, local_variables),
            Rule::r#while => r#while::create_instruction(pair, local_variables),
            Rule::r#for => r#for::create_instruction(pair, local_variables),
            Rule::r#break if local_variables.in_loop => Ok(Self::Break),
            Rule::r#break => Err(Error::BreakOutsideLoop),
            Rule::r#continue if local_variables.in_loop => Ok(Self::Continue),
            Rule::r#continue => Err(Error::ContinueOutsideLoop),
            rule => unexpected!(rule),
        }
    }
}

impl Exec for Instruction {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        match self {
            Self::Variable(var) => Ok(var.clone()),
            Self::Base(ins) => ins.exec(interpreter),
            Self::Break => Err(ExecStop::Break),
            Self::Continue => Err(ExecStop::Continue),
        }
    }
}

impl Recreate for Instruction {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        match self {
            Self::Variable(variable) => Self::Variable(variable.clone()),
            Self::Base(ins) => ins.recreate(local_variables),
            _ => self.clone(),
        }
    }
}

impl ReturnType for Instruction {
    fn return_type(&self) -> Type {
        match self {
            Self::Variable(variable) => variable.as_type(),
            Self::Base(ins) => ins.return_type(),
            Self::Break | Self::Continue => Type::Never,
        }
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
        Self::Base(Arc::from(value))
    }
}
