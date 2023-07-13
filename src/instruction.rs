mod add;
mod and;
mod array;
mod at;
mod bin_and;
mod bin_not;
mod bin_or;
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
mod lshift;
mod modulo;
mod not;
mod or;
mod rshift;
mod set;
mod traits;
mod tuple;
mod xor;
use crate::{
    error::Error,
    interpreter::{Interpreter, VariableMap},
    parse::Rule,
    variable::Variable,
    variable::{GetReturnType, GetType, Type},
};
use pest::iterators::Pair;
use std::fmt;
pub use traits::{CreateInstruction, Exec, Recreate};
use {
    add::Add,
    and::And,
    array::Array,
    at::At,
    bin_and::BinAnd,
    bin_not::BinNot,
    bin_or::BinOr,
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
    lshift::LShift,
    modulo::Modulo,
    not::Not,
    or::Or,
    rshift::RShift,
    set::Set,
    tuple::Tuple,
    xor::Xor,
};

#[derive(Clone)]
pub enum Instruction {
    FunctionCall(FunctionCall),
    LocalFunctionCall(LocalFunctionCall),
    Variable(Variable),
    LocalVariable(String, LocalVariable),
    Array(Array),
    Function(Function),
    Tuple(Tuple),
    Set(Set),
    Not(Not),
    BinNot(BinNot),
    Equal(Equal),
    Greater(Greater),
    GreaterOrEqual(GreaterOrEqual),
    And(And),
    Or(Or),
    Multiply(Box<Instruction>, Box<Instruction>),
    Divide(Box<Instruction>, Box<Instruction>),
    Add(Add),
    Subtract(Box<Instruction>, Box<Instruction>),
    Modulo(Modulo),
    BinAnd(BinAnd),
    BinOr(BinOr),
    Xor(Xor),
    LShift(LShift),
    RShift(RShift),
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
            Rule::not => Not::create_instruction(pair, variables, local_variables),
            Rule::bin_not => BinNot::create_instruction(pair, variables, local_variables),
            Rule::equal => Equal::create_instruction(pair, variables, local_variables),
            Rule::not_equal => Ok(
                match Equal::create_instruction(pair, variables, local_variables)? {
                    Instruction::Variable(Variable::Int(value)) => {
                        Instruction::Variable((value == 0).into())
                    }
                    instruction => Not {
                        instruction: instruction.into(),
                    }
                    .into(),
                },
            ),
            Rule::greater | Rule::lower => {
                Greater::create_instruction(pair, variables, local_variables)
            }
            Rule::greater_equal | Rule::lower_equal => {
                GreaterOrEqual::create_instruction(pair, variables, local_variables)
            }
            Rule::and => And::create_instruction(pair, variables, local_variables),
            Rule::or => Or::create_instruction(pair, variables, local_variables),
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
            Rule::add => Ok(Add::create_instruction(pair, variables, local_variables)?),
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
            Rule::modulo => Modulo::create_instruction(pair, variables, local_variables),
            Rule::bin_and => BinAnd::create_instruction(pair, variables, local_variables),
            Rule::bin_or => BinOr::create_instruction(pair, variables, local_variables),
            Rule::xor => Xor::create_instruction(pair, variables, local_variables),
            Rule::lshift => RShift::create_instruction(pair, variables, local_variables),
            Rule::rshift => LShift::create_instruction(pair, variables, local_variables),
            Rule::function_call => Self::create_function_call(pair, variables, local_variables),
            Rule::int | Rule::float | Rule::string | Rule::null => {
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
                        let value = variables.get(var_name)?;
                        Self::Variable(value)
                    }
                })
            }
            Rule::array => Array::create_instruction(pair, variables, local_variables),
            Rule::function => Function::create_instruction(pair, variables, local_variables),
            Rule::tuple => Tuple::create_instruction(pair, variables, local_variables),
            Rule::block => Block::create_instruction(pair, variables, local_variables),
            Rule::if_else | Rule::if_stm => {
                IfElse::create_instruction(pair, variables, local_variables)
            }
            Rule::at => At::create_instruction(pair, variables, local_variables),
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
            Self::Tuple(function) => function.exec(interpreter, local_variables),
            Self::Set(set) => set.exec(interpreter, local_variables),
            Self::Not(not) => not.exec(interpreter, local_variables),
            Self::BinNot(bin_not) => bin_not.exec(interpreter, local_variables),
            Self::Equal(equal) => equal.exec(interpreter, local_variables),
            Self::Greater(greater) => greater.exec(interpreter, local_variables),
            Self::GreaterOrEqual(greater_or_equal) => {
                greater_or_equal.exec(interpreter, local_variables)
            }
            Self::And(and) => and.exec(interpreter, local_variables),
            Self::Or(or) => or.exec(interpreter, local_variables),
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
            Self::Modulo(modulo) => modulo.exec(interpreter, local_variables),
            Self::BinAnd(bin_and) => bin_and.exec(interpreter, local_variables),
            Self::BinOr(bin_or) => bin_or.exec(interpreter, local_variables),
            Self::Xor(xor) => xor.exec(interpreter, local_variables),
            Self::LShift(lshift) => lshift.exec(interpreter, local_variables),
            Self::RShift(rshift) => rshift.exec(interpreter, local_variables),
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
            Self::Tuple(tuple) => tuple.recreate(local_variables, args),
            Self::Set(set) => set.recreate(local_variables, args),
            Self::Not(not) => not.recreate(local_variables, args),
            Self::BinNot(bin_not) => bin_not.recreate(local_variables, args),
            Self::Equal(equal) => equal.recreate(local_variables, args),
            Self::Greater(greater) => greater.recreate(local_variables, args),
            Self::GreaterOrEqual(greater_or_equal) => {
                greater_or_equal.recreate(local_variables, args)
            }
            Self::And(and) => and.recreate(local_variables, args),
            Self::Or(or) => or.recreate(local_variables, args),
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
            Self::Modulo(modulo) => modulo.recreate(local_variables, args),
            Self::BinAnd(bin_and) => bin_and.recreate(local_variables, args),
            Self::BinOr(bin_or) => bin_or.recreate(local_variables, args),
            Self::Xor(xor) => xor.recreate(local_variables, args),
            Self::LShift(lshift) => lshift.recreate(local_variables, args),
            Self::RShift(rshift) => rshift.recreate(local_variables, args),
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
            Self::LocalVariable(_, local_variable) => local_variable.get_type(),
            Self::Tuple(tuple) => tuple.get_return_type(),
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
            | Self::Xor(..)
            | Self::LShift(..)
            | Self::RShift(..) => Type::Int,
            Self::Multiply(instruction, _)
            | Self::Divide(instruction, _)
            | Self::Subtract(instruction, _) => instruction.get_return_type(),
        }
    }
}

impl From<Variable> for Instruction {
    fn from(value: Variable) -> Self {
        Self::Variable(value)
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
