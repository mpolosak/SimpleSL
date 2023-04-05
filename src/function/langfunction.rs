use pest::iterators::Pair;
use super::{Function, Param, param::param_from_pair};
use super::instruction::{Line, hashset_from_params, Instruction};
use crate::intepreter::{Intepreter, VariableMap};
use crate::{variable::Variable,error::Error,parse::Rule};

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
                        = Instruction::new(variables, pair, &local_variables)?;
                    if let Some(var) = result_var.clone(){
                        local_variables.insert(var);
                    }
                    body.push(Line{result_var, instruction});
                    result_var = None;
                }
            }
        }
        Ok(Self{params, body})
    }
}


impl Function for LangFunction {
    fn exec_intern(&self, _name: String, intepreter: &mut Intepreter,
            mut args: VariableMap) -> Result<Variable, Error> {
        let mut to_return = Variable::Null;
        for Line{result_var, instruction} in &self.body{
            let result = instruction.exec(intepreter, &args)?;
            if let Some(var) = result_var{
                args.insert(var, result);
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