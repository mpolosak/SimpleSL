use super::{Function, LangFunction, Line, Param};
use crate::variable::Variable;
use crate::{
    error::Error,
    intepreter::{Intepreter, VariableMap},
    parse::Rule,
};
use pest::iterators::Pair;
use std::{collections::HashSet, fmt, rc::Rc};

#[derive(Clone)]
pub enum Instruction {
    FunctionCall(Rc<dyn Function>, Vec<Instruction>),
    LocalFunctionCall(String, Vec<Instruction>),
    Variable(Variable),
    LocalVariable(String),
    Array(Vec<Instruction>),
    Function(Vec<Param>, Vec<Line>),
}

impl Instruction {
    pub fn new(
        variables: &VariableMap,
        pair: Pair<Rule>,
        local_variables: &HashSet<String>,
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
                if local_variables.contains(var_name) {
                    Ok(Self::LocalFunctionCall(String::from(var_name), args))
                } else {
                    let Variable::Function(function)
                        = variables.get(var_name)? else {
                        return Err(Error::WrongType(
                            String::from(var_name),
                            String::from("Function")
                        ));
                    };
                    Ok(Self::FunctionCall(function, args))
                }
            }
            Rule::num => {
                let value = pair.as_str().parse::<f64>().unwrap();
                Ok(Self::Variable(Variable::Float(value)))
            }
            Rule::ident => {
                let var_name = String::from(pair.as_str());
                if local_variables.contains(&var_name) {
                    Ok(Self::LocalVariable(var_name))
                } else {
                    let value = variables.get(&var_name)?;
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
                for Param { name, type_name: _ } in &params {
                    local_variables.insert(name.clone());
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
            Self::LocalFunctionCall(name, instructions) => {
                let args = exec_instructions(instructions, intepreter, local_variables)?;
                let Variable::Function(function)
                    = local_variables.get(name).or(
                        intepreter.variables.get(name)).unwrap() else {
                    return Err(
                        Error::WrongType(name.clone(), String::from("function"))
                    );
                };
                function.exec(name, intepreter, args)
            }
            Self::Variable(var) => Ok(var.clone()),
            Self::LocalVariable(name) => Ok(local_variables
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
                    .map(|Param { name, type_name: _ }| name.clone())
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
        local_variables: &HashSet<String>,
        args: &VariableMap,
    ) -> Result<Self, Error> {
        Ok(match self {
            Self::LocalFunctionCall(name, instructions) => {
                let instructions = recreate_instructions(instructions, local_variables, args)?;
                if local_variables.contains(name) {
                    Self::LocalFunctionCall(name.clone(), instructions)
                } else {
                    let Variable::Function(function)
                        = args.get(name).unwrap() else {
                            return Err(Error::WrongType(
                                name.clone(), String::from("function")));
                    };
                    Self::FunctionCall(function, instructions)
                }
            }
            Self::FunctionCall(function, instructions) => {
                let instructions = recreate_instructions(instructions, local_variables, args)?;
                Self::FunctionCall(function.clone(), instructions)
            }
            Self::LocalVariable(name) => {
                if local_variables.contains(name) {
                    Self::LocalVariable(name.clone())
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
                for Param { name, type_name: _ } in params {
                    local_variables.insert(name.clone());
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

pub fn recreate_instructions(
    instructions: &[Instruction],
    local_variables: &HashSet<String>,
    args: &VariableMap,
) -> Result<Vec<Instruction>, Error> {
    instructions
        .iter()
        .map(|instruction| instruction.recreate(local_variables, args))
        .collect()
}

pub fn recreate_lines(
    lines: &[Line],
    local_variables: &mut HashSet<String>,
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
