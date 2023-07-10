mod add;
mod array;
mod at;
mod block;
mod check_args;
mod equal;
mod function;
mod function_call;
mod greater;
mod greater_or_equal;
mod if_else;
mod local_function_call;
pub mod local_variable;
mod set;
mod traits;
use crate::{
    error::Error,
    interpreter::{Interpreter, VariableMap},
    parse::Rule,
    variable::Variable,
    variable::{GetReturnType, GetType, Type},
};
use pest::iterators::Pair;
use std::fmt;
pub use traits::{Exec, Recreate};

use {
    add::Add,
    array::Array,
    at::At,
    block::Block,
    check_args::check_args,
    equal::Equal,
    function::Function,
    function_call::FunctionCall,
    greater::Greater,
    greater_or_equal::GreaterOrEqual,
    if_else::IfElse,
    local_function_call::LocalFunctionCall,
    local_variable::{LocalVariable, LocalVariableMap},
    set::Set,
};

#[derive(Clone)]
pub enum Instruction {
    FunctionCall(FunctionCall),
    LocalFunctionCall(LocalFunctionCall),
    Variable(Variable),
    LocalVariable(String, LocalVariable),
    Array(Array),
    Function(Function),
    Set(Set),
    Not(Box<Instruction>),
    BinNot(Box<Instruction>),
    Equal(Equal),
    Greater(Greater),
    GreaterOrEqual(GreaterOrEqual),
    And(Box<Instruction>, Box<Instruction>),
    Or(Box<Instruction>, Box<Instruction>),
    Multiply(Box<Instruction>, Box<Instruction>),
    Divide(Box<Instruction>, Box<Instruction>),
    Add(Add),
    Subtract(Box<Instruction>, Box<Instruction>),
    Modulo(Box<Instruction>, Box<Instruction>),
    BinAnd(Box<Instruction>, Box<Instruction>),
    BinOr(Box<Instruction>, Box<Instruction>),
    XOR(Box<Instruction>, Box<Instruction>),
    LShift(Box<Instruction>, Box<Instruction>),
    RShift(Box<Instruction>, Box<Instruction>),
    Block(Block),
    IfElse(IfElse),
    At(At),
}

