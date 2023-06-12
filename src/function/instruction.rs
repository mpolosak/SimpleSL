use super::{Function, LangFunction, Line, Param};
use crate::variable::Variable;
use crate::variable_type::{GetType, Type};
use crate::{
    error::Error,
    intepreter::{Intepreter, VariableMap},
    parse::Rule,
};
use pest::iterators::Pair;
use std::iter::zip;
use std::{collections::HashMap, fmt, rc::Rc};

#[derive(Clone)]
pub enum Instruction {
    FunctionCall(Rc<dyn Function>, Vec<Instruction>),
    LocalFunctionCall(String, Vec<Instruction>, Type),
    Variable(Variable),
    LocalVariable(String, Type),
    Array(Vec<Instruction>),
    Function(Vec<Param>, Vec<Line>),
}

impl Instruction {
    pub fn new(
        variables: &VariableMap,
        pair: Pair<Rule>,
        local_variables: &HashMap<String, Type>,
    ) -> Result<Self, Error> {
        match pair.as_rule() {
            Rule::function_call => {
                let mut inner = pair.clone().into_inner();
                let var_name = inner.next().unwrap().as_str();
                let args = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|pair| Self::new(variables, pair, local_variables))
                    .collect::<Result<Vec<_>, _>>()?;
                match local_variables.get(var_name) {
                    Some(Type::Function(return_type, _param_types, _catch_rest)) => Ok(
                        // todo: check if arguments match params
                        Self::LocalFunctionCall(String::from(var_name), args, *return_type.clone()),
                    ),
                    Some(Type::Any) => Ok(Self::LocalFunctionCall(
                        String::from(var_name),
                        args,
                        Type::Any,
                    )),
                    Some(_) => {
                        let param_types = args.iter().map(Instruction::get_type).collect();
                        Err(Error::WrongType(
                            var_name.to_owned(),
                            Type::Function(Type::Any.into(), param_types, false),
                        ))
                    }
                    None => {
                        let Variable::Function(function)
                        = variables.get(var_name)? else {
                            let param_types = args.iter().map(Instruction::get_type).collect();
                            return Err(Error::WrongType(
                                String::from(var_name),
                                Type::Function(Type::Any.into(), param_types, false)
                            ));
                        };
                        let params = function.get_params();
                        check_args(var_name, params, &args)?;
                        Ok(Self::FunctionCall(function, args))
                    }
                }
            }
            Rule::num => {
                let value = pair.as_str().parse::<f64>().unwrap();
                Ok(Self::Variable(Variable::Float(value)))
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
            Rule::string => {
                let value = pair.clone().into_inner().next().unwrap().as_str();
                let variable = Variable::String(value.into());
                Ok(Self::Variable(variable))
            }
            Rule::array => {
                let inner = pair.clone().into_inner();
                let array = inner
                    .map(|arg| Self::new(variables, arg, local_variables))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::Array(array))
            }
            Rule::function => {
                let mut inner = pair.clone().into_inner();
                let params_pair = inner.next().unwrap();
                let params: Vec<Param> = params_pair.into_inner().map(Param::from).collect();
                let mut local_variables = local_variables.clone();
                for param in &params {
                    local_variables.insert(param.get_name().to_owned(), param.get_type());
                }
                let body = inner
                    .map(|arg| Line::new(variables, arg, &mut local_variables))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::Function(params, body))
            }
            Rule::null => Ok(Self::Variable(Variable::Null)),
            _ => panic!(),
        }
    }
    pub fn exec(
        &self,
        intepreter: &mut Intepreter,
        local_variables: &VariableMap,
    ) -> Result<Variable, Error> {
        match &self {
            Self::FunctionCall(function, instructions) => {
                let args = exec_instructions(instructions, intepreter, local_variables)?;
                function.exec("name", intepreter, args)
            }
            Self::LocalFunctionCall(name, instructions, _) => {
                let args = exec_instructions(instructions, intepreter, local_variables)?;
                let Variable::Function(function)
                    = local_variables.get(name).or(
                        intepreter.variables.get(name)).unwrap() else {
                    let param_types = args.iter().map(Variable::get_type).collect();
                    return Err(
                        Error::WrongType(name.clone(), Type::Function(Type::Any.into(), param_types, false))
                    );
                };
                function.exec(name, intepreter, args)
            }
            Self::Variable(var) => Ok(var.clone()),
            Self::LocalVariable(name, _) => Ok(local_variables
                .get(name)
                .or(intepreter.variables.get(name))
                .unwrap()),
            Self::Array(instructions) => {
                let array = exec_instructions(instructions, intepreter, local_variables)?;
                Ok(Variable::Array(array.into()))
            }
            Self::Function(params, lines) => {
                let mut fn_local_variables = params
                    .iter()
                    .map(|param| (param.get_name().to_owned(), param.get_type()))
                    .collect();
                let body = recreate_lines(lines, &mut fn_local_variables, local_variables)?;
                Ok(Variable::Function(Rc::new(LangFunction {
                    params: params.clone(),
                    body,
                })))
            }
        }
    }
    pub fn recreate(
        &self,
        local_variables: &HashMap<String, Type>,
        args: &VariableMap,
    ) -> Result<Self, Error> {
        Ok(match self {
            Self::LocalFunctionCall(name, instructions, var_type) => {
                let instructions = recreate_instructions(instructions, local_variables, args)?;
                if local_variables.contains_key(name) {
                    Self::LocalFunctionCall(name.clone(), instructions, var_type.clone())
                } else {
                    let Variable::Function(function)
                        = args.get(name).unwrap() else {
                            let param_types = instructions.iter().map(Instruction::get_type).collect();
                            return Err(Error::WrongType(
                                name.clone(), Type::Function(Type::Any.into(), param_types, false)));
                    };
                    Self::FunctionCall(function, instructions)
                }
            }
            Self::FunctionCall(function, instructions) => {
                let instructions = recreate_instructions(instructions, local_variables, args)?;
                Self::FunctionCall(function.clone(), instructions)
            }
            Self::LocalVariable(name, var_type) => {
                if local_variables.contains_key(name) {
                    Self::LocalVariable(name.clone(), var_type.clone())
                } else {
                    let variable = args.get(name).unwrap();
                    Self::Variable(variable)
                }
            }
            Self::Array(instructions) => {
                let instructions = recreate_instructions(instructions, local_variables, args)?;
                Self::Array(instructions)
            }
            Self::Function(params, lines) => {
                let mut local_variables = local_variables.clone();
                for param in params {
                    local_variables.insert(param.get_name().to_owned(), param.get_type());
                }
                let new_lines = recreate_lines(lines, &mut local_variables, args)?;
                Self::Function(params.clone(), new_lines)
            }
            _ => self.clone(),
        })
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl GetType for Instruction {
    fn get_type(&self) -> Type {
        match self {
            Instruction::Variable(variable) => variable.get_type(),
            Instruction::Array(_) => Type::Array,
            Instruction::Function(params, lines) => {
                let mut param_types = Vec::new();
                let mut catch_rest = false;
                for param in params {
                    match param {
                        Param::Standard(_, param_type) => param_types.push(param_type.clone()),
                        Param::CatchRest(_) => catch_rest = true,
                    }
                }
                match lines.last() {
                    Some(Line {
                        result_var: _,
                        instruction,
                    }) => Type::Function(instruction.get_type().into(), param_types, catch_rest),
                    None => Type::Function(Box::new(Type::Null), param_types, catch_rest),
                }
            }
            Instruction::FunctionCall(function, _) => function.get_return_type(),
            Instruction::LocalFunctionCall(_, _, var_type) => var_type.clone(),
            Instruction::LocalVariable(_, var_type) => var_type.clone(),
        }
    }
}

pub fn check_args(
    var_name: &str,
    params: &Vec<Param>,
    args: &Vec<Instruction>,
) -> Result<(), Error> {
    match params.last() {
        Some(Param::CatchRest(_)) if args.len() < params.len() - 1 => {
            return Err(Error::WrongNumberOfArguments(
                String::from(var_name),
                params.len() - 1,
            ))
        }
        None if !args.is_empty() => {
            return Err(Error::WrongNumberOfArguments(String::from(var_name), 0))
        }
        _ => (),
    }
    for (arg, param) in zip(args, params) {
        match param {
            Param::Standard(name, var_type) if !arg.get_type().matches(var_type) => {
                return Err(Error::WrongType(name.clone(), var_type.clone()));
            }
            _ => (),
        }
    }
    Ok(())
}

pub fn recreate_instructions(
    instructions: &[Instruction],
    local_variables: &HashMap<String, Type>,
    args: &VariableMap,
) -> Result<Vec<Instruction>, Error> {
    instructions
        .iter()
        .map(|instruction| instruction.recreate(local_variables, args))
        .collect()
}

pub fn recreate_lines(
    lines: &[Line],
    local_variables: &mut HashMap<String, Type>,
    args: &VariableMap,
) -> Result<Vec<Line>, Error> {
    lines
        .iter()
        .map(|line| line.recreate(local_variables, args))
        .collect::<Result<Vec<Line>, Error>>()
}

pub fn exec_instructions(
    instructions: &[Instruction],
    intepreter: &mut Intepreter,
    local_variables: &VariableMap,
) -> Result<Vec<Variable>, Error> {
    instructions
        .iter()
        .map(|instruction| instruction.exec(intepreter, local_variables))
        .collect::<Result<Vec<_>, _>>()
}
