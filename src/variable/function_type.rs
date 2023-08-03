use super::{GetReturnType, Type};
use crate::{join, parse::Rule};
use pest::iterators::Pair;
use std::{fmt::Display, iter::zip};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct FunctionType {
    pub params: Box<[Type]>,
    pub return_type: Type,
}

impl FunctionType {
    pub fn matches(&self, other: &Self) -> bool {
        self.params.len() == other.params.len()
            && zip(self.params.iter(), other.params.iter())
                .all(|(type1, type2)| type2.matches(type1))
            && self.return_type.matches(&other.return_type)
    }
}

impl Display for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "function({})->{}",
            join(&self.params, ", "),
            self.return_type
        )
    }
}

impl GetReturnType for FunctionType {
    fn get_return_type(&self) -> Type {
        self.return_type.clone()
    }
}

impl From<Pair<'_, Rule>> for FunctionType {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut return_type = Type::Any;
        let mut params: Box<[Type]> = [].into();
        for pair in pair.into_inner() {
            if pair.as_rule() == Rule::function_type_params {
                params = pair.into_inner().map(Type::from).collect();
            } else {
                return_type = Type::from(pair);
            }
        }
        Self {
            return_type,
            params,
        }
    }
}

impl From<FunctionType> for Type {
    fn from(value: FunctionType) -> Self {
        Self::Function(value.into())
    }
}
