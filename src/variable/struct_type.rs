use std::{collections::HashMap, fmt::Display, hash::Hash, sync::Arc};

use itertools::Itertools;
use pest::iterators::Pair;
use simplesl_parser::Rule;

use crate::variable::Type;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructType(pub Arc<HashMap<Arc<str>, Type>>);

impl StructType {
    pub fn matches(&self, other: &Self) -> bool {
        for (key, var_type2) in other.0.iter() {
            let Some(var_type) = self.0.get(key) else {
                return false;
            };
            if !var_type.matches(var_type2) {
                return false;
            }
        }
        true
    }
}

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
            .map(|(key, value)| format!("{key}: {value}"))
            .join(", ");
        write!(f, "struct{{{elements}}}")
    }
}

impl From<HashMap<Arc<str>, Type>> for StructType {
    fn from(value: HashMap<Arc<str>, Type>) -> Self {
        StructType(value.into())
    }
}

impl<const N: usize> From<[(Arc<str>, Type); N]> for StructType {
    fn from(value: [(Arc<str>, Type); N]) -> Self {
        StructType(Arc::new(value.into()))
    }
}

#[doc(hidden)]
impl From<Pair<'_, Rule>> for StructType {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let tm = pair
            .into_inner()
            .tuples()
            .map(|(key, value)| (key.as_str().into(), Type::from(value)))
            .collect();
        Self(Arc::new(tm))
    }
}

impl From<StructType> for Type {
    fn from(value: StructType) -> Self {
        Type::Struct(value)
    }
}

#[cfg(test)]
mod tests {
    use crate as simplesl;
    use crate::variable::StructType;
    use simplesl_macros::var_type;

    #[test]
    pub fn matches() {
        let s1 = StructType::from([("a".into(), var_type!(int))]);
        assert!(s1.matches(&s1));
        let s2 = StructType::from([("a".into(), var_type!(int))]);
        assert!(s1.matches(&s2));
        assert!(s2.matches(&s1));
        let s2 = StructType::from([("a".into(), var_type!(int | float))]);
        assert!(s1.matches(&s2));
        assert!(!s2.matches(&s1));
        let s1 = StructType::from([
            ("a".into(), var_type!(int | float)),
            ("b".into(), var_type!(any)),
        ]);
        assert!(s1.matches(&s2));
        assert!(!s2.matches(&s1));
        let s2 = StructType::from([
            ("a".into(), var_type!(int | float | string)),
            ("b".into(), var_type!(any)),
        ]);
        assert!(s1.matches(&s2));
        assert!(!s2.matches(&s1));
    }
}
