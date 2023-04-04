use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;
use pest::iterators::Pair;
use crate::function::param::param_from_pair;
use crate::function::{Function, Param};
use crate::intepreter::{Intepreter, VariableMap};
use crate::variable::{Variable, Array};
use crate::error::Error;
use crate::parse::Rule;

#[derive(Clone)]
pub struct LangFunction {
    pub params: Vec<Param>,
    pub body: Vec<Line>
}

impl LangFunction{
    pub fn new(variables:&VariableMap, pair: Pair<Rule>)->Result<Self, Error>{
        let mut inner = pair.into_inner();
        let params_pair = inner.next().unwrap();
        let params = param_from_pair(params_pair);
        let mut local_variables= hashset_from_params(&params);
        let mut result_var=Option::<String>::None;
        let mut body = Vec::<Line>::new();
        for pair in inner {
            match pair.as_rule(){
                Rule::return_variable => {
                    result_var = Some(pair.as_str().to_string())
                },
                Rule::line_end => (),
                _ => {
                    let instruction
                        = Instruction::new(&variables, pair, &local_variables)?;
                    if let Some(var) = result_var.clone(){
                        local_variables.insert(var);
                    }
                    body.push(Line{
                        result_var: result_var,
                        instruction: instruction
                    });
                    result_var = None;
                }
            }
        }
        Ok(Self{ params: params, body: body})
    }
}


impl Function for LangFunction {
    fn exec_intern(&self, _name: String, mut intepreter: &mut Intepreter,
            mut args: VariableMap) -> Result<Variable, Error> {
        let mut to_return = Variable::Null;
        for Line{result_var, instruction} in &self.body{
            let result = instruction.exec(&mut intepreter, &args)?;
            if let Some(var) = result_var{
                args.insert(&var, result);
            } else {
                to_return = result;
            }
        }
        Ok(to_return)
    }
    fn get_params(&self) -> &Vec<Param> {
        &self.params
    }
}

#[derive(Clone, Debug)]
pub struct Line{
    result_var: Option<String>,
    instruction: Instruction
}

#[derive(Clone)]
enum Instruction{
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
        return match pair.as_rule(){
            Rule::function_call => {
                let mut inter = pair.clone().into_inner();
                let Some(ident) = inter.next() else {
                    return Err(Error::SomethingStrange)
                };
                let var_name = ident.as_str();
                let mut array = Vec::<Instruction>::new();
                let Some(args) = inter.next() else {
                    return Err(Error::SomethingStrange)
                };
                for arg in args.into_inner() {
                    array.push(Self::new(&variables, arg, &local_variables)?);
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
                let Ok(value) = pair.as_str().parse::<f64>() else {
                    return Err(Error::SomethingStrange)
                };
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
                let Some(ident)
                    = pair.clone().into_inner().next() else {
                    return Err(Error::SomethingStrange)
                };
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
                let mut local_variables= local_variables.clone();
                for Param{name, type_name: _} in &params{
                    local_variables.insert(name.clone());
                }
                let mut result_var=Option::<String>::None;
                let mut body = Vec::<Line>::new();
                for pair in inner {
                    match pair.as_rule(){
                        Rule::return_variable => {
                            result_var = Some(pair.as_str().to_string())
                        },
                        Rule::line_end => (),
                        _ => {
                            let instruction
                                = Instruction::new(&variables, pair, &local_variables)?;
                            if let Some(var) = result_var.clone(){
                                local_variables.insert(var);
                            }
                            body.push(Line{
                                result_var: result_var,
                                instruction: instruction
                            });
                            result_var = None;
                        }
                    }
                }
                Ok(Self::Function(params, body))
            },
            Rule::null => Ok(Self::Variable(Variable::Null)),
            _ => Err(Error::SomethingStrange)
        }
    }
    fn exec(&self, mut intepreter: &mut Intepreter, local_variables: &VariableMap)
        -> Result<Variable, Error> {
        return match &self {
            Self::FunctionCall(function, instructions) => {
                let mut args = Array::new();
                for instruction in instructions {
                    args.push(instruction.exec(&mut intepreter, &local_variables)?);
                }
                function.exec(String::from("name"), &mut intepreter, args)
            }
            Self::LocalFunctionCall(name, instructions) => {
                let mut args = Array::new();
                for instruction in instructions {
                    args.push(instruction.exec(&mut intepreter, &local_variables)?);
                }
                let Variable::Function(function)
                    = local_variables.get(&name)? else {
                    return Err(Error::WrongType(name.clone(), String::from("function")));
                };
                function.exec(name.clone(), &mut intepreter, args)
            }
            Self::Variable(var) => Ok(var.clone()),
            Self::LocalVariable(name) => local_variables.get(name),
            Self::Array(instructions) => {
                let mut array = Array::new();
                for instruction in instructions {
                    array.push(instruction.exec(&mut intepreter, &local_variables)?);
                }
                Ok(Variable::Array(array.into()))
            }
            Self::Function(params, instructions) => {
                let mut body = Vec::<Line>::new();
                let mut fn_local_variables = hashset_from_params(params);
                for Line{instruction, result_var} in instructions {
                    let instruction= instruction.recreate(
                        &fn_local_variables,local_variables);
                    body.push(Line{result_var: result_var.clone(), instruction});
                    if let Some(var) = result_var {
                        fn_local_variables.insert(var.clone());
                    }
                }
                Ok(Variable::Function(Rc::new(LangFunction{params: params.clone(), body})))
            }
        }
    }
    fn recreate(&self, local_variables: &HashSet<String>, args: &VariableMap) -> Self{
        return match self {
            Self::LocalFunctionCall(name, instructions) => {
                let mut new_instuctions = Vec::<Instruction>::new();
                for instruction in instructions {
                    new_instuctions.push(instruction.recreate(local_variables, args));
                }
                if local_variables.contains(name) {
                    Self::LocalFunctionCall(name.clone(), new_instuctions)
                } else {
                    let Variable::Function(function)
                        = args.get(name).unwrap() else {
                            panic!();
                    };
                    Self::FunctionCall(function, new_instuctions)
                }
            }
            Self::FunctionCall(function, instructions) => {
                let mut new_instuctions = Vec::<Instruction>::new();
                for instruction in instructions {
                    new_instuctions.push(instruction.recreate(local_variables, args));
                }
                Self::FunctionCall(function.clone(), new_instuctions)
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
                let mut new_instructions = Vec::<Instruction>::new();
                for instruction in instructions {
                    new_instructions.push(instruction.recreate(local_variables, args));
                }
                Self::Array(new_instructions)
            }
            Self::Function(params, lines)=>{
                let mut local_variables = local_variables.clone();
                for Param{name, type_name: _} in params{
                    local_variables.insert(name.clone());
                }
                let mut new_instructions = Vec::new();
                for Line{result_var, instruction}in lines {
                    new_instructions.push(Line{
                        result_var: result_var.clone(),
                        instruction: instruction.recreate(&local_variables, args)
                    });
                    if let Some(var) = result_var {
                        local_variables.insert(var.clone());
                    }
                }
                Self::Function(params.clone(), new_instructions)
            }
            _ => self.clone()
        };
    }
}

impl fmt::Debug for Instruction{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

fn hashset_from_params(params: &Vec<Param>) -> HashSet<String>{
    let mut hashset=HashSet::new();
    for Param{name, type_name: _} in params{
        hashset.insert(name.clone());
    }
    hashset
}