impl Instruction {
    pub fn new(
        pair: Pair<Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Self, Error> {
        match pair.as_rule() {
            Rule::line => {
                let pair = pair.into_inner().next().unwrap();
                Instruction::new(pair, variables, local_variables)
            }
            Rule::set => Ok(Set::new(pair, variables, local_variables)?.into()),
            Rule::not => {
                let pair = pair.into_inner().next().unwrap();
                let instruction = Instruction::new(pair, variables, local_variables)?;
                if instruction.get_return_type() == Type::Int {
                    Ok(Self::Not(instruction.into()))
                } else {
                    Err(Error::OperandMustBeInt("!"))
                }
            }
            Rule::bin_not => {
                let pair = pair.into_inner().next().unwrap();
                let instruction = Instruction::new(pair, variables, local_variables)?;
                if instruction.get_return_type() == Type::Int {
                    Ok(Self::BinNot(instruction.into()))
                } else {
                    Err(Error::OperandMustBeInt("~"))
                }
            }
            Rule::equal => Ok(Equal::new(pair, variables, local_variables)?.into()),
            Rule::not_equal => Ok(Self::Not(Box::new(
                Equal::new(pair, variables, local_variables)?.into(),
            ))),
            Rule::greater | Rule::lower => {
                Ok(Greater::new(pair, variables, local_variables)?.into())
            }
            Rule::greater_equal | Rule::lower_equal => {
                Ok(GreaterOrEqual::new(pair, variables, local_variables)?.into())
            }
            Rule::and => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let lhs = Instruction::new(pair, variables, local_variables)?;
                let pair = inner.next().unwrap();
                let rhs = Instruction::new(pair, variables, local_variables)?;
                match (lhs.get_return_type(), rhs.get_return_type()) {
                    (Type::Int, Type::Int) => Ok(Self::And(lhs.into(), rhs.into())),
                    _ => Err(Error::BothOperandsMustBeInt("&&")),
                }
            }
            Rule::or => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let lhs = Instruction::new(pair, variables, local_variables)?;
                let pair = inner.next().unwrap();
                let rhs = Instruction::new(pair, variables, local_variables)?;
                match (lhs.get_return_type(), rhs.get_return_type()) {
                    (Type::Int, Type::Int) => Ok(Self::Or(lhs.into(), rhs.into())),
                    _ => Err(Error::BothOperandsMustBeInt("||")),
                }
            }
            Rule::multiply => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let lhs = Instruction::new(pair, variables, local_variables)?;
                let pair = inner.next().unwrap();
                let rhs = Instruction::new(pair, variables, local_variables)?;
                match (lhs.get_return_type(), rhs.get_return_type()) {
                    (Type::Int, Type::Int) | (Type::Float, Type::Float) => {
                        Ok(Self::Multiply(lhs.into(), rhs.into()))
                    }
                    _ => Err(Error::OperandsMustBeBothIntOrBothFloat("*")),
                }
            }
            Rule::divide => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let lhs = Instruction::new(pair, variables, local_variables)?;
                let pair = inner.next().unwrap();
                let rhs = Instruction::new(pair, variables, local_variables)?;
                match (lhs.get_return_type(), rhs.get_return_type()) {
                    (Type::Int, Type::Int) | (Type::Float, Type::Float) => {
                        Ok(Self::Divide(lhs.into(), rhs.into()))
                    }
                    _ => Err(Error::OperandsMustBeBothIntOrBothFloat("/")),
                }
            }
            Rule::add => Ok(Add::new(pair, variables, local_variables)?.into()),
            Rule::subtract => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let lhs = Instruction::new(pair, variables, local_variables)?;
                let pair = inner.next().unwrap();
                let rhs = Instruction::new(pair, variables, local_variables)?;
                match (lhs.get_return_type(), rhs.get_return_type()) {
                    (Type::Int, Type::Int) | (Type::Float, Type::Float) => {
                        Ok(Self::Subtract(lhs.into(), rhs.into()))
                    }
                    _ => Err(Error::OperandsMustBeBothIntOrBothFloat("-")),
                }
            }
            Rule::modulo => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let lhs = Instruction::new(pair, variables, local_variables)?;
                let pair = inner.next().unwrap();
                let rhs = Instruction::new(pair, variables, local_variables)?;
                match (lhs.get_return_type(), rhs.get_return_type()) {
                    (Type::Int, Type::Int) => Ok(Self::Modulo(lhs.into(), rhs.into())),
                    _ => Err(Error::BothOperandsMustBeInt("%")),
                }
            }
            Rule::bin_and => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let lhs = Instruction::new(pair, variables, local_variables)?;
                let pair = inner.next().unwrap();
                let rhs = Instruction::new(pair, variables, local_variables)?;
                match (lhs.get_return_type(), rhs.get_return_type()) {
                    (Type::Int, Type::Int) => Ok(Self::BinAnd(lhs.into(), rhs.into())),
                    _ => Err(Error::BothOperandsMustBeInt("&")),
                }
            }
            Rule::bin_or => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let lhs = Instruction::new(pair, variables, local_variables)?;
                let pair = inner.next().unwrap();
                let rhs = Instruction::new(pair, variables, local_variables)?;
                match (lhs.get_return_type(), rhs.get_return_type()) {
                    (Type::Int, Type::Int) => Ok(Self::BinOr(lhs.into(), rhs.into())),
                    _ => Err(Error::BothOperandsMustBeInt("|")),
                }
            }
            Rule::xor => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let lhs = Instruction::new(pair, variables, local_variables)?;
                let pair = inner.next().unwrap();
                let rhs = Instruction::new(pair, variables, local_variables)?;
                match (lhs.get_return_type(), rhs.get_return_type()) {
                    (Type::Int, Type::Int) => Ok(Self::XOR(lhs.into(), rhs.into())),
                    _ => Err(Error::BothOperandsMustBeInt("^")),
                }
            }
            Rule::lshift => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let lhs = Instruction::new(pair, variables, local_variables)?;
                let pair = inner.next().unwrap();
                let rhs = Instruction::new(pair, variables, local_variables)?;
                match (lhs.get_return_type(), rhs.get_return_type()) {
                    (Type::Int, Type::Int) => Ok(Self::LShift(lhs.into(), rhs.into())),
                    _ => Err(Error::BothOperandsMustBeInt("<<")),
                }
            }
            Rule::rshift => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let lhs = Instruction::new(pair, variables, local_variables)?;
                let pair = inner.next().unwrap();
                let rhs = Instruction::new(pair, variables, local_variables)?;
                match (lhs.get_return_type(), rhs.get_return_type()) {
                    (Type::Int, Type::Int) => Ok(Self::RShift(lhs.into(), rhs.into())),
                    _ => Err(Error::BothOperandsMustBeInt(">>")),
                }
            }
            Rule::function_call => Self::create_function_call(pair, variables, local_variables),
            Rule::int | Rule::float | Rule::string | Rule::null => {
                let variable = Variable::try_from(pair).unwrap();
                Ok(Self::Variable(variable))
            }
            Rule::ident => {
                let var_name = pair.as_str();
                if let Some(var_type) = local_variables.get(var_name) {
                    Ok(Self::LocalVariable(var_name.to_string(), var_type.clone()))
                } else {
                    let value = variables.get(var_name)?;
                    Ok(Self::Variable(value))
                }
            }
            Rule::array => Ok(Array::new(pair, variables, local_variables)?.into()),
            Rule::function => Ok(Function::new(pair, local_variables, variables)?.into()),
            Rule::block => Ok(Block::new(pair, local_variables, variables)?.into()),
            Rule::if_else | Rule::if_stm => {
                Ok(IfElse::new(pair, variables, local_variables)?.into())
            }
            Rule::at => Ok(At::new(pair, local_variables, variables)?.into()),
            _ => panic!(),
        }
    }
    fn create_function_call(
        pair: Pair<'_, Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let var_name = inner.next().unwrap().as_str();
        let args = inner
            .next()
            .unwrap()
            .into_inner()
            .map(|pair| Self::new(pair, variables, local_variables))
            .collect::<Result<Vec<_>, _>>()?;
        match local_variables.get(var_name) {
            Some(LocalVariable::Function(params, return_type, ..)) => {
                Ok(LocalFunctionCall::new(var_name, params, args, return_type.clone())?.into())
            }
            Some(_) => Err(error_wrong_type(&args, var_name)),
            None => Ok(FunctionCall::new(var_name, variables, args)?.into()),
        }
    }
}

