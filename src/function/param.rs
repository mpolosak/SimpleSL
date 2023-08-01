use crate::{
    error::Error,
    instruction::local_variable::LocalVariableMap,
    join,
    parse::Rule,
    variable::{Generics, Type},
};
use pest::iterators::Pair;
use std::{fmt, rc::Rc};

#[derive(Clone, Debug)]
pub struct Param {
    pub name: Rc<str>,
    pub var_type: Type,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.var_type)
    }
}

impl Param {
    pub fn new(generics: Option<&Generics>, pair: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut inner = pair.into_inner();
        Ok(Self {
            name: inner.next().unwrap().as_str().into(),
            var_type: Type::new(generics, inner.next().unwrap())?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Params {
    pub standard: Rc<[Param]>,
    pub catch_rest: Option<Rc<str>>,
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
            .iter()
            .map(|Param { name, var_type }| (name.clone(), var_type.clone().into()))
            .collect();
        if let Some(catch_rest) = params.catch_rest {
            result.insert(catch_rest, Type::Array(Type::Any.into()).into());
        }
        result
    }
}
