mod array;
mod array_ops;
mod array_repeat;
mod at;
mod bin_op;
mod bitwise;
mod block;
mod control_flow;
mod destruct_tuple;
mod equal;
mod function;
mod import;
pub mod local_variable;
mod logic;
mod math;
pub mod ord;
mod prefix_op;
mod r#return;
mod return_type;
mod set;
mod shift;
mod traits;
mod tuple;
use self::{
    array::Array,
    array_ops::{Filter, Map, Reduce, TypeFilter},
    array_repeat::ArrayRepeat,
    at::At,
    bitwise::{BitwiseAnd, BitwiseOr, Xor},
    block::Block,
    control_flow::{IfElse, Match, SetIfElse},
    destruct_tuple::DestructTuple,
    equal::Equal,
    function::{AnonymousFunction, FunctionDeclaration},
    import::Import,
    local_variable::{LocalVariable, LocalVariables},
    logic::{And, Or},
    math::{Add, Divide, Modulo, Multiply, Pow, Subtract},
    ord::{Greater, GreaterOrEqual, Lower, LowerOrEqual},
    r#return::Return,
    set::Set,
    shift::{LShift, RShift},
    traits::BaseInstruction,
    tuple::Tuple,
};
use crate::{
    interpreter::Interpreter,
    parse::{unexpected, Rule, PRATT_PARSER},
    variable::{ReturnType, Type, Typed, Variable},
    Error, ExecError,
};
pub(crate) use function::FunctionCall;
use match_any::match_any;
use pest::iterators::Pair;
use std::rc::Rc;
pub(crate) use traits::{
    CreateInstruction, Exec, ExecResult, ExecStop, MutCreateInstruction, Recreate,
};

#[derive(Debug, Clone)]
pub enum Instruction {
    AnonymousFunction(AnonymousFunction),
    LocalVariable(Rc<str>, LocalVariable),
    Tuple(Tuple),
    Variable(Variable),
    Other(Rc<dyn BaseInstruction>),
}

