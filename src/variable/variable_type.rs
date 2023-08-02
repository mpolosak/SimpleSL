use super::{function_type::FunctionType, type_set::TypeSet, Generics};
use crate::{
    error::Error,
    join,
    parse::{Rule, SimpleSLParser},
};
use pest::{iterators::Pair, Parser};
use std::{collections::HashSet, fmt::Display, hash::Hash, iter::zip, rc::Rc};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Function(Rc<FunctionType>),
    Array(Box<Type>),
    Tuple(Box<[Type]>),
    EmptyArray,
    Void,
    Multi(Box<TypeSet>),
    Generic(Rc<str>, Rc<TypeSet>),
    Any,
}

impl Type {
    pub fn new(generics: Option<&Generics>, pair: Pair<Rule>) -> Result<Self, Error> {
        match pair.as_rule() {
            Rule::int_type => Ok(Self::Int),
            Rule::float_type => Ok(Self::Float),
            Rule::string_type => Ok(Self::String),
            Rule::void_type => Ok(Self::Void),
            Rule::function_type => Ok(FunctionType::new(generics, pair)?.into()),
            Rule::array_type => {
                let pair = pair.into_inner().next().unwrap();
                Ok(Self::Array(Self::new(generics, pair)?.into()))
            }
            Rule::tuple_type => {
                let types = pair
                    .into_inner()
                    .map(|pair| Type::new(generics, pair))
                    .collect::<Result<_, _>>()?;
                Ok(Self::Tuple(types))
            }
            Rule::multi => {
                let types = pair
                    .into_inner()
                    .map(|pair| Type::new(generics, pair))
                    .collect::<Result<_, _>>()?;
                Ok(Type::Multi(Box::new(types)))
            }
            Rule::any => Ok(Self::Any),
            Rule::generic_type => {
                let ident = pair.as_str();
                let Some(generics) = generics else {
                    return Err(Error::TypeDoesntExist(ident.into()))
                };
                let Some(typeset) = generics.0.get(ident) else {
                    return Err(Error::TypeDoesntExist(ident.into()))
                };
                Ok(Self::Generic(ident.into(), typeset.clone()))
            }
            _ => panic!(),
        }
    }
    pub fn new_from_str(generics: Option<&Generics>, str: &str) -> Result<Self, Error> {
        let Some(pair) = SimpleSLParser::parse(Rule::r#type, str)?.next() else {
            return Err(Error::ArgumentDoesntContainType)
        };
        Self::new(generics, pair)
    }
    pub fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Function(function_type), Self::Function(function_type2)) => {
                function_type.matches(function_type2)
            }
            (Self::Multi(types), Self::Multi(types2)) => types.types.is_subset(&types2.types),
            (_, Self::Multi(types)) => types.types.iter().any(|var_type| self.matches(var_type)),
            (_, Self::Any) | (Self::EmptyArray, Self::Array(_)) => true,
            (Self::Array(element_type), Self::Array(element_type2)) => {
                element_type.matches(element_type2)
            }
            (Self::Tuple(types), Self::Tuple(types2)) => {
                types.len() == types2.len()
                    && zip(types.iter(), types2.iter())
                        .all(|(var_type, var_type2)| var_type.matches(var_type2))
            }
            (_, Self::Generic(_, typeset)) => {
                typeset.types.iter().any(|var_type| self.matches(var_type))
            }
            (Self::Generic(_, typeset), _) => {
                typeset.types.iter().all(|var_type| var_type.matches(self))
            }
            _ => self == other,
        }
    }
    pub fn concat(self, other: Self) -> Self {
        match (self, other) {
            (Type::Any, _) | (_, Type::Any) => Type::Any,
            (first, second) if first == second => first,
            (Type::Multi(mut types), Type::Multi(types2)) => {
                types.types.extend(types2.types);
                Type::Multi(types)
            }
            (Type::Multi(mut types), var_type) | (var_type, Type::Multi(mut types)) => {
                types.types.insert(var_type);
                Type::Multi(types)
            }
            (first, second) => Type::Multi(
                TypeSet {
                    types: HashSet::from([first, second]),
                }
                .into(),
            ),
        }
    }
    pub fn simplify_generics(&self, generics: &Generics) -> Self {
        match self {
            Type::Function(function) => function.simplify_generics(generics).into(),
            Type::Array(var_type) => Type::Array(var_type.simplify_generics(generics).into()),
            Type::Tuple(types) => Type::Tuple(
                types
                    .iter()
                    .map(|var_type| var_type.simplify_generics(generics))
                    .collect(),
            ),
            Type::Multi(typeset) => Type::Multi(
                TypeSet {
                    types: typeset
                        .types
                        .iter()
                        .map(|var_type| var_type.simplify_generics(generics))
                        .collect(),
                }
                .into(),
            ),
            Type::Generic(name, typeset) => {
                let typeset = generics.0.get(name).unwrap_or(typeset);
                if typeset.types.len() == 1 {
                    let var_type = typeset.types.iter().next().unwrap();
                    if !matches!(var_type, &Type::Any) {
                        return var_type.clone();
                    }
                }
                Type::Generic(name.clone(), typeset.clone())
            }
            other => other.clone(),
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
            Self::Generic(ident, _) => write!(f, "{ident}"),
            Self::Any => write!(f, "any"),
        }
    }
}

impl From<[Type; 2]> for Type {
    fn from(value: [Type; 2]) -> Self {
        Type::Multi(Box::new(value.into()))
    }
}
pub trait GetType {
    fn get_type(&self) -> Type;
}

pub trait GetReturnType {
    fn get_return_type(&self) -> Type;
}
