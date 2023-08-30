mod array;
mod array_ops;
mod array_repeat;
mod at;
mod bitwise;
mod block;
mod comp;
mod control_flow;
mod destruct_tuple;
mod function;
mod import;
pub mod local_variable;
mod logic;
mod math;
mod set;
mod traits;
mod tuple;
use self::{
    array::Array,
    array_ops::{Filter, Map, Reduce, TypeFilter},
    array_repeat::ArrayRepeat,
    at::At,
    bitwise::{BitwiseAnd, BitwiseNot, BitwiseOr, LShift, RShift, Xor},
    block::Block,
    comp::{Equal, Greater, GreaterOrEqual, Lower, LowerOrEqual},
    control_flow::{IfElse, Match, SetIfElse},
    destruct_tuple::DestructTuple,
    function::{AnonymousFunction, FunctionCall, FunctionDeclaration},
    import::Import,
    local_variable::{LocalVariable, LocalVariables},
    logic::{And, Not, Or},
    math::{Add, Divide, Modulo, Multiply, Pow, Subtract, UnaryMinus},
    set::Set,
    traits::{BaseInstruction, CreateBinOp, PrefixOp},
    tuple::Tuple,
};
use crate::{
    interpreter::Interpreter,
    parse::{Rule, PRATT_PARSER},
    variable::{GetReturnType, GetType, Type, Variable},
    Result,
};
use pest::iterators::Pair;
use std::rc::Rc;
pub(crate) use traits::{CreateInstruction, Exec, MutCreateInstruction, Recreate};

#[derive(Debug)]
pub enum Instruction {
    AnonymousFunction(AnonymousFunction),
    LocalVariable(Rc<str>, LocalVariable),
    Tuple(Tuple),
    Variable(Variable),
    Other(Box<dyn BaseInstruction>),
}

