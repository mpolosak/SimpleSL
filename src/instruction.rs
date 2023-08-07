mod array;
mod array_ops;
mod at;
mod bin;
mod block;
mod comp;
mod control_flow;
mod destruct_tuple;
mod function;
mod function_call;
mod import;
mod local_function_call;
pub mod local_variable;
mod logic;
mod math;
mod set;
mod traits;
mod tuple;
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{function_type::FunctionType, GetReturnType, GetType, Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;
use std::rc::Rc;
pub use traits::{CreateInstruction, Exec, Recreate};
use {
    array::Array,
    array_ops::{Filter, Map, TypeFilter},
    at::At,
    bin::{BinAnd, BinNot, BinOr, LShift, RShift, Xor},
    block::Block,
    comp::{Equal, Greater, GreaterOrEqual},
    control_flow::{IfElse, Match, SetIfElse},
    destruct_tuple::DestructTuple,
    function::Function,
    function_call::FunctionCall,
    import::Import,
    local_function_call::LocalFunctionCall,
    local_variable::{LocalVariable, LocalVariables},
    logic::{And, Not, Or},
    math::{Add, Divide, Modulo, Multiply, Pow, Subtract},
    set::Set,
    tuple::Tuple,
};

#[derive(Clone, Debug)]
pub enum Instruction {
    FunctionCall(FunctionCall),
    LocalFunctionCall(LocalFunctionCall),
    Variable(Variable),
    LocalVariable(Rc<str>, LocalVariable),
    Array(Array),
    Function(Function),
    Tuple(Tuple),
    Set(Box<Set>),
    DestructTuple(Box<DestructTuple>),
    Not(Box<Not>),
    BinNot(Box<BinNot>),
    Equal(Box<Equal>),
    Greater(Box<Greater>),
    GreaterOrEqual(Box<GreaterOrEqual>),
    And(Box<And>),
    Or(Box<Or>),
    Pow(Box<Pow>),
    Multiply(Box<Multiply>),
    Divide(Box<Divide>),
    Add(Box<Add>),
    Subtract(Box<Subtract>),
    Modulo(Box<Modulo>),
    BinAnd(Box<BinAnd>),
    BinOr(Box<BinOr>),
    Xor(Box<Xor>),
    LShift(Box<LShift>),
    RShift(Box<RShift>),
    Block(Block),
    IfElse(Box<IfElse>),
    At(Box<At>),
    SetIfElse(Box<SetIfElse>),
    Match(Box<Match>),
    Import(Import),
    Map(Box<Map>),
    Filter(Box<Filter>),
    TypeFilter(Box<TypeFilter>),
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
            Rule::bin_not => BinNot::create_instruction(pair, interpreter, local_variables),
            Rule::equal => Equal::create_instruction(pair, interpreter, local_variables),
            Rule::not_equal => Ok(
                match Equal::create_instruction(pair, interpreter, local_variables)? {
                    Instruction::Variable(Variable::Int(value)) => {
                        Instruction::Variable((value == 0).into())
                    }
                    instruction => Not { instruction }.into(),
                },
            ),
            Rule::greater | Rule::lower => {
                Greater::create_instruction(pair, interpreter, local_variables)
            }
            Rule::greater_equal | Rule::lower_equal => {
                GreaterOrEqual::create_instruction(pair, interpreter, local_variables)
            }
            Rule::pow => Pow::create_instruction(pair, interpreter, local_variables),
            Rule::and => And::create_instruction(pair, interpreter, local_variables),
            Rule::or => Or::create_instruction(pair, interpreter, local_variables),
            Rule::multiply => Multiply::create_instruction(pair, interpreter, local_variables),
            Rule::divide => Divide::create_instruction(pair, interpreter, local_variables),
            Rule::add => Add::create_instruction(pair, interpreter, local_variables),
            Rule::subtract => Subtract::create_instruction(pair, interpreter, local_variables),
            Rule::modulo => Modulo::create_instruction(pair, interpreter, local_variables),
            Rule::bin_and => BinAnd::create_instruction(pair, interpreter, local_variables),
            Rule::bin_or => BinOr::create_instruction(pair, interpreter, local_variables),
            Rule::xor => Xor::create_instruction(pair, interpreter, local_variables),
            Rule::rshift => RShift::create_instruction(pair, interpreter, local_variables),
            Rule::lshift => LShift::create_instruction(pair, interpreter, local_variables),
            Rule::function_call => Self::create_function_call(pair, interpreter, local_variables),
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
            Rule::function => Function::create_instruction(pair, interpreter, local_variables),
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
            _ => panic!(),
        }
    }
    fn create_function_call(
        pair: Pair<'_, Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let var_name = inner.next().unwrap().as_str();
        let args = inner
            .next()
            .unwrap()
            .into_inner()
            .map(|pair| Self::new(pair, interpreter, local_variables))
            .collect::<Result<Box<_>>>()?;
        match local_variables.get(var_name) {
            Some(LocalVariable::Function(params, return_type, ..)) => {
                Ok(LocalFunctionCall::new(var_name, params, args, return_type.clone())?.into())
            }
            Some(_) => Err(error_wrong_type(&args, var_name.into())),
            None => Ok(FunctionCall::new(var_name, interpreter, args)?.into()),
        }
    }
}

