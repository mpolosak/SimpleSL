mod array;
mod array_repeat;
pub mod at;
mod bin_op;
mod block;
mod control_flow;
mod destruct_tuple;
pub mod function;
mod import;
pub mod local_variable;
mod r#mut;
mod prefix_op;
mod reduce;
mod r#return;
mod return_type;
mod set;
mod tuple;
mod type_filter;
mod unary_operation;
mod r#while;
use self::{
    array::Array,
    array_repeat::ArrayRepeat,
    bin_op::*,
    block::Block,
    control_flow::{IfElse, Match, SetIfElse},
    destruct_tuple::DestructTuple,
    function::{AnonymousFunction, FunctionDeclaration},
    import::Import,
    local_variable::{LocalVariable, LocalVariables},
    set::Set,
    tuple::Tuple,
};
use crate::{
    interpreter::Interpreter,
    variable::{ReturnType, Type, Typed, Variable},
    Error, ExecError,
};
use duplicate::duplicate_item;
use match_any::match_any;
use pest::iterators::Pair;
use r#mut::Mut;
use r#while::While;
use reduce::Reduce;
use simplesl_parser::{unexpected, Rule, PRATT_PARSER};
use std::sync::Arc;
use type_filter::TypeFilter;
use unary_operation::UnaryOperation;

#[derive(Debug, Clone)]
pub struct InstructionWithStr {
    pub instruction: Instruction,
    pub str: Arc<str>,
}

impl InstructionWithStr {
    pub fn new(pair: Pair<Rule>, local_variables: &mut LocalVariables) -> Result<Self, Error> {
        if pair.as_rule() == Rule::expr {
            return Self::new_expression(pair, local_variables);
        }
        let str = pair.as_str().into();
        let instruction = Instruction::new(pair, local_variables)?;
        Ok(Self { instruction, str })
    }

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
            Rule::ident => local_variables.get(&str).map_or_else(
                || {
                    local_variables
                        .interpreter
                        .get_variable(&str)
                        .cloned()
                        .map(Instruction::from)
                        .ok_or_else(|| Error::VariableDoesntExist(str.clone()))
                },
                |var| match var.clone() {
                    LocalVariable::Variable(variable) => Ok(Instruction::Variable(variable)),
                    local_variable => Ok(Instruction::LocalVariable(str.clone(), local_variable)),
                },
            ),
            Rule::r#true | Rule::r#false | Rule::int | Rule::float | Rule::string | Rule::void => {
                Variable::try_from(pair).map(Instruction::from)
            }
            Rule::r#mut => Mut::create_instruction(pair, local_variables),
            Rule::tuple => Tuple::create_instruction(pair, local_variables),
            Rule::array => Array::create_instruction(pair, local_variables),
            Rule::array_repeat => ArrayRepeat::create_instruction(pair, local_variables),
            Rule::function => AnonymousFunction::create_instruction(pair, local_variables),
            rule => unexpected!(rule),
        }?;
        Ok(Self { instruction, str })
    }

    pub fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Self, ExecError> {
        let instruction = self.instruction.recreate(local_variables)?;
        let str = self.str.clone();
        Ok(Self { instruction, str })
    }

    pub fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(Instruction) -> Instruction,
    {
        Self {
            instruction: f(self.instruction),
            str: self.str,
        }
    }
    pub fn try_map<F, E>(self, f: F) -> Result<Self, E>
    where
        F: FnOnce(Instruction) -> Result<Instruction, E>,
    {
        let instruction = f(self.instruction)?;
        Ok(Self {
            instruction,
            str: self.str,
        })
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

#[derive(Debug, Clone)]
pub enum Instruction {
    AnonymousFunction(AnonymousFunction),
    Array(Arc<Array>),
    ArrayRepeat(Arc<ArrayRepeat>),
    Block(Block),
    DestructTuple(Arc<DestructTuple>),
    FunctionDeclaration(Arc<FunctionDeclaration>),
    IfElse(Arc<IfElse>),
    Import(Import),
    LocalVariable(Arc<str>, LocalVariable),
    Match(Arc<Match>),
    Mut(Arc<Mut>),
    Reduce(Arc<Reduce>),
    Set(Arc<Set>),
    SetIfElse(Arc<SetIfElse>),
    Tuple(Tuple),
    TypeFilter(Arc<TypeFilter>),
    Variable(Variable),
    BinOperation(Arc<BinOperation>),
    UnaryOperation(Arc<UnaryOperation>),
    While(Arc<While>),
}

impl Instruction {
    pub fn new(pair: Pair<Rule>, local_variables: &mut LocalVariables) -> Result<Self, Error> {
        match pair.as_rule() {
            Rule::set => Set::create_instruction(pair, local_variables),
            Rule::destruct_tuple => DestructTuple::create_instruction(pair, local_variables),
            Rule::block => Block::create_instruction(pair, local_variables),
            Rule::import => Import::create_instruction(pair, local_variables),
            Rule::if_else => IfElse::create_instruction(pair, local_variables),
            Rule::set_if_else => SetIfElse::create_instruction(pair, local_variables),
            Rule::r#match => Match::create_instruction(pair, local_variables),
            Rule::function_declaration => {
                FunctionDeclaration::create_instruction(pair, local_variables)
            }
            Rule::r#return => r#return::create(pair, local_variables),
            Rule::expr => {
                InstructionWithStr::new_expression(pair, local_variables).map(|iws| iws.instruction)
            }
            Rule::r#while => While::create_instruction(pair, local_variables),
            rule => unexpected!(rule),
        }
    }
}

impl Exec for Instruction {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        match_any! { self,
            Self::Variable(var) => Ok(var.clone()),
            Self::LocalVariable(ident, _) => interpreter
                .get_variable(ident)
                .cloned()
                .ok_or_else(|| panic!("Tried to get variable {ident} that doest exist")),
            Self::AnonymousFunction(ins) | Self::Array(ins) | Self::ArrayRepeat(ins)
            | Self::Block(ins) | Self::DestructTuple(ins) | Self::Tuple(ins)
            | Self::BinOperation(ins) | Self::FunctionDeclaration(ins) | Self::IfElse(ins)
            | Self::Import(ins) | Self::Match(ins) | Self::Mut(ins) | Self::Reduce(ins)
            | Self::Set(ins) | Self::SetIfElse(ins) | Self::TypeFilter(ins)
            | Self::UnaryOperation(ins) | Self::While(ins)
            => ins.exec(interpreter)
        }
    }
}

