use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;
use pest::iterators::Pair;
use crate::function::{Function, Param};
use crate::intepreter::{Intepreter, VariableMap};
use crate::variable::Variable;
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
        let mut params = Vec::<Param>::new();
        for pair in params_pair.into_inner() {
            params.push(Param::from(pair));
        }
        let mut local_variables=HashSet::<String>::new();
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
                    body.push(Line{result_var: result_var, instruction: instruction});
                    result_var = None;
                }
            }
        }
        Ok(Self{ params: params, body: body})
    }
}


impl Function for LangFunction {
    fn exec_intern(&self, _name: String, intepreter: &mut Intepreter,
            _args: VariableMap) -> Result<Variable, Error> {
        for instruction in &self.body{
            println!("{instruction:?}");
        }
        Ok(Variable::Null)
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
            Rule::null => Ok(Self::Variable(Variable::Null)),
            _ => Err(Error::SomethingStrange)
        }
    }
}

impl fmt::Debug for Instruction{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}