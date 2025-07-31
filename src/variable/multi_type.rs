use super::Type;
use crate::join;
use derive_more::Display;
use std::{
    collections::{HashSet, hash_set::Iter},
    hash::Hash,
    sync::Arc,
};

#[derive(Clone, Debug, Display, Eq, PartialEq)]
#[display("{}", join(self, "|"))]
pub struct MultiType(pub(crate) Arc<HashSet<Type>>);

impl MultiType {
    pub fn iter(&self) -> Iter<'_, Type> {
        self.0.iter()
    }
}

impl Hash for MultiType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.len().hash(state);
    }
}

impl<const N: usize> From<[Type; N]> for MultiType {
    fn from(value: [Type; N]) -> Self {
        Self(HashSet::from(value).into())
    }
}

impl<'a> IntoIterator for &'a MultiType {
    type Item = &'a Type;

    type IntoIter = Iter<'a, Type>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