impl Exec for Instruction {
    fn exec(
        &self,
        interpreter: &mut Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<Variable, Error> {
        match self {
            Self::FunctionCall(function_call) => function_call.exec(interpreter, local_variables),
            Self::LocalFunctionCall(function_call) => {
                function_call.exec(interpreter, local_variables)
            }
            Self::Variable(var) => Ok(var.clone()),
            Self::LocalVariable(name, _) => Ok(local_variables.get(name).unwrap()),
            Self::Array(array) => array.exec(interpreter, local_variables),
            Self::Function(function) => function.exec(interpreter, local_variables),
            Self::Set(set) => set.exec(interpreter, local_variables),
            Self::Not(instruction) => {
                let Variable::Int(result) = instruction.exec(interpreter, local_variables)? else {
                    panic!()
                };
                Ok((result == 0).into())
            }
            Self::BinNot(instruction) => {
                let Variable::Int(result) = instruction.exec(interpreter, local_variables)? else {
                    panic!()
                };
                Ok(Variable::Int(!result))
            }
            Self::Equal(equal) => equal.exec(interpreter, local_variables),
            Self::Greater(greater) => greater.exec(interpreter, local_variables),
            Self::GreaterOrEqual(greater_or_equal) => {
                greater_or_equal.exec(interpreter, local_variables)
            }
            Self::And(lhs, rhs) => {
                let result1 = lhs.exec(interpreter, local_variables)?;
                let result2 = rhs.exec(interpreter, local_variables)?;
                match (result1, result2) {
                    (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 * value2).into()),
                    _ => panic!(),
                }
            }
            Self::Or(lhs, rhs) => {
                let result1 = lhs.exec(interpreter, local_variables)?;
                let result2 = rhs.exec(interpreter, local_variables)?;
                match (result1, result2) {
                    (Variable::Int(value1), Variable::Int(value2)) => {
                        Ok((value1 != 0 || value2 != 0).into())
                    }
                    _ => panic!(),
                }
            }
            Self::Multiply(lhs, rhs) => {
                let result1 = lhs.exec(interpreter, local_variables)?;
                let result2 = rhs.exec(interpreter, local_variables)?;
                match (result1, result2) {
                    (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 * value2).into()),
                    (Variable::Float(value1), Variable::Float(value2)) => {
                        Ok((value1 * value2).into())
                    }
                    _ => panic!(),
                }
            }
            Self::Divide(lhs, rhs) => {
                let result1 = lhs.exec(interpreter, local_variables)?;
                let result2 = rhs.exec(interpreter, local_variables)?;
                match (result1, result2) {
                    (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 / value2).into()),
                    (Variable::Float(value1), Variable::Float(value2)) => {
                        Ok((value1 / value2).into())
                    }
                    _ => panic!(),
                }
            }
            Self::Add(add) => add.exec(interpreter, local_variables),
            Self::Subtract(lhs, rhs) => {
                let result1 = lhs.exec(interpreter, local_variables)?;
                let result2 = rhs.exec(interpreter, local_variables)?;
                match (result1, result2) {
                    (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 - value2).into()),
                    (Variable::Float(value1), Variable::Float(value2)) => {
                        Ok((value1 - value2).into())
                    }
                    _ => panic!(),
                }
            }
            Self::Modulo(lhs, rhs) => {
                let result1 = lhs.exec(interpreter, local_variables)?;
                let result2 = rhs.exec(interpreter, local_variables)?;
                match (result1, result2) {
                    (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 % value2).into()),
                    _ => panic!(),
                }
            }
            Self::BinAnd(lhs, rhs) => {
                let result1 = lhs.exec(interpreter, local_variables)?;
                let result2 = rhs.exec(interpreter, local_variables)?;
                match (result1, result2) {
                    (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 & value2).into()),
                    _ => panic!(),
                }
            }
            Self::BinOr(lhs, rhs) => {
                let result1 = lhs.exec(interpreter, local_variables)?;
                let result2 = rhs.exec(interpreter, local_variables)?;
                match (result1, result2) {
                    (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 | value2).into()),
                    _ => panic!(),
                }
            }
            Self::XOR(lhs, rhs) => {
                let result1 = lhs.exec(interpreter, local_variables)?;
                let result2 = rhs.exec(interpreter, local_variables)?;
                match (result1, result2) {
                    (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 ^ value2).into()),
                    _ => panic!(),
                }
            }
            Self::LShift(lhs, rhs) => {
                let result1 = lhs.exec(interpreter, local_variables)?;
                let result2 = rhs.exec(interpreter, local_variables)?;
                match (result1, result2) {
                    (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 << value2).into()),
                    _ => panic!(),
                }
            }
            Self::RShift(lhs, rhs) => {
                let result1 = lhs.exec(interpreter, local_variables)?;
                let result2 = rhs.exec(interpreter, local_variables)?;
                match (result1, result2) {
                    (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 >> value2).into()),
                    _ => panic!(),
                }
            }
            Self::Block(block) => block.exec(interpreter, local_variables),
            Self::IfElse(if_else) => if_else.exec(interpreter, local_variables),
            Self::At(at) => at.exec(interpreter, local_variables),
        }
    }
}

