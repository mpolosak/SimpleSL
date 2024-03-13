use crate::{instruction::local_variable::LocalVariableMap, join, parse::Rule, variable::Type};
use pest::iterators::Pair;
use std::{fmt, ops::Deref, rc::Rc};

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

#[derive(Clone, Debug)]
pub struct Params(pub Rc<[Param]>);

impl fmt::Display for Params {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", join(self, ", "))
    }
}

impl Deref for Params {
    type Target = Rc<[Param]>;

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
