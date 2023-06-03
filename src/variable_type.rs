use crate::parse::Rule;
use pest::iterators::Pair;
use std::fmt::{Debug, Display};

#[derive(Clone, PartialEq)]
pub enum Type {
    Float,
    String,
    Function(Box<Type>),
    Array,
    Null,
    Any,
}

impl Type {
    pub fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Function(return_type), Self::Function(return_type2)) => {
                return_type.matches(return_type2)
            }
            (Self::Any, Self::Any) | (_, Self::Any) | (Self::Any, _) => true,
            _ => self == other,
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float => write!(f, "float"),
            Self::String => write!(f, "string"),
            Self::Function(var_type) => write!(f, "function->{var_type}"),
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
            Self::Function(var_type) => write!(f, "function->{var_type}"),
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
            Rule::function_type => {
                if let Some(return_pair) = pair.into_inner().next() {
                    Self::Function(Box::new(Type::from(return_pair)))
                } else {
                    Self::Function(Self::Any.into())
                }
            }
            Rule::array_type => Self::Array,
            Rule::any => Self::Any,
            _ => panic!(),
        }
    }
}