impl Exec for Instruction {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        match self {
            Self::FunctionCall(function_call) => function_call.exec(interpreter),
            Self::LocalFunctionCall(function_call) => function_call.exec(interpreter),
            Self::Variable(var) => Ok(var.clone()),
            Self::LocalVariable(name, _) => Ok(interpreter.get_variable(name).unwrap()),
            Self::Array(array) => array.exec(interpreter),
            Self::Function(function) => function.exec(interpreter),
            Self::Tuple(function) => function.exec(interpreter),
            Self::Set(set) => set.exec(interpreter),
            Self::DestructTuple(destruct_tuple) => destruct_tuple.exec(interpreter),
            Self::Not(not) => not.exec(interpreter),
            Self::BinNot(bin_not) => bin_not.exec(interpreter),
            Self::Equal(equal) => equal.exec(interpreter),
            Self::Greater(greater) => greater.exec(interpreter),
            Self::GreaterOrEqual(greater_or_equal) => greater_or_equal.exec(interpreter),
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
            Self::LocalFunctionCall(function_call) => {
                function_call.recreate(local_variables, interpreter)
            }
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
            Self::Function(function) => function.recreate(local_variables, interpreter),
            Self::Tuple(tuple) => tuple.recreate(local_variables, interpreter),
            Self::Set(set) => set.recreate(local_variables, interpreter),
            Self::DestructTuple(destruct_tuple) => {
                destruct_tuple.recreate(local_variables, interpreter)
            }
            Self::Not(not) => not.recreate(local_variables, interpreter),
            Self::BinNot(bin_not) => bin_not.recreate(local_variables, interpreter),
            Self::Equal(equal) => equal.recreate(local_variables, interpreter),
            Self::Greater(greater) => greater.recreate(local_variables, interpreter),
            Self::GreaterOrEqual(greater_or_equal) => {
                greater_or_equal.recreate(local_variables, interpreter)
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
        }
    }
}

impl GetReturnType for Instruction {
    fn get_return_type(&self) -> Type {
        match self {
            Self::Variable(variable) => variable.get_type(),
            Self::Array(array) => array.get_return_type(),
            Self::Function(function) => function.get_return_type(),
            Self::FunctionCall(function_call) => function_call.get_return_type(),
            Self::LocalFunctionCall(function_call) => function_call.get_return_type(),
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
            Self::GreaterOrEqual(greater_or_equal) => greater_or_equal.get_return_type(),
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

fn error_wrong_type(args: &[Instruction], var_name: Rc<str>) -> Error {
    let params = args.iter().map(Instruction::get_return_type).collect();
    Error::WrongType(
        var_name,
        FunctionType {
            return_type: Type::Any,
            params,
        }
        .into(),
    )
}
