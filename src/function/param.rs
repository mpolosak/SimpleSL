use crate::{parse::Rule, variable_type::Type};
use pest::iterators::Pair;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Param {
    Standard(String, Type),
    CatchRest(String),
}

impl Param {
    pub fn get_name(&self) -> &str {
        match self {
            Self::Standard(name, _) | Self::CatchRest(name) => name,
        }
    }
    pub fn get_type(&self) -> Type {
        match self {
            Self::Standard(_, var_type) => var_type.clone(),
            Self::CatchRest(_) => Type::Array,
        }
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard(name, var_type) => write!(f, "{name}:{var_type}"),
            Self::CatchRest(name) => write!(f, "{name}..."),
        }
    }
}

impl From<Pair<'_, Rule>> for Param {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut inner = value.into_inner();
        Self::Standard(
            String::from(inner.next().unwrap().as_str()),
            Type::from(inner.next().unwrap()),
        )
    }
}
