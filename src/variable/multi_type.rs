use super::Type;
use crate::join;
use std::{
    collections::HashSet,
    fmt::Display,
    hash::Hash,
    ops::{Deref, DerefMut},
    sync::Arc,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultiType(Arc<HashSet<Type>>);

impl Hash for MultiType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.len().hash(state);
    }
}

impl<const N: usize> From<[Type; N]> for MultiType {
    fn from(value: [Type; N]) -> Self {
        Self(HashSet::from(value).into())
    }
}

impl From<MultiType> for Box<[Type]> {
    fn from(value: MultiType) -> Self {
        value.0.iter().cloned().collect()
    }
}

impl FromIterator<Type> for MultiType {
    fn from_iter<T: IntoIterator<Item = Type>>(iter: T) -> Self {
        let types: HashSet<_> = iter.into_iter().collect();
        Self(types.into())
    }
}

impl Display for MultiType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let types = Box::from(self.clone());
        write!(f, "{}", join(&types, "|"))
    }
}

impl Deref for MultiType {
    type Target = Arc<HashSet<Type>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MultiType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
