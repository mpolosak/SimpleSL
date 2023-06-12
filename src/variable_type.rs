use crate::{join, join_debug, parse::Rule};
use pest::iterators::Pair;
use std::{
    fmt::{Debug, Display},
    iter::zip,
};

#[derive(Clone, PartialEq)]
pub enum Type {
    Float,
    String,
    Function(Box<Type>, Vec<Type>, bool),
    Array,
    Null,
    Any,
}

impl Type {
    pub fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Function(return_type, param_types, catch_rest),
                Self::Function(return_type2, param_types2, catch_rest2),
            ) => {
                if (*catch_rest2 || param_types.len() != param_types2.len()) && !catch_rest {
                    false
                } else {
                    for (type1, type2) in zip(param_types, param_types2) {
                        if !type1.matches(type2) {
                            return false;
                        }
                    }
                    return_type.matches(return_type2)
                }
            }
            (_, Self::Any) => true,
            _ => self == other,
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float => write!(f, "float"),
            Self::String => write!(f, "string"),
            Self::Function(return_type, param_types, false) => {
                write!(
                    f,
                    "function({})->{return_type}",
                    join_debug(param_types, ", ")
                )
            }
            Self::Function(return_type, param_types, true) => {
                write!(
                    f,
                    "function({},...)->{return_type}",
                    join_debug(param_types, ", ")
                )
            }
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
            Self::Function(return_type, param_types, false) => {
                write!(f, "function({})->{return_type}", join(param_types, ", "))
            }
            Self::Function(return_type, param_types, true) => {
                write!(
                    f,
                    "function({},...)->{return_type}",
                    join(param_types, ", ")
                )
            }
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
                // if let Some(return_pair) = pair.into_inner().next() {
                //     Self::Function(Box::new(Type::from(return_pair)))
                // } else {
                //     Self::Function(Self::Any.into())
                // }
                todo!()
            }
            Rule::array_type => Self::Array,
            Rule::any => Self::Any,
            _ => panic!(),
        }
    }
}

pub trait GetType {
    fn get_type(&self) -> Type;
}