impl Instruction {
    pub(crate) fn new(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Self> {
        match pair.as_rule() {
            Rule::set => Ok(Set::new(pair, interpreter, local_variables)?.into()),
            Rule::destruct_tuple => {
                DestructTuple::create_instruction(pair, interpreter, local_variables)
            }
            Rule::block => Block::create_instruction(pair, interpreter, local_variables),
            Rule::import => Import::create_instruction(pair, interpreter, local_variables),
            Rule::if_else | Rule::if_stm => {
                IfElse::create_instruction(pair, interpreter, local_variables)
            }
            Rule::set_if_else | Rule::set_if => {
                SetIfElse::create_instruction(pair, interpreter, local_variables)
            }
            Rule::r#match => Match::create_instruction(pair, interpreter, local_variables),
            Rule::function_declaration => {
                FunctionDeclaration::create_instruction(pair, interpreter, local_variables)
            }
            Rule::expr => Self::new_expression(pair, interpreter, local_variables),
            rule => unreachable!("Unexpected rule: {rule:?}"),
        }
    }
    pub(crate) fn new_expression(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &LocalVariables,
    ) -> Result<Self> {
        PRATT_PARSER
            .map_primary(|pair| match pair.as_rule() {
                Rule::expr => Self::new_expression(pair, interpreter, local_variables),
                Rule::ident => {
                    let ident = pair.as_str();
                    local_variables.get(ident).map_or_else(
                        || interpreter.get_variable(ident).map(Instruction::from),
                        |var| match var {
                            LocalVariable::Variable(variable) => {
                                Ok(Self::Variable(variable.clone()))
                            }
                            local_variable => {
                                Ok(Self::LocalVariable(ident.into(), local_variable.clone()))
                            }
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
                rule => unreachable!("Unexpected rule: {rule:?}"),
            })
            .map_prefix(|op, rhs| match op.as_rule() {
                Rule::not => Not::create_instruction(rhs?),
                Rule::bitwise_not => BitwiseNot::create_instruction(rhs?),
                Rule::unary_minus => UnaryMinus::create_instruction(rhs?),
                rule => unreachable!("Unexpected rule: {rule:?}"),
            })
            .map_infix(|lhs, op, rhs| match op.as_rule() {
                Rule::pow => Pow::create_bin_op(lhs?, rhs?),
                Rule::multiply => Multiply::create_bin_op(lhs?, rhs?),
                Rule::add => Add::create_bin_op(lhs?, rhs?),
                Rule::subtract => Subtract::create_bin_op(lhs?, rhs?),
                Rule::divide => Divide::create_bin_op(lhs?, rhs?),
                Rule::modulo => Modulo::create_bin_op(lhs?, rhs?),
                Rule::equal => Equal::create_bin_op(lhs?, rhs?),
                Rule::lower => Lower::create_bin_op(lhs?, rhs?),
                Rule::lower_equal => LowerOrEqual::create_bin_op(lhs?, rhs?),
                Rule::greater => Greater::create_bin_op(lhs?, rhs?),
                Rule::greater_equal => GreaterOrEqual::create_bin_op(lhs?, rhs?),
                Rule::map => Map::create_bin_op(lhs?, rhs?),
                Rule::filter => Filter::create_bin_op(lhs?, rhs?),
                Rule::bitwise_and => BitwiseAnd::create_bin_op(lhs?, rhs?),
                Rule::bitwise_or => BitwiseOr::create_bin_op(lhs?, rhs?),
                Rule::xor => Xor::create_bin_op(lhs?, rhs?),
                Rule::rshift => RShift::create_bin_op(lhs?, rhs?),
                Rule::lshift => LShift::create_bin_op(lhs?, rhs?),
                Rule::and => And::create_bin_op(lhs?, rhs?),
                Rule::or => Or::create_bin_op(lhs?, rhs?),
                Rule::reduce => {
                    Reduce::create_instruction(lhs?, op, rhs?, local_variables, interpreter)
                }
                rule => unreachable!("Unexpected rule: {rule:?}"),
            })
            .map_postfix(|lhs, op| match op.as_rule() {
                Rule::at => At::create_instruction(lhs?, op, interpreter, local_variables),
                Rule::type_filter => TypeFilter::create_instruction(lhs?, op),
                Rule::function_call => {
                    FunctionCall::create_instruction(lhs?, op, interpreter, local_variables)
                }
                rule => unreachable!("Unexpected rule: {rule:?}"),
            })
            .parse(pair.into_inner())
    }
}

impl Exec for Instruction {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        match self {
            Self::Variable(var) => Ok(var.clone()),
            Self::LocalVariable(name, _) => interpreter.get_variable(name),
            Self::AnonymousFunction(function) => function.exec(interpreter),
            Self::Tuple(function) => function.exec(interpreter),
            Self::Other(other) => other.exec(interpreter),
        }
    }
}

impl Recreate for Instruction {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        match self {
            Self::LocalVariable(ident, _) => local_variables.get(ident).map_or_else(
                || interpreter.get_variable(ident).map(Instruction::from),
                |var| match var {
                    LocalVariable::Variable(variable) => Ok(Self::Variable(variable.clone())),
                    local_variable => {
                        Ok(Self::LocalVariable(ident.clone(), local_variable.clone()))
                    }
                },
            ),
            Self::AnonymousFunction(function) => function.recreate(local_variables, interpreter),
            Self::Tuple(tuple) => tuple.recreate(local_variables, interpreter),
            Self::Variable(variable) => Ok(Self::Variable(variable.clone())),
            Self::Other(other) => other.recreate(local_variables, interpreter),
        }
    }
}

impl GetReturnType for Instruction {
    fn get_return_type(&self) -> Type {
        match self {
            Self::Variable(variable) => variable.get_type(),
            Self::AnonymousFunction(function) => function.get_return_type(),
            Self::LocalVariable(_, local_variable) => local_variable.get_type(),
            Self::Tuple(tuple) => tuple.get_return_type(),
            Self::Other(other) => other.get_return_type(),
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
) -> Result<Box<[Instruction]>> {
    instructions
        .iter()
        .map(|instruction| instruction.recreate(local_variables, interpreter))
        .collect()
}

pub fn exec_instructions(
    instructions: &[Instruction],
    interpreter: &mut Interpreter,
) -> Result<Rc<[Variable]>> {
    instructions
        .iter()
        .map(|instruction| instruction.exec(interpreter))
        .collect::<Result<Rc<_>>>()
}