impl Recreate for Instruction {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        match_any! {self,
            Self::LocalVariable(ident, _) => Ok(local_variables.get(ident).map_or_else(
                || {
                    local_variables.interpreter
                        .get_variable(ident)
                        .cloned()
                        .map(Instruction::from)
                        .unwrap_or_else(|| panic!("Tried to get variable {ident} that doest exist"))
                },
                |var| match var.clone() {
                    LocalVariable::Variable(variable) => Self::Variable(variable),
                    local_variable => Self::LocalVariable(ident.clone(), local_variable),
                },
            )),
            Self::Variable(variable) => Ok(Self::Variable(variable.clone())),
            Self::AnonymousFunction(ins) | Self::Array(ins) | Self::ArrayRepeat(ins)
            | Self::Block(ins) | Self::DestructTuple(ins) | Self::Tuple(ins)
            | Self::BinOperation(ins) | Self::FunctionDeclaration(ins) | Self::IfElse(ins)
            | Self::Import(ins) | Self::Match(ins) | Self::Mut(ins) | Self::Reduce(ins)
            | Self::Set(ins) | Self::SetIfElse(ins) | Self::TypeFilter(ins)
            | Self::UnaryOperation(ins) | Self::While(ins)
            => ins.recreate(local_variables)
        }
    }
}

impl ReturnType for Instruction {
    fn return_type(&self) -> Type {
        match_any! { self,
            Self::Variable(variable) | Self::LocalVariable(_, variable) => variable.as_type(),
            Self::AnonymousFunction(ins) | Self::Array(ins) | Self::ArrayRepeat(ins)
            | Self::Block(ins) | Self::DestructTuple(ins) | Self::Tuple(ins)
            | Self::BinOperation(ins) | Self::FunctionDeclaration(ins) | Self::IfElse(ins)
            | Self::Import(ins) | Self::Match(ins) | Self::Mut(ins) | Self::Reduce(ins)
            | Self::Set(ins) | Self::SetIfElse(ins) | Self::TypeFilter(ins)
            | Self::UnaryOperation(ins)
            => ins.return_type(),
            _ => Type::Void
        }
    }
}

#[duplicate_item(
    T; [Block]; [Variable]; [UnaryOperation]; [BinOperation]; [AnonymousFunction]; [Array];
    [ArrayRepeat]; [Tuple]; [DestructTuple]; [FunctionDeclaration]; [IfElse]; [Import]; [Reduce];
    [TypeFilter]; [Match]; [Mut]; [Set]; [SetIfElse];
)]
impl From<T> for Instruction {
    fn from(value: T) -> Self {
        #[allow(clippy::useless_conversion)]
        Self::T(value.into())
    }
}

pub(crate) fn recreate_instructions(
    instructions: &[InstructionWithStr],
    local_variables: &mut LocalVariables,
) -> Result<Arc<[InstructionWithStr]>, ExecError> {
    instructions
        .iter()
        .map(|iws| iws.recreate(local_variables))
        .collect()
}

pub trait Recreate {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError>;
}

pub trait Exec {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult;
}

pub type ExecResult = Result<Variable, ExecStop>;
pub enum ExecStop {
    Return(Variable),
    Error(ExecError),
}

impl From<ExecError> for ExecStop {
    fn from(value: ExecError) -> Self {
        Self::Error(value)
    }
}