impl Recreate for Instruction {
    fn recreate(self, local_variables: &mut LocalVariableMap, args: &VariableMap) -> Instruction {
        match self {
            Self::LocalFunctionCall(function_call) => function_call.recreate(local_variables, args),
            Self::FunctionCall(function_call) => function_call.recreate(local_variables, args),
            Self::LocalVariable(name, var_type) => {
                if local_variables.contains_key(&name) {
                    Self::LocalVariable(name, var_type)
                } else {
                    let variable = args.get(&name).unwrap();
                    Self::Variable(variable)
                }
            }
            Self::Array(array) => array.recreate(local_variables, args),
            Self::Function(function) => function.recreate(local_variables, args),
            Self::Set(set) => set.recreate(local_variables, args),
            Self::Not(instruction) => {
                let instruction = instruction.recreate(local_variables, args);
                Self::Not(instruction.into())
            }
            Self::BinNot(instruction) => {
                let instruction = instruction.recreate(local_variables, args);
                Self::BinNot(instruction.into())
            }
            Self::Equal(equal) => equal.recreate(local_variables, args),
            Self::Greater(greater) => greater.recreate(local_variables, args),
            Self::GreaterOrEqual(greater_or_equal) => {
                greater_or_equal.recreate(local_variables, args)
            }
            Self::And(lhs, rhs) => {
                let lhs = lhs.recreate(local_variables, args);
                let rhs = rhs.recreate(local_variables, args);
                Self::And(lhs.into(), rhs.into())
            }
            Self::Or(lhs, rhs) => {
                let lhs = lhs.recreate(local_variables, args);
                let rhs = rhs.recreate(local_variables, args);
                Self::Or(lhs.into(), rhs.into())
            }
            Self::Multiply(lhs, rhs) => {
                let lhs = lhs.recreate(local_variables, args);
                let rhs = rhs.recreate(local_variables, args);
                Self::Multiply(lhs.into(), rhs.into())
            }
            Self::Divide(lhs, rhs) => {
                let lhs = lhs.recreate(local_variables, args);
                let rhs = rhs.recreate(local_variables, args);
                Self::Divide(lhs.into(), rhs.into())
            }
            Self::Add(add) => add.recreate(local_variables, args),
            Self::Subtract(lhs, rhs) => {
                let lhs = lhs.recreate(local_variables, args);
                let rhs = rhs.recreate(local_variables, args);
                Self::Subtract(lhs.into(), rhs.into())
            }
            Self::Modulo(lhs, rhs) => {
                let lhs = lhs.recreate(local_variables, args);
                let rhs = rhs.recreate(local_variables, args);
                Self::Modulo(lhs.into(), rhs.into())
            }
            Self::BinAnd(lhs, rhs) => {
                let lhs = lhs.recreate(local_variables, args);
                let rhs = rhs.recreate(local_variables, args);
                Self::BinAnd(lhs.into(), rhs.into())
            }
            Self::BinOr(lhs, rhs) => {
                let lhs = lhs.recreate(local_variables, args);
                let rhs = rhs.recreate(local_variables, args);
                Self::BinOr(lhs.into(), rhs.into())
            }
            Self::XOR(lhs, rhs) => {
                let lhs = lhs.recreate(local_variables, args);
                let rhs = rhs.recreate(local_variables, args);
                Self::XOR(lhs.into(), rhs.into())
            }
            Self::LShift(lhs, rhs) => {
                let lhs = lhs.recreate(local_variables, args);
                let rhs = rhs.recreate(local_variables, args);
                Self::XOR(lhs.into(), rhs.into())
            }
            Self::RShift(lhs, rhs) => {
                let lhs = lhs.recreate(local_variables, args);
                let rhs = rhs.recreate(local_variables, args);
                Self::XOR(lhs.into(), rhs.into())
            }
            Self::Block(block) => block.recreate(local_variables, args),
            Self::IfElse(if_else) => if_else.recreate(local_variables, args),
            Self::At(at) => at.recreate(local_variables, args),
            _ => self,
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
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
            Self::LocalVariable(_, var_type) => var_type.clone().into(),
            Self::Set(set) => set.get_return_type(),
            Self::Add(add) => add.get_return_type(),
            Self::Block(block) => block.get_return_type(),
            Self::IfElse(if_else) => if_else.get_return_type(),
            Self::At(at) => at.get_return_type(),
            Self::Not(_)
            | Self::BinNot(_)
            | Self::Equal(..)
            | Self::Greater(..)
            | Self::GreaterOrEqual(..)
            | Self::And(..)
            | Self::Or(..)
            | Self::Modulo(..)
            | Self::BinAnd(..)
            | Self::BinOr(..)
            | Self::XOR(..)
            | Self::LShift(..)
            | Self::RShift(..) => Type::Int,
            Self::Multiply(instruction, _)
            | Self::Divide(instruction, _)
            | Self::Subtract(instruction, _) => instruction.get_return_type(),
        }
    }
}

pub fn recreate_instructions(
    instructions: Vec<Instruction>,
    local_variables: &mut LocalVariableMap,
    args: &VariableMap,
) -> Vec<Instruction> {
    instructions
        .into_iter()
        .map(|instruction| instruction.recreate(local_variables, args))
        .collect()
}

pub fn exec_instructions(
    instructions: &[Instruction],
    interpreter: &mut Interpreter,
    local_variables: &mut VariableMap,
) -> Result<Vec<Variable>, Error> {
    instructions
        .iter()
        .map(|instruction| instruction.exec(interpreter, local_variables))
        .collect::<Result<Vec<_>, _>>()
}

fn error_wrong_type(args: &[Instruction], var_name: &str) -> Error {
    let params = args.iter().map(Instruction::get_return_type).collect();
    Error::WrongType(
        var_name.to_owned(),
        Type::Function {
            return_type: Type::Any.into(),
            params,
            catch_rest: false,
        },
    )
}
