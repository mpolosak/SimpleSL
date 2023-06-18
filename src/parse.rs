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
        Rule::int => {
            let value = pair.as_str().parse::<f64>().unwrap();
            Ok(Variable::Float(value))
        }
        Rule::float => {
            let value = pair.as_str().parse::<f64>().unwrap();
            Ok(Variable::Float(value))
        }
        Rule::string => {
            let value = pair.into_inner().next().unwrap().as_str();
            Ok(Variable::String(value.into()))
        }
        Rule::array => {
            let array = pair
                .into_inner()
                .map(variable_from_pair)
                .collect::<Result<Array, Error>>()?;
            Ok(Variable::Array(Rc::new(array)))
        }
        Rule::null => Ok(Variable::Null),
        _ => Err(Error::CannotBeParsed(pair.as_str().into())),
    };
}
