pub mod type_set;
use crate::{
    error::Error,
    function::{Param, Params},
    instruction::local_variable::LocalVariable,
    join, join_debug,
    parse::{Rule, SimpleSLParser},
};
use pest::{iterators::Pair, Parser};
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    hash::Hash,
    iter::zip,
    str::FromStr,
};
use type_set::TypeSet;

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Function {
        return_type: Box<Type>,
        params: Vec<Type>,
        catch_rest: bool,
    },
    Array(Box<Type>),
    EmptyArray,
    Null,
    Multi(TypeSet),
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
            (Self::Multi(types), Self::Multi(types2)) => types.types.is_subset(&types2.types),
            (_, Self::Multi(types)) => types.types.contains(self),
            (_, Self::Any) | (Self::EmptyArray, Self::Array(_)) => true,
            (Self::Array(element_type), Self::Array(element_type2)) => {
                element_type.matches(&element_type2)
            }
            _ => self == other,
        }
    }
    pub fn concat(self, other: Self) -> Self {
        match (self, other) {
            (Type::Any, _) | (_, Type::Any) => Type::Any,
            (first, second) if first == second => first,
            (Type::Multi(TypeSet { mut types }), Type::Multi(TypeSet { types: types2 })) => {
                types.extend(types2);
                Type::Multi(TypeSet { types })
            }
            (Type::Multi(TypeSet { mut types }), var_type)
            | (var_type, Type::Multi(TypeSet { mut types })) => {
                types.insert(var_type);
                Type::Multi(TypeSet { types })
            }
            (first, second) => Type::Multi(TypeSet {
                types: HashSet::from([first, second]),
            }),
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
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
            Self::Array(var_type) => write!(f, "[{var_type:?}]"),
            Self::EmptyArray => write!(f, "[]"),
            Self::Null => write!(f, "null"),
            Self::Multi(types) => write!(f, "{types}"),
            Self::Any => write!(f, "any"),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
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
            Self::Array(var_type) => write!(f, "[{var_type:?}]"),
            Self::EmptyArray => write!(f, "[]"),
            Self::Null => write!(f, "null"),
            Self::Multi(types) => write!(f, "{types}"),
            Self::Any => write!(f, "any"),
        }
    }
}

impl FromStr for Type {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(pair) = SimpleSLParser::parse(Rule::r#type, s)?.into_iter().next() else {
            return Err(Error::Other("Argument doesn't contain type name".to_owned()))
        };
        Ok(Self::from(pair))
    }
}

impl From<Pair<'_, Rule>> for Type {
    fn from(pair: Pair<'_, Rule>) -> Self {
        match pair.as_rule() {
            Rule::int_type => Self::Int,
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
            Rule::array_type => {
                let pair = pair.into_inner().next().unwrap();
                Self::Array(Self::from(pair).into())
            }

            Rule::multi => {
                let types = pair.into_inner().map(|pair| Type::from(pair)).collect();
                Type::Multi(types)
            }
            Rule::any => Self::Any,
            _ => panic!(),
        }
    }
}

impl From<LocalVariable> for Type {
    fn from(value: LocalVariable) -> Self {
        match value {
            LocalVariable::Function(
                Params {
                    standard,
                    catch_rest,
                },
                return_type,
            ) => {
                let params = standard
                    .into_iter()
                    .map(|Param { var_type, .. }| var_type)
                    .collect();
                Self::Function {
                    return_type: Box::new(return_type),
                    params,
                    catch_rest: catch_rest.is_some(),
                }
            }
            LocalVariable::Other(var_type) => var_type,
        }
    }
}
pub trait GetType {
    fn get_type(&self) -> Type;
}

pub trait GetReturnType {
    fn get_return_type(&self) -> Type;
}
