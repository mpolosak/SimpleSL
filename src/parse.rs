use std::rc::Rc;
use pest::iterators::Pair;
use crate::variable::{Variable, Array};

#[derive(Parser)]
#[grammar = "simplesl.pest"]
pub struct SimpleSLParser;

pub fn variable_from_pair(pair: Pair<Rule>) -> Result<Variable, String>{
    return match pair.as_rule() {
        Rule::num => {
            let Ok(value) = pair.as_str().parse::<f64>() else {
                return Err(String::from("Something strange happened"))
            };
            Ok(Variable::Float(value))
        },
        Rule::string => {
            let Some(ident) = pair.into_inner().next() else {
                return Err(String::from("Something strange happened"))
            };
            let value = ident.as_str();
            Ok(Variable::String(value.into()))
        },
        Rule::array => {
            let mut array = Array::new();
            for element in pair.into_inner() {
                array.push(variable_from_pair(element)?);
            }
            Ok(Variable::Array(Rc::new(array)))
        },
        Rule::null => Ok(Variable::Null),
        _ => Err(String::from("This cannot be parsed to variable"))
    }
}