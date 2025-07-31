use crate::{instruction::local_variable::LocalVariableMap, join, variable::Type};
use derive_more::Display;
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::{ops::Deref, sync::Arc};

#[derive(Clone, Debug, Display, PartialEq, Eq)]
#[display("{name}: {var_type}")]
pub struct Param {
    pub name: Arc<str>,
    pub var_type: Type,
}

#[doc(hidden)]
impl From<Pair<'_, Rule>> for Param {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut inner = value.into_inner();
        Self {
            name: inner.next().unwrap().as_str().into(),
            var_type: Type::from(inner.next().unwrap()),
        }
    }
}

#[derive(Clone, Debug, Display)]
#[display("{}", join(self.as_ref(), ", "))]
pub struct Params(pub Arc<[Param]>);

impl Deref for Params {
    type Target = Arc<[Param]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Params> for LocalVariableMap {
    fn from(params: Params) -> Self {
        params
            .iter()
            .map(|Param { name, var_type }| (name.clone(), var_type.clone().into()))
            .collect()
    }
}

impl FromIterator<Param> for Params {
    fn from_iter<T: IntoIterator<Item = Param>>(iter: T) -> Self {
        let value = iter.into_iter().collect();
        Self(value)
    }
}
