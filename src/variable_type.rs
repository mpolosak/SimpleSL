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
    Function {
        return_type: Box<Type>,
        params: Vec<Type>,
        catch_rest: bool,
    },
    Array,
    Null,
    Any,
}

impl Type {
    pub fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Function {
                    return_type,
                    params,
                    catch_rest,
                },
                Self::Function {
                    return_type: return_type2,
                    params: params2,
                    catch_rest: catch_rest2,
                },
            ) => {
                if (*catch_rest2 || params.len() != params2.len()) && !catch_rest {
                    false
                } else {
                    for (type1, type2) in zip(params, params2) {
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
            Self::Function {
                return_type,
                params,
                catch_rest: false,
            } => {
                write!(f, "function({})->{return_type}", join_debug(params, ", "))
            }
            Self::Function {
                return_type,
                params,
                catch_rest: true,
            } => {
                write!(
                    f,
                    "function({},...)->{return_type}",
                    join_debug(params, ", ")
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
            Self::Function {
                return_type,
                params,
                catch_rest: false,
            } => {
                write!(f, "function({})->{return_type}", join(params, ", "))
            }
            Self::Function {
                return_type,
                params,
                catch_rest: true,
            } => {
                write!(f, "function({},...)->{return_type}", join(params, ", "))
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
                let mut return_type = Self::Any;
                let mut params = Vec::new();
                let catch_rest = false;
                for pair in pair.into_inner() {
                    if pair.as_rule() == Rule::function_type_params {
                        params = pair.into_inner().map(Type::from).collect();
                    } else {
                        return_type = Type::from(pair);
                    }
                }
                let return_type = Box::new(return_type);
                Self::Function {
                    return_type,
                    params,
                    catch_rest,
                }
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
