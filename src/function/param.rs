use crate::{
    instruction::local_variable::LocalVariableMap, join, parse::Rule, variable_type::Type,
};
use pest::iterators::Pair;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Param {
    pub name: String,
    pub var_type: Type,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.var_type)
    }
}

impl From<Pair<'_, Rule>> for Param {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut inner = value.into_inner();
        Self {
            name: String::from(inner.next().unwrap().as_str()),
            var_type: Type::from(inner.next().unwrap()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Params {
    pub standard: Vec<Param>,
    pub catch_rest: Option<String>,
}

impl fmt::Display for Params {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self {
                standard: params,
                catch_rest: None,
            } => write!(f, "{}", join(params, ", ")),
            Self {
                standard,
                catch_rest: Some(catch_rest),
            } if standard.is_empty() => write!(f, "{catch_rest}..."),
            Self {
                standard,
                catch_rest: Some(catch_rest),
            } => write!(f, "{}, {catch_rest}...", join(standard, ", ")),
        }
    }
}

impl From<Params> for LocalVariableMap {
    fn from(params: Params) -> Self {
        let mut result: Self = params
            .standard
            .into_iter()
            .map(|Param { name, var_type }| (name, var_type.into()))
            .collect();
        if let Some(catch_rest) = params.catch_rest {
            result.insert(catch_rest, Type::Array(Type::Any.into()).into());
        }
        result
    }
}
