use super::{ReturnType, Type};
use crate::join;
use derive_more::Display;
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::{iter::zip, ops::BitOr, sync::Arc};

#[derive(Clone, Debug, Display, Hash, PartialEq, Eq)]
#[display(
    "({})->{}",
    join(self.params.as_ref(), ", "),
    if let Type::Multi(_) = self.return_type {
        format!("({})", self.return_type)
    } else {
        self.return_type.to_string()
    }
)]
pub struct FunctionType {
    pub params: Arc<[Type]>,
    pub return_type: Type,
}

impl FunctionType {
    #[must_use]
    pub fn matches(&self, other: &Self) -> bool {
        self.params.len() == other.params.len()
            && zip(self.params.iter(), other.params.iter())
                .all(|(type1, type2)| type2.matches(type1))
            && self.return_type.matches(&other.return_type)
    }

    #[must_use]
    pub fn concat(self, other: Self) -> Type {
        Type::from(self) | Type::from(other)
    }
}

impl ReturnType for FunctionType {
    fn return_type(&self) -> Type {
        self.return_type.clone()
    }
}

impl<T: Into<Type>> BitOr<T> for FunctionType {
    type Output = Type;

    fn bitor(self, rhs: T) -> Self::Output {
        Type::concat(self.into(), rhs.into())
    }
}

#[doc(hidden)]
impl From<Pair<'_, Rule>> for FunctionType {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut pairs = pair.into_inner();
        let params: Arc<[Type]> = pairs.next().unwrap().into_inner().map(Type::from).collect();
        let return_type = Type::from(pairs.next().unwrap());
        Self {
            params,
            return_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use simplesl_macros::var_type;

    use super::FunctionType;
    use crate as simplesl;

    #[test]
    fn check_function_type_matches() {
        let function_type = FunctionType {
            params: [var_type!(any)].into(),
            return_type: var_type!(int),
        };
        let function_type2 = FunctionType {
            params: [var_type!(int)].into(),
            return_type: var_type!(any),
        };
        assert!(function_type.matches(&function_type));
        assert!(function_type2.matches(&function_type2));
        assert!(function_type.matches(&function_type2));
        assert!(!function_type2.matches(&function_type));
        let function_type = FunctionType {
            params: [
                var_type!(any),
                var_type!(int),
                var_type!(float | string | [any]),
            ]
            .into(),
            return_type: var_type!(int),
        };
        let function_type2 = FunctionType {
            params: [var_type!(int)].into(),
            return_type: var_type!(any),
        };
        let function_type3 = FunctionType {
            params: [var_type!(string), var_type!(int), var_type!(float | string)].into(),
            return_type: var_type!(int | float | string),
        };
        assert!(function_type.matches(&function_type));
        assert!(function_type2.matches(&function_type2));
        assert!(function_type3.matches(&function_type3));
        assert!(!function_type.matches(&function_type2));
        assert!(!function_type2.matches(&function_type));
        assert!(function_type.matches(&function_type3));
        assert!(!function_type3.matches(&function_type));
        assert!(!function_type3.matches(&function_type2));
        assert!(!function_type2.matches(&function_type3));
    }
}
