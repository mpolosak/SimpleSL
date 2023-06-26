mod array;
mod check_args;
mod function;
mod function_call;
mod local_function_call;
pub mod local_variable;
mod set;
mod traits;
use self::array::Array;
use self::function::Function;
use self::function_call::FunctionCall;
use self::local_function_call::LocalFunctionCall;
use self::local_variable::{LocalVariable, LocalVariableMap};
use self::set::Set;
pub use self::traits::{Exec, Recreate};
use crate::variable::Variable;
use crate::variable_type::{GetReturnType, GetType, Type};
use crate::{
    error::Error,
    interpreter::{Interpreter, VariableMap},
    parse::Rule,
};
use check_args::check_args;
use pest::iterators::Pair;
use std::fmt;

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
    Equal(Box<Instruction>, Box<Instruction>),
    Greater(Box<Instruction>, Box<Instruction>),
    GreaterOrEqual(Box<Instruction>, Box<Instruction>),
    And(Box<Instruction>, Box<Instruction>),
    Or(Box<Instruction>, Box<Instruction>),
}

impl Instruction {
    pub fn new(
        variables: &VariableMap,
        pair: Pair<Rule>,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Self, Error> {
        match pair.as_rule() {
            Rule::line => {
                let pair = pair.into_inner().next().unwrap();
                Instruction::new(variables, pair, local_variables)
            }
            Rule::set => Ok(Set::new(variables, pair, local_variables)?.into()),
            Rule::not => {
                let pair = pair.into_inner().next().unwrap();
                let instruction = Instruction::new(variables, pair, local_variables)?;
                if instruction.get_return_type() == Type::Int {
                    Ok(Self::Not(instruction.into()))
                } else {
                    Err(Error::WrongType(
                        String::from("Variable after '!' operator"),
                        Type::Int,
                    ))
                }
            }
            Rule::bin_not => {
                let pair = pair.into_inner().next().unwrap();
                let instruction = Instruction::new(variables, pair, local_variables)?;
                if instruction.get_return_type() == Type::Int {
                    Ok(Self::BinNot(instruction.into()))
                } else {
                    Err(Error::WrongType(
                        String::from("Variable after '~' operator"),
                        Type::Int,
                    ))
                }
            }
            Rule::equal => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let instruction = Instruction::new(variables, pair, local_variables)?;
                let pair = inner.next().unwrap();
                let instruction2 = Instruction::new(variables, pair, local_variables)?;
                Ok(Self::Equal(instruction.into(), instruction2.into()))
            }
            Rule::not_equal => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let instruction = Instruction::new(variables, pair, local_variables)?;
                let pair = inner.next().unwrap();
                let instruction2 = Instruction::new(variables, pair, local_variables)?;
                Ok(Self::Not(
                    Self::Equal(instruction.into(), instruction2.into()).into(),
                ))
            }
            Rule::greater => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let instruction = Instruction::new(variables, pair, local_variables)?;
                let pair = inner.next().unwrap();
                let instruction2 = Instruction::new(variables, pair, local_variables)?;
                match (
                    instruction.get_return_type(),
                    instruction2.get_return_type(),
                ) {
                    (Type::Int, Type::Int) | (Type::Float, Type::Float) => {
                        Ok(Self::Greater(instruction.into(), instruction2.into()))
                    }
                    _ => Err(Error::Other(String::from(
                        "Arguments of > operator must be both int or both float",
                    ))),
                }
            }
            Rule::greater_equal => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let instruction = Instruction::new(variables, pair, local_variables)?;
                let pair = inner.next().unwrap();
                let instruction2 = Instruction::new(variables, pair, local_variables)?;
                match (
                    instruction.get_return_type(),
                    instruction2.get_return_type(),
                ) {
                    (Type::Int, Type::Int) | (Type::Float, Type::Float) => Ok(
                        Self::GreaterOrEqual(instruction.into(), instruction2.into()),
                    ),
                    _ => Err(Error::Other(String::from(
                        "Arguments of >= operator must be both int or both float",
                    ))),
                }
            }
            Rule::lower => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let instruction = Instruction::new(variables, pair, local_variables)?;
                let pair = inner.next().unwrap();
                let instruction2 = Instruction::new(variables, pair, local_variables)?;
                match (
                    instruction.get_return_type(),
                    instruction2.get_return_type(),
                ) {
                    (Type::Int, Type::Int) | (Type::Float, Type::Float) => {
                        Ok(Self::Greater(instruction2.into(), instruction.into()))
                    }
                    _ => Err(Error::Other(String::from(
                        "Arguments of < operator must be both int or both float",
                    ))),
                }
            }
            Rule::lower_equal => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let instruction = Instruction::new(variables, pair, local_variables)?;
                let pair = inner.next().unwrap();
                let instruction2 = Instruction::new(variables, pair, local_variables)?;
                match (
                    instruction.get_return_type(),
                    instruction2.get_return_type(),
                ) {
                    (Type::Int, Type::Int) | (Type::Float, Type::Float) => Ok(
                        Self::GreaterOrEqual(instruction2.into(), instruction.into()),
                    ),
                    _ => Err(Error::Other(String::from(
                        "Arguments of <= operator must be both int or both float",
                    ))),
                }
            }
            Rule::and => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let instruction = Instruction::new(variables, pair, local_variables)?;
                let pair = inner.next().unwrap();
                let instruction2 = Instruction::new(variables, pair, local_variables)?;
                match (
                    instruction.get_return_type(),
                    instruction2.get_return_type(),
                ) {
                    (Type::Int, Type::Int) => {
                        Ok(Self::And(instruction2.into(), instruction.into()))
                    }
                    _ => Err(Error::WrongType(String::from("Argument of && "), Type::Int)),
                }
            }
            Rule::or => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let instruction = Instruction::new(variables, pair, local_variables)?;
                let pair = inner.next().unwrap();
                let instruction2 = Instruction::new(variables, pair, local_variables)?;
                match (
                    instruction.get_return_type(),
                    instruction2.get_return_type(),
                ) {
                    (Type::Int, Type::Int) => Ok(Self::Or(instruction2.into(), instruction.into())),
                    _ => Err(Error::WrongType(String::from("Argument of || "), Type::Int)),
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
            Rule::array => Ok(Array::new(variables, pair, local_variables)?.into()),
            Rule::function => Ok(Function::new(pair, local_variables, variables)?.into()),
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
            .map(|pair| Self::new(variables, pair, local_variables))
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
            Self::Equal(instruction1, instruction2) => {
                let result1 = instruction1.exec(interpreter, local_variables)?;
                let result2 = instruction2.exec(interpreter, local_variables)?;
                Ok((result1 == result2).into())
            }
            Self::Greater(instruction1, instruction2) => {
                let result1 = instruction1.exec(interpreter, local_variables)?;
                let result2 = instruction2.exec(interpreter, local_variables)?;
                match (result1, result2) {
                    (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 > value2).into()),
                    (Variable::Float(value1), Variable::Float(value2)) => {
                        Ok((value1 > value2).into())
                    }
                    _ => panic!(),
                }
            }
            Self::GreaterOrEqual(instruction1, instruction2) => {
                let result1 = instruction1.exec(interpreter, local_variables)?;
                let result2 = instruction2.exec(interpreter, local_variables)?;
                match (result1, result2) {
                    (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 >= value2).into()),
                    (Variable::Float(value1), Variable::Float(value2)) => {
                        Ok((value1 >= value2).into())
                    }
                    _ => panic!(),
                }
            }
            Self::And(instruction1, instruction2) => {
                let result1 = instruction1.exec(interpreter, local_variables)?;
                let result2 = instruction2.exec(interpreter, local_variables)?;
                match (result1, result2) {
                    (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 * value2).into()),
                    _ => panic!(),
                }
            }
            Self::Or(instruction1, instruction2) => {
                let result1 = instruction1.exec(interpreter, local_variables)?;
                let result2 = instruction2.exec(interpreter, local_variables)?;
                match (result1, result2) {
                    (Variable::Int(value1), Variable::Int(value2)) => {
                        Ok((value1 != 0 || value2 != 0).into())
                    }
                    _ => panic!(),
                }
            }
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
            Self::Equal(instruction1, instruction2) => {
                let instruction1 = instruction1.recreate(local_variables, args);
                let instruction2 = instruction2.recreate(local_variables, args);
                Self::Equal(instruction1.into(), instruction2.into())
            }
            Self::Greater(instruction1, instruction2) => {
                let instruction1 = instruction1.recreate(local_variables, args);
                let instruction2 = instruction2.recreate(local_variables, args);
                Self::Greater(instruction1.into(), instruction2.into())
            }
            Self::GreaterOrEqual(instruction1, instruction2) => {
                let instruction1 = instruction1.recreate(local_variables, args);
                let instruction2 = instruction2.recreate(local_variables, args);
                Self::GreaterOrEqual(instruction1.into(), instruction2.into())
            }
            Self::And(instruction1, instruction2) => {
                let instruction1 = instruction1.recreate(local_variables, args);
                let instruction2 = instruction2.recreate(local_variables, args);
                Self::GreaterOrEqual(instruction1.into(), instruction2.into())
            }
            Self::Or(instruction1, instruction2) => {
                let instruction1 = instruction1.recreate(local_variables, args);
                let instruction2 = instruction2.recreate(local_variables, args);
                Self::GreaterOrEqual(instruction1.into(), instruction2.into())
            }
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
            Self::Array(_) => Type::Array,
            Self::Function(function) => function.get_return_type(),
            Self::FunctionCall(function_call) => function_call.get_return_type(),
            Self::LocalFunctionCall(function_call) => function_call.get_return_type(),
            Self::LocalVariable(_, var_type) => var_type.clone().into(),
            Self::Set(set) => set.get_return_type(),
            Self::Not(_)
            | Self::BinNot(_)
            | Self::Equal(..)
            | Self::Greater(..)
            | Self::GreaterOrEqual(..)
            | Self::And(..)
            | Self::Or(..) => Type::Int,
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
    intepreter: &mut Interpreter,
    local_variables: &mut VariableMap,
) -> Result<Vec<Variable>, Error> {
    instructions
        .iter()
        .map(|instruction| instruction.exec(intepreter, local_variables))
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
