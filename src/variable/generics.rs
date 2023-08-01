use super::{type_set::TypeSet, Type};
use crate::{
    error::Error,
    join,
    parse::{Rule, SimpleSLParser},
};
use pest::{iterators::Pair, Parser};
use std::{collections::HashMap, fmt::Display, rc::Rc};

#[derive(Default, Debug)]
pub struct Generics(pub HashMap<Rc<str>, Rc<TypeSet>>);
impl Generics {
    pub fn new(generics: Option<&Generics>, pair: Pair<'_, Rule>) -> Result<Self, Error> {
        let hash_map = pair
            .into_inner()
            .map(|pair| {
                let mut inner = pair.into_inner();
                let ident = inner.next().unwrap().as_str();
                let types = inner
                    .map(|pair| Type::new(generics, pair))
                    .collect::<Result<TypeSet, Error>>()?
                    .into();
                Ok((ident.into(), types))
            })
            .collect::<Result<_, Error>>()?;
        Ok(Generics(hash_map))
    }
    pub fn new_from_str(generics: Option<&Generics>, str: &str) -> Result<Self, Error> {
        let Some(pair) = SimpleSLParser::parse(Rule::generics, str)?.next() else {
            return Err(Error::ArgumentDoesntContainType)
        };
        Self::new(generics, pair)
    }
}

impl Display for Generics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<")?;
        let strings: Box<_> = self
            .0
            .iter()
            .map(|(ident, typeset)| {
                let types = Box::from(typeset.as_ref().clone());
                format!("{ident}: {}", join(&types, ", "))
            })
            .collect();
        write!(f, "{}>", join(&strings, "; "))
    }
}
