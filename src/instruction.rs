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
    array_ops::{Filter, Map, TypeFilter},
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
    math::{Add, Divide, Modulo, Multiply, Pow, Subtract},
    set::Set,
    tuple::Tuple,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, GetType, Type, Variable},
    Result,
};
use pest::iterators::Pair;
use std::rc::Rc;
pub use traits::{CreateInstruction, Exec, Recreate};

#[derive(Debug)]
pub enum Instruction {
    Add(Box<Add>),
    And(Box<And>),
    AnonymousFunction(AnonymousFunction),
    Array(Array),
    ArrayRepeat(Box<ArrayRepeat>),
    At(Box<At>),
    BinAnd(Box<BitwiseAnd>),
    BinNot(Box<BitwiseNot>),
    BinOr(Box<BitwiseOr>),
    Block(Block),
    DestructTuple(Box<DestructTuple>),
    Divide(Box<Divide>),
    Equal(Box<Equal>),
    Filter(Box<Filter>),
    FunctionCall(Box<FunctionCall>),
    Greater(Box<Greater>),
    Lower(Box<Lower>),
    GreaterOrEqual(Box<GreaterOrEqual>),
    LowerOrEqual(Box<LowerOrEqual>),
    IfElse(Box<IfElse>),
    Import(Import),
    LShift(Box<LShift>),
    LocalVariable(Rc<str>, LocalVariable),
    Map(Box<Map>),
    Match(Box<Match>),
    Modulo(Box<Modulo>),
    Multiply(Box<Multiply>),
    Not(Box<Not>),
    Or(Box<Or>),
    Pow(Box<Pow>),
    RShift(Box<RShift>),
    Set(Box<Set>),
    SetIfElse(Box<SetIfElse>),
    Subtract(Box<Subtract>),
    Tuple(Tuple),
    TypeFilter(Box<TypeFilter>),
    Variable(Variable),
    Xor(Box<Xor>),
    FunctionDeclaration(FunctionDeclaration),
}

