use crate::parse::Rule;
use pest::iterators::Pair;
use std::fmt::{Debug, Display};

#[derive(Copy, Clone, PartialEq)]
pub enum Type {
    Float,
    String,
    Function,
    Array,
    Null,
    Any,
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float => write!(f, "float"),
            Self::String => write!(f, "string"),
            Self::Function => write!(f, "function"),
            Self::Array => write!(f, "array"),
            Self::Null => write!(f, "null"),
            Self::Any => write!(f, "any"),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float => write!(f, "float"),
            Self::String => write!(f, "string"),
            Self::Function => write!(f, "function"),
            Self::Array => write!(f, "array"),
            Self::Null => write!(f, "null"),
            Self::Any => write!(f, "any"),
        }
    }
}

impl From<Pair<'_, Rule>> for Type {
    fn from(pair: Pair<'_, Rule>) -> Self {
        match pair.as_rule() {
            Rule::float_type => Self::Float,
            Rule::string_type => Self::String,
            Rule::null_type => Self::Null,
            Rule::function_type => Self::Function,
            Rule::array_type => Self::Array,
            Rule::any => Self::Any,
            _ => panic!(),
        }
    }
}
