use super::Type;
use crate::{join, join_debug};
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    hash::Hash,
};

#[derive(Clone, Eq)]
pub struct TypeSet {
    pub(crate) types: HashSet<Type>,
}

impl PartialEq for TypeSet {
    fn eq(&self, other: &Self) -> bool {
        self.types.symmetric_difference(&other.types).count() == 0
    }
}

impl Hash for TypeSet {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let types = Vec::from(self.clone());
        types.hash(state)
    }
}

impl From<TypeSet> for Vec<Type> {
    fn from(value: TypeSet) -> Self {
        value.types.into_iter().collect()
    }
}

impl FromIterator<Type> for TypeSet {
    fn from_iter<T: IntoIterator<Item = Type>>(iter: T) -> Self {
        let types = iter.into_iter().collect();
        Self { types }
    }
}

impl Display for TypeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let types = Vec::from(self.clone());
        write!(f, "{}", join(&types, "|"))
    }
}

impl Debug for TypeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let types = Vec::from(self.clone());
        write!(f, "{}", join_debug(&types, "|"))
    }
}
