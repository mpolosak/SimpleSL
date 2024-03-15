use super::Type;
use crate::join;
use std::{
    collections::HashSet,
    fmt::Display,
    hash::Hash,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeSet(pub HashSet<Type>);

impl Hash for TypeSet {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.len().hash(state);
    }
}

impl<const N: usize> From<[Type; N]> for TypeSet {
    fn from(value: [Type; N]) -> Self {
        Self(value.into())
    }
}

impl From<TypeSet> for Box<[Type]> {
    fn from(value: TypeSet) -> Self {
        value.0.into_iter().collect()
    }
}

impl FromIterator<Type> for TypeSet {
    fn from_iter<T: IntoIterator<Item = Type>>(iter: T) -> Self {
        let types = iter.into_iter().collect();
        Self(types)
    }
}

impl Display for TypeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let types = Box::from(self.clone());
        write!(f, "{}", join(&types, "|"))
    }
}

impl Deref for TypeSet {
    type Target = HashSet<Type>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TypeSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
