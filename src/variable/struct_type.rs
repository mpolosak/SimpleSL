use std::{collections::HashMap, fmt::Display, hash::Hash, sync::Arc};

use itertools::Itertools;

use crate::variable::Type;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructType(pub(crate) Arc<HashMap<Arc<str>, Type>>);

impl Hash for StructType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.keys().collect::<Box<[&Arc<str>]>>().hash(state)
    }
}

impl Display for StructType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elements = self
            .0
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .join(", ");
        write!(f, "struct{{{elements}}}")
    }
}

impl From<StructType> for Type {
    fn from(value: StructType) -> Self {
        Type::Struct(value)
    }
}
