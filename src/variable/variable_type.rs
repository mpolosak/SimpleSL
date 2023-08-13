use super::{function_type::FunctionType, type_set::TypeSet};
use crate::{
    join,
    parse::{Rule, SimpleSLParser},
    Error, Result,
};
use pest::{iterators::Pair, Parser};
use std::{fmt::Display, hash::Hash, iter::zip, rc::Rc, str::FromStr};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Function(Rc<FunctionType>),
    Array(Rc<Type>),
    Tuple(Rc<[Rc<Type>]>),
    EmptyArray,
    Void,
    Multi(Box<TypeSet>),
    Any,
}

impl Type {
    pub fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Function(function_type), Self::Function(function_type2)) => {
                function_type.matches(function_type2)
            }
            (Self::Multi(types), Self::Multi(types2)) => types.is_subset(types2),
            (_, Self::Multi(types)) => types.iter().any(|var_type| self.matches(var_type)),
            (_, Self::Any) | (Self::EmptyArray, Self::Array(_)) => true,
            (Self::Array(element_type), Self::Array(element_type2)) => {
                element_type.matches(element_type2)
            }
            (Self::Tuple(types), Self::Tuple(types2)) => {
                types.len() == types2.len()
                    && zip(types.iter(), types2.iter())
                        .all(|(var_type, var_type2)| var_type.matches(var_type2))
            }
            _ => self == other,
        }
    }
    pub fn concat(&self, other: &Self) -> Self {
        match (self, other) {
            (Type::Any, _) | (_, Type::Any) => Type::Any,
            (first, second) if first == second => first.clone(),
            (Type::Multi(types), Type::Multi(types2)) => {
                let mut types = types.clone();
                types.extend(types2.iter().cloned());
                Type::Multi(types)
            }
            (Type::Multi(types), var_type) | (var_type, Type::Multi(types)) => {
                let mut types = types.clone();
                types.insert(var_type.clone());
                Type::Multi(types)
            }
            (first, second) => Type::Multi(TypeSet::from([first.clone(), second.clone()]).into()),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::String => write!(f, "string"),
            Self::Function(function_type) => write!(f, "{function_type}"),
            Self::Array(var_type) => write!(f, "[{var_type}]"),
            Self::EmptyArray => write!(f, "[]"),
            Self::Tuple(types) => write!(f, "({})", join(types, ", ")),
            Self::Void => write!(f, "()"),
            Self::Multi(types) => write!(f, "{types}"),
            Self::Any => write!(f, "any"),
        }
    }
}

impl FromStr for Type {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let Some(pair) = SimpleSLParser::parse(Rule::r#type, s)?.next() else {
            return Err(Error::ArgumentDoesntContainType)
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
            Rule::void_type => Self::Void,
            Rule::function_type => FunctionType::from(pair).into(),
            Rule::array_type => {
                let pair = pair.into_inner().next().unwrap();
                Self::Array(Self::from(pair).into())
            }
            Rule::tuple_type => {
                let types = pair
                    .into_inner()
                    .map(|pair| Type::from(pair).into())
                    .collect();
                Self::Tuple(types)
            }
            Rule::multi => {
                let types = pair.into_inner().map(Type::from).collect();
                Type::Multi(Box::new(types))
            }
            Rule::any => Self::Any,
            _ => panic!(),
        }
    }
}

impl From<[Type; 2]> for Type {
    fn from(value: [Type; 2]) -> Self {
        Type::Multi(Box::new(value.into()))
    }
}
pub trait GetType {
    fn get_type(&self) -> Rc<Type>;
}

pub trait GetReturnType {
    fn get_return_type(&self) -> Rc<Type>;
}
