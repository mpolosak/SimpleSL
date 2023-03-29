use pest::iterators::Pair;
use crate::parse::Rule;
use std::fmt;

#[derive(Clone,Debug)]
pub struct Param {
    pub name: String,
    pub type_name: String,
}

impl Param {
    pub fn new(name: &str, type_name: &str) -> Self {
        Param {
            name: String::from(name),
            type_name: String::from(type_name)
        }
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.type_name == "..." {
            write!(f, "{}...", self.name)
        } else {
            write!(f, "{}:{}", self.name, self.type_name)
        }
    }
}

impl From<Pair<'_, Rule>> for Param {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut inner = value.into_inner();
        Self::new(inner.next().unwrap().as_str(), inner.next().unwrap().as_str())
    }
}