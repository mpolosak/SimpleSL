use std::{rc::Rc, collections::HashSet, fmt};
use pest::iterators::Pair;
use crate::{intepreter::{VariableMap, Intepreter}, parse::Rule, error::Error};
use crate::variable::{Variable,Array};
use super::{Function, Param, param::param_from_pair, LangFunction, Line};

#[derive(Clone)]
pub enum Instruction{
    FunctionCall(Rc<dyn Function>, Vec<Instruction>),
    LocalFunctionCall(String, Vec<Instruction>),
    Variable(Variable),
    LocalVariable(String),
    Array(Vec<Instruction>),
    Function(Vec<Param>, Vec<Line>)
}

impl Instruction {
    pub fn new(variables: &VariableMap, pair: Pair<Rule>,
        local_variables: &HashSet<String>) -> Result<Self, Error>{
        match pair.as_rule(){
            Rule::function_call => {
                let mut inter = pair.clone().into_inner();
                let ident = inter.next().unwrap();
                let var_name = ident.as_str();
                let mut array = Vec::<Instruction>::new();
                let args = inter.next().unwrap();
                for arg in args.into_inner() {
                    array.push(Self::new(variables, arg, local_variables)?);
                }
                if local_variables.contains(var_name) {
                    Ok(Self::LocalFunctionCall(String::from(var_name), array))
                } else {
                    let Variable::Function(function)
                        = variables.get(var_name)? else {
                        return Err(Error::WrongType(
                            String::from(var_name),
                            String::from("Function")
                        ));
                    };
                    Ok(Self::FunctionCall(function, array))
                }
            },
            Rule::num => {
                let value = pair.as_str().parse::<f64>().unwrap();
                Ok(Self::Variable(Variable::Float(value)))
            },
            Rule::ident => {
                let var_name = String::from(pair.as_str());
                if local_variables.contains(&var_name){
                    Ok(Self::LocalVariable(var_name))
                } else {
                    let value = variables.get(&var_name)?;
                    Ok(Self::Variable(value))
                }
            },
            Rule::string => {
                let ident = pair.clone().into_inner().next().unwrap();
                let value = ident.as_str();
                let variable = Variable::String(value.into());
                Ok(Self::Variable(variable))
            },
            Rule::array => {
                let inter = pair.clone().into_inner();
                let mut array = Vec::<Instruction>::new();
                for element in inter {
                    array.push(Self::new(variables, element, local_variables)?);
                }
                Ok(Self::Array(array))
            },
            Rule::function => {
                let mut inner = pair.clone().into_inner();
                let params_pair = inner.next().unwrap();
                let params = param_from_pair(params_pair);
                let mut local_variables= hashset_from_params(&params);
                let mut body = Vec::<Line>::new();
                for pair in inner {
                    body.push(Line::new(variables,pair,&mut local_variables)?);
                }
                Ok(Self::Function(params, body))
            },
            Rule::null => Ok(Self::Variable(Variable::Null)),
            _ => panic!()
        }
    }
    pub fn exec(&self, intepreter: &mut Intepreter, local_variables: &VariableMap)
        -> Result<Variable, Error> {
        match &self {
            Self::FunctionCall(function, instructions) => {
                let mut args = Array::new();
                for instruction in instructions {
                    args.push(instruction.exec(intepreter, local_variables)?);
                }
                function.exec(String::from("name"), intepreter, args)
            }
            Self::LocalFunctionCall(name, instructions) => {
                let mut args = Array::new();
                for instruction in instructions {
                    args.push(instruction.exec(intepreter, local_variables)?);
                }
                let Variable::Function(function) = local_variables.get(name)? else {
                    return Err(Error::WrongType(name.clone(), String::from("function")));
                };
                function.exec(name.clone(), intepreter, args)
            }
            Self::Variable(var) => Ok(var.clone()),
            Self::LocalVariable(name) => local_variables.get(name),
            Self::Array(instructions) => {
                let mut array = Array::new();
                for instruction in instructions {
                    array.push(instruction.exec(intepreter, local_variables)?);
                }
                Ok(Variable::Array(array.into()))
            }
            Self::Function(params, instructions) => {
                let mut body = Vec::<Line>::new();
                let mut fn_local_variables = hashset_from_params(params);
                for Line{instruction, result_var} in instructions {
                    let instruction= instruction.recreate(
                        &fn_local_variables,local_variables)?;
                    body.push(Line{result_var: result_var.clone(), instruction});
                    if let Some(var) = result_var {
                        fn_local_variables.insert(var.clone());
                    }
                }
                Ok(Variable::Function(Rc::new(LangFunction{params: params.clone(), body})))
            }
        }
    }
    fn recreate(&self, local_variables: &HashSet<String>, args: &VariableMap) -> Result<Self, Error>{
        Ok(match self {
            Self::LocalFunctionCall(name, instructions) => {
                let instructions
                    = recreate_instructions(instructions, local_variables, args)?;
                if local_variables.contains(name) {
                    Self::LocalFunctionCall(name.clone(), instructions)
                } else {
                    let Variable::Function(function)
                        = args.get(name).unwrap() else {
                            return Err(Error::WrongType(name.clone(), String::from("function")));
                    };
                    Self::FunctionCall(function, instructions)
                }
            }
            Self::FunctionCall(function, instructions) => {
                let instructions
                    = recreate_instructions(instructions, local_variables, args)?;
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
                let instructions
                    = recreate_instructions(instructions, local_variables, args)?;
                Self::Array(instructions)
            }
            Self::Function(params, lines)=>{
                let mut local_variables = local_variables.clone();
                for Param{name, type_name: _} in params{
                    local_variables.insert(name.clone());
                }
                let mut new_lines = Vec::new();
                for Line{result_var, instruction}in lines {
                    new_lines.push(Line{
                        result_var: result_var.clone(),
                        instruction: instruction.recreate(&local_variables, args)?
                    });
                    if let Some(var) = result_var {
                        local_variables.insert(var.clone());
                    }
                }
                Self::Function(params.clone(), new_lines)
            }
            _ => self.clone()
        })
    }
}

impl fmt::Debug for Instruction{
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub fn hashset_from_params(params: &Vec<Param>) -> HashSet<String>{
    let mut hashset=HashSet::new();
    for Param{name, type_name: _} in params{
        hashset.insert(name.clone());
    }
    hashset
}

pub fn recreate_instructions(instructions: &Vec<Instruction>,
    local_variables: &HashSet<String>, args: &VariableMap)
    -> Result<Vec<Instruction>, Error> {
    let mut new_instuctions = Vec::<Instruction>::new();
    for instruction in instructions {
        new_instuctions.push(instruction.recreate(local_variables, args)?);
    }
    Ok(new_instuctions)
}