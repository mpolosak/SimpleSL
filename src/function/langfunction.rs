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

impl From<Pair<'_, Rule>> for LangFunction{
    fn from(value: Pair<Rule>) -> Self {
        let mut inner = value.into_inner();
        let params_pair = inner.next().unwrap();
        let mut params = Vec::<Param>::new();
        for pair in params_pair.into_inner() {
            params.push(Param::from(pair));
        }
        for pair in inner {
            println!("{pair:?}");
        }
        LangFunction { params: Vec::new(), body: Vec::new() }
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
    Variable(Variable),
    LocalVariable(String),
    Array(Vec<Instruction>)
}

impl fmt::Debug for Instruction{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}