impl Instruction {
    pub fn new(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Self> {
        match pair.as_rule() {
            Rule::line => {
                let pair = pair.into_inner().next().unwrap();
                Instruction::new(pair, interpreter, local_variables)
            }
            Rule::set => Ok(Set::new(pair, interpreter, local_variables)?.into()),
            Rule::destruct_tuple => {
                DestructTuple::create_instruction(pair, interpreter, local_variables)
            }
            Rule::not => Not::create_instruction(pair, interpreter, local_variables),
            Rule::bin_not => BitwiseNot::create_instruction(pair, interpreter, local_variables),
            Rule::equal => Equal::create_instruction(pair, interpreter, local_variables),
            Rule::not_equal => Ok(
                match Equal::create_instruction(pair, interpreter, local_variables)? {
                    Instruction::Variable(Variable::Int(value)) => {
                        Instruction::Variable((value == 0).into())
                    }
                    instruction => Not { instruction }.into(),
                },
            ),
            Rule::greater => Greater::create_instruction(pair, interpreter, local_variables),
            Rule::lower => Lower::create_instruction(pair, interpreter, local_variables),
            Rule::greater_equal => {
                GreaterOrEqual::create_instruction(pair, interpreter, local_variables)
            }
            Rule::lower_equal => {
                LowerOrEqual::create_instruction(pair, interpreter, local_variables)
            }
            Rule::pow => Pow::create_instruction(pair, interpreter, local_variables),
            Rule::and => And::create_instruction(pair, interpreter, local_variables),
            Rule::or => Or::create_instruction(pair, interpreter, local_variables),
            Rule::multiply => Multiply::create_instruction(pair, interpreter, local_variables),
            Rule::divide => Divide::create_instruction(pair, interpreter, local_variables),
            Rule::add => Add::create_instruction(pair, interpreter, local_variables),
            Rule::subtract => Subtract::create_instruction(pair, interpreter, local_variables),
            Rule::modulo => Modulo::create_instruction(pair, interpreter, local_variables),
            Rule::bin_and => BitwiseAnd::create_instruction(pair, interpreter, local_variables),
            Rule::bin_or => BitwiseOr::create_instruction(pair, interpreter, local_variables),
            Rule::xor => Xor::create_instruction(pair, interpreter, local_variables),
            Rule::rshift => RShift::create_instruction(pair, interpreter, local_variables),
            Rule::lshift => LShift::create_instruction(pair, interpreter, local_variables),
            Rule::function_call => {
                FunctionCall::create_instruction(pair, interpreter, local_variables)
            }
            Rule::int | Rule::float | Rule::string | Rule::void => {
                let variable = Variable::try_from(pair).unwrap();
                Ok(Self::Variable(variable))
            }
            Rule::ident => {
                let var_name = pair.as_str();
                Ok(match local_variables.get(var_name) {
                    Some(LocalVariable::Variable(variable)) => Self::Variable(variable.clone()),
                    Some(local_variable) => {
                        Self::LocalVariable(var_name.into(), local_variable.clone())
                    }
                    None => {
                        let value = interpreter.get_variable(var_name)?;
                        Self::Variable(value)
                    }
                })
            }
            Rule::array => Array::create_instruction(pair, interpreter, local_variables),
            Rule::array_repeat => {
                ArrayRepeat::create_instruction(pair, interpreter, local_variables)
            }
            Rule::function => {
                AnonymousFunction::create_instruction(pair, interpreter, local_variables)
            }
            Rule::tuple => Tuple::create_instruction(pair, interpreter, local_variables),
            Rule::block => Block::create_instruction(pair, interpreter, local_variables),
            Rule::if_else | Rule::if_stm => {
                IfElse::create_instruction(pair, interpreter, local_variables)
            }
            Rule::at => At::create_instruction(pair, interpreter, local_variables),
            Rule::set_if_else | Rule::set_if => {
                SetIfElse::create_instruction(pair, interpreter, local_variables)
            }
            Rule::r#match => Match::create_instruction(pair, interpreter, local_variables),
            Rule::import => Import::create_instruction(pair, interpreter, local_variables),
            Rule::map => Map::create_instruction(pair, interpreter, local_variables),
            Rule::filter => Filter::create_instruction(pair, interpreter, local_variables),
            Rule::type_filter => TypeFilter::create_instruction(pair, interpreter, local_variables),
            Rule::function_declaration => {
                FunctionDeclaration::create_instruction(pair, interpreter, local_variables)
            }
            rule => panic!("Unexpected rule: {rule:?}"),
        }
    }
}

impl Exec for Instruction {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        match self {
            Self::FunctionCall(function_call) => function_call.exec(interpreter),
            Self::Variable(var) => Ok(var.clone()),
            Self::LocalVariable(name, _) => Ok(interpreter.get_variable(name).unwrap()),
            Self::Array(array) => array.exec(interpreter),
            Self::ArrayRepeat(array) => array.exec(interpreter),
            Self::AnonymousFunction(function) => function.exec(interpreter),
            Self::Tuple(function) => function.exec(interpreter),
            Self::Set(set) => set.exec(interpreter),
            Self::DestructTuple(destruct_tuple) => destruct_tuple.exec(interpreter),
            Self::Not(not) => not.exec(interpreter),
            Self::BinNot(bin_not) => bin_not.exec(interpreter),
            Self::Equal(equal) => equal.exec(interpreter),
            Self::Greater(greater) => greater.exec(interpreter),
            Self::Lower(lower) => lower.exec(interpreter),
            Self::GreaterOrEqual(greater_or_equal) => greater_or_equal.exec(interpreter),
            Self::LowerOrEqual(greater_or_equal) => greater_or_equal.exec(interpreter),
            Self::And(and) => and.exec(interpreter),
            Self::Or(or) => or.exec(interpreter),
            Self::Pow(pow) => pow.exec(interpreter),
            Self::Multiply(multiply) => multiply.exec(interpreter),
            Self::Divide(divide) => divide.exec(interpreter),
            Self::Add(add) => add.exec(interpreter),
            Self::Subtract(subtract) => subtract.exec(interpreter),
            Self::Modulo(modulo) => modulo.exec(interpreter),
            Self::BinAnd(bin_and) => bin_and.exec(interpreter),
            Self::BinOr(bin_or) => bin_or.exec(interpreter),
            Self::Xor(xor) => xor.exec(interpreter),
            Self::LShift(lshift) => lshift.exec(interpreter),
            Self::RShift(rshift) => rshift.exec(interpreter),
            Self::Block(block) => block.exec(interpreter),
            Self::IfElse(if_else) => if_else.exec(interpreter),
            Self::At(at) => at.exec(interpreter),
            Self::SetIfElse(set_if) => set_if.exec(interpreter),
            Self::Match(match_stm) => match_stm.exec(interpreter),
            Self::Import(import) => import.exec(interpreter),
            Self::Map(map) => map.exec(interpreter),
            Self::Filter(filter) => filter.exec(interpreter),
            Self::TypeFilter(filter) => filter.exec(interpreter),
            Self::FunctionDeclaration(declaration) => declaration.exec(interpreter),
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
            Self::FunctionCall(function_call) => {
                function_call.recreate(local_variables, interpreter)
            }
            Self::LocalVariable(name, var_type) => Ok(match local_variables.get(name) {
                Some(LocalVariable::Variable(variable)) => Self::Variable(variable.clone()),
                Some(_) => Self::LocalVariable(name.clone(), var_type.clone()),
                None => {
                    let variable = interpreter.get_variable(name).unwrap();
                    Self::Variable(variable)
                }
            }),
            Self::Array(array) => array.recreate(local_variables, interpreter),
            Self::ArrayRepeat(array) => array.recreate(local_variables, interpreter),
            Self::AnonymousFunction(function) => function.recreate(local_variables, interpreter),
            Self::Tuple(tuple) => tuple.recreate(local_variables, interpreter),
            Self::Set(set) => set.recreate(local_variables, interpreter),
            Self::DestructTuple(destruct_tuple) => {
                destruct_tuple.recreate(local_variables, interpreter)
            }
            Self::Not(not) => not.recreate(local_variables, interpreter),
            Self::BinNot(bin_not) => bin_not.recreate(local_variables, interpreter),
            Self::Equal(equal) => equal.recreate(local_variables, interpreter),
            Self::Greater(greater) => greater.recreate(local_variables, interpreter),
            Self::Lower(lower) => lower.recreate(local_variables, interpreter),
            Self::GreaterOrEqual(greater_or_equal) => {
                greater_or_equal.recreate(local_variables, interpreter)
            }
            Self::LowerOrEqual(lower_or_equal) => {
                lower_or_equal.recreate(local_variables, interpreter)
            }
            Self::And(and) => and.recreate(local_variables, interpreter),
            Self::Or(or) => or.recreate(local_variables, interpreter),
            Self::Pow(pow) => pow.recreate(local_variables, interpreter),
            Self::Multiply(multiply) => multiply.recreate(local_variables, interpreter),
            Self::Divide(divide) => divide.recreate(local_variables, interpreter),
            Self::Add(add) => add.recreate(local_variables, interpreter),
            Self::Subtract(subtract) => subtract.recreate(local_variables, interpreter),
            Self::Modulo(modulo) => modulo.recreate(local_variables, interpreter),
            Self::BinAnd(bin_and) => bin_and.recreate(local_variables, interpreter),
            Self::BinOr(bin_or) => bin_or.recreate(local_variables, interpreter),
            Self::Xor(xor) => xor.recreate(local_variables, interpreter),
            Self::LShift(lshift) => lshift.recreate(local_variables, interpreter),
            Self::RShift(rshift) => rshift.recreate(local_variables, interpreter),
            Self::Block(block) => block.recreate(local_variables, interpreter),
            Self::IfElse(if_else) => if_else.recreate(local_variables, interpreter),
            Self::At(at) => at.recreate(local_variables, interpreter),
            Self::Match(match_stm) => match_stm.recreate(local_variables, interpreter),
            Self::Variable(variable) => Ok(Self::Variable(variable.clone())),
            Self::SetIfElse(set_if_else) => set_if_else.recreate(local_variables, interpreter),
            Self::Import(import) => import.recreate(local_variables, interpreter),
            Self::Map(map) => map.recreate(local_variables, interpreter),
            Self::Filter(filter) => filter.recreate(local_variables, interpreter),
            Self::TypeFilter(filter) => filter.recreate(local_variables, interpreter),
            Self::FunctionDeclaration(declaration) => {
                declaration.recreate(local_variables, interpreter)
            }
        }
    }
}

