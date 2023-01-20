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
        Rule::text => {
            let Some(ident) = pair.into_inner().next() else {
                return Err(String::from("Something strange happened"))
            };
            let value = String::from(ident.as_str());
            Ok(Variable::Text(value))
        },
        Rule::array => {
            let mut array = Array::new();
            for element in pair.into_inner() {
                array.push(variable_from_pair(element)?);
            }
            Ok(Variable::Array(array))
        },
        Rule::referance => {
            let Some(ident) = pair.into_inner().next() else {
                return Err(String::from("Something strange happened"))
            };
            let value = String::from(ident.as_str());
            Ok(Variable::Referance(value))
        },
        _ => Err(String::from("This cannot be parsed to variable"))
    }
}