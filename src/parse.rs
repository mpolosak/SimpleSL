use crate::{
    error::Error,
    variable::{Array, Variable},
};
use pest::iterators::Pair;
use std::rc::Rc;

#[derive(Parser)]
#[grammar = "simplesl.pest"]
pub struct SimpleSLParser;

pub fn variable_from_pair(pair: Pair<Rule>) -> Result<Variable, Error> {
    return match pair.as_rule() {
        Rule::num => {
            let value = pair.as_str().parse::<f64>().unwrap();
            Ok(Variable::Float(value))
        }
        Rule::string => {
            let ident = pair.into_inner().next().unwrap();
            let value = ident.as_str();
            Ok(Variable::String(value.into()))
        }
        Rule::array => {
            let mut array = Array::new();
            for element in pair.into_inner() {
                array.push(variable_from_pair(element)?);
            }
            Ok(Variable::Array(Rc::new(array)))
        }
        Rule::null => Ok(Variable::Null),
        _ => Err(Error::CannotBeParsed(pair.as_str().into())),
    };
}