impl GetReturnType for Instruction {
    fn get_return_type(&self) -> Type {
        match self {
            Self::Variable(variable) => variable.get_type(),
            Self::Array(array) => array.get_return_type(),
            Self::ArrayRepeat(array) => array.get_return_type(),
            Self::AnonymousFunction(function) => function.get_return_type(),
            Self::FunctionCall(function_call) => function_call.get_return_type(),
            Self::LocalVariable(_, local_variable) => local_variable.get_type(),
            Self::Tuple(tuple) => tuple.get_return_type(),
            Self::Set(set) => set.get_return_type(),
            Self::DestructTuple(destruct_tuple) => destruct_tuple.get_return_type(),
            Self::Pow(pow) => pow.get_return_type(),
            Self::Add(add) => add.get_return_type(),
            Self::Subtract(subtract) => subtract.get_return_type(),
            Self::Multiply(multiply) => multiply.get_return_type(),
            Self::Divide(divide) => divide.get_return_type(),
            Self::Block(block) => block.get_return_type(),
            Self::IfElse(if_else) => if_else.get_return_type(),
            Self::At(at) => at.get_return_type(),
            Self::SetIfElse(set_if) => set_if.get_return_type(),
            Self::Match(match_stm) => match_stm.get_return_type(),
            Self::Import(import) => import.get_return_type(),
            Self::Not(not) => not.get_return_type(),
            Self::BinNot(bin_not) => bin_not.get_return_type(),
            Self::Greater(greater) => greater.get_return_type(),
            Self::Lower(lower) => lower.get_return_type(),
            Self::GreaterOrEqual(greater_or_equal) => greater_or_equal.get_return_type(),
            Self::LowerOrEqual(greater_or_equal) => greater_or_equal.get_return_type(),
            Self::BinAnd(bin_and) => bin_and.get_return_type(),
            Self::BinOr(bin_or) => bin_or.get_return_type(),
            Self::Xor(xor) => xor.get_return_type(),
            Self::LShift(lshift) => lshift.get_return_type(),
            Self::RShift(rshift) => rshift.get_return_type(),
            Self::Or(or) => or.get_return_type(),
            Self::And(and) => and.get_return_type(),
            Self::Modulo(modulo) => modulo.get_return_type(),
            Self::Map(map) => map.get_return_type(),
            Self::Filter(filter) => filter.get_return_type(),
            Self::TypeFilter(filter) => filter.get_return_type(),
            Self::FunctionDeclaration(declaration) => declaration.get_return_type(),
            Self::Equal(..) => Type::Int,
        }
    }
}

impl From<Variable> for Instruction {
    fn from(value: Variable) -> Self {
        Self::Variable(value)
    }
}

pub fn recreate_instructions(
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