impl Instruction {
    pub(crate) fn new(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Self, Error> {
        match pair.as_rule() {
            Rule::set => Set::create_instruction(pair, interpreter, local_variables),
            Rule::destruct_tuple => {
                DestructTuple::create_instruction(pair, interpreter, local_variables)
            }
            Rule::block => Block::create_instruction(pair, interpreter, local_variables),
            Rule::import => Import::create_instruction(pair, interpreter, local_variables),
            Rule::if_else => IfElse::create_instruction(pair, interpreter, local_variables),
            Rule::set_if_else => SetIfElse::create_instruction(pair, interpreter, local_variables),
            Rule::r#match => Match::create_instruction(pair, interpreter, local_variables),
            Rule::function_declaration => {
                FunctionDeclaration::create_instruction(pair, interpreter, local_variables)
            }
            Rule::r#return => Return::create_instruction(pair, interpreter, local_variables),
            Rule::expr => Self::new_expression(pair, interpreter, local_variables),
            rule => unexpected(rule),
        }
    }
    pub(crate) fn new_expression(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &LocalVariables,
    ) -> Result<Self, Error> {
        PRATT_PARSER
            .map_primary(|pair| Self::create_primary(pair, interpreter, local_variables))
            .map_prefix(|op, rhs| Self::create_prefix(op, rhs?))
            .map_infix(|lhs, op, rhs| {
                Self::create_infix(op, lhs?, rhs?, local_variables, interpreter)
            })
            .map_postfix(|lhs, op| Self::create_postfix(op, lhs?, interpreter, local_variables))
            .parse(pair.into_inner())
    }

    fn create_primary(
        pair: Pair<'_, Rule>,
        interpreter: &Interpreter<'_>,
        local_variables: &LocalVariables<'_>,
    ) -> Result<Self, Error> {
        match pair.as_rule() {
            Rule::expr => Self::new_expression(pair, interpreter, local_variables),
            Rule::ident => {
                let ident = pair.as_str();
                local_variables.get(ident).map_or_else(
                    || {
                        interpreter
                            .get_variable(ident)
                            .cloned()
                            .map(Instruction::from)
                            .ok_or_else(|| Error::VariableDoesntExist(ident.into()))
                    },
                    |var| match var.clone() {
                        LocalVariable::Variable(variable) => Ok(Self::Variable(variable)),
                        local_variable => Ok(Self::LocalVariable(ident.into(), local_variable)),
                    },
                )
            }
            Rule::int | Rule::float | Rule::string | Rule::void => {
                Variable::try_from(pair).map(Instruction::from)
            }
            Rule::tuple => Tuple::create_instruction(pair, interpreter, local_variables),
            Rule::array => Array::create_instruction(pair, interpreter, local_variables),
            Rule::array_repeat => {
                ArrayRepeat::create_instruction(pair, interpreter, local_variables)
            }
            Rule::function => {
                AnonymousFunction::create_instruction(pair, interpreter, local_variables)
            }
            rule => unexpected(rule),
        }
    }

    fn create_infix(
        op: Pair<'_, Rule>,
        lhs: Self,
        rhs: Self,
        local_variables: &LocalVariables<'_>,
        interpreter: &Interpreter<'_>,
    ) -> Result<Self, Error> {
        match op.as_rule() {
            Rule::pow => Pow::create_op(lhs, rhs),
            Rule::multiply => Multiply::create_op(lhs, rhs),
            Rule::add => Add::create_op(lhs, rhs),
            Rule::subtract => Subtract::create_op(lhs, rhs),
            Rule::divide => Divide::create_op(lhs, rhs),
            Rule::modulo => Modulo::create_op(lhs, rhs),
            Rule::equal => Equal::create_op(lhs, rhs),
            Rule::lower => Lower::create_op(lhs, rhs),
            Rule::lower_equal => LowerOrEqual::create_op(lhs, rhs),
            Rule::greater => Greater::create_op(lhs, rhs),
            Rule::greater_equal => GreaterOrEqual::create_op(lhs, rhs),
            Rule::map => Map::create_op(lhs, rhs),
            Rule::filter => Filter::create_op(lhs, rhs),
            Rule::bitwise_and => BitwiseAnd::create_op(lhs, rhs),
            Rule::bitwise_or => BitwiseOr::create_op(lhs, rhs),
            Rule::xor => Xor::create_op(lhs, rhs),
            Rule::rshift => RShift::create_op(lhs, rhs),
            Rule::lshift => LShift::create_op(lhs, rhs),
            Rule::and => And::create_op(lhs, rhs),
            Rule::or => Or::create_op(lhs, rhs),
            Rule::reduce => Reduce::create_instruction(lhs, op, rhs, local_variables, interpreter),
            rule => unexpected(rule),
        }
    }

    fn create_postfix(
        op: Pair<'_, Rule>,
        lhs: Self,
        interpreter: &Interpreter<'_>,
        local_variables: &LocalVariables<'_>,
    ) -> Result<Self, Error> {
        match op.as_rule() {
            Rule::at => At::create_instruction(lhs, op, interpreter, local_variables),
            Rule::type_filter => {
                TypeFilter::create_instruction(lhs, op.into_inner().next().unwrap())
            }
            Rule::function_call => {
                FunctionCall::create_instruction(lhs, op, interpreter, local_variables)
            }
            rule => unexpected(rule),
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
            Self::AnonymousFunction(ins) | Self::Tuple(ins) | Self::Other(ins)
                => ins.exec(interpreter)
        }
    }
}

impl Recreate for Instruction {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, ExecError> {
        match_any! {self,
            Self::LocalVariable(ident, _) => Ok(local_variables.get(ident).map_or_else(
                || {
                    interpreter
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
            Self::AnonymousFunction(ins) | Self::Tuple(ins) | Self::Other(ins)
                => ins.recreate(local_variables, interpreter)
        }
    }
}

impl ReturnType for Instruction {
    fn return_type(&self) -> Type {
        match_any! { self,
            Self::Variable(variable) | Self::LocalVariable(_, variable) => variable.as_type(),
            Self::AnonymousFunction(ins) | Self::Tuple(ins) | Self::Other(ins) => ins.return_type()
        }
    }
}

impl From<Variable> for Instruction {
    fn from(value: Variable) -> Self {
        Self::Variable(value)
    }
}

pub(crate) fn recreate_instructions(
    instructions: &[Instruction],
    local_variables: &mut LocalVariables,
    interpreter: &Interpreter,
) -> Result<Rc<[Instruction]>, ExecError> {
    instructions
        .iter()
        .map(|instruction| instruction.recreate(local_variables, interpreter))
        .collect()
}
