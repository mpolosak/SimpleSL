use super::{ReturnType, Type};
use crate::{join, parse::Rule};
use pest::iterators::Pair;
use std::{fmt::Display, iter::zip};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct FunctionType {
    pub params: Box<[Type]>,
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
}

impl Display for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if matches!(self.return_type, Type::Multi(_)) {
            return write!(
                f,
                "function({})->({})",
                join(&self.params, ", "),
                self.return_type
            );
        }
        write!(
            f,
            "function({})->{}",
            join(&self.params, ", "),
            self.return_type
        )
    }
}

impl ReturnType for FunctionType {
    fn return_type(&self) -> Type {
        self.return_type.clone()
    }
}

#[doc(hidden)]
impl From<Pair<'_, Rule>> for FunctionType {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut return_type = Type::Any;
        let mut params: Box<[Type]> = [].into();
        for pair in pair.into_inner() {
            if pair.as_rule() == Rule::function_type_params {
                params = pair.into_inner().map(Type::from).collect();
            } else {
                return_type = Type::from(pair);
            }
        }
        Self {
            params,
            return_type,
        }
    }
}

impl From<FunctionType> for Type {
    fn from(value: FunctionType) -> Self {
        Self::Function(value.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::variable::{type_set::TypeSet, Type};

    use super::FunctionType;
    #[test]
    fn check_function_type_matches() {
        let function_type = FunctionType {
            params: [Type::Any].into(),
            return_type: Type::Int,
        };
        let function_type2 = FunctionType {
            params: [Type::Int].into(),
            return_type: Type::Any,
        };
        assert!(function_type.matches(&function_type));
        assert!(function_type2.matches(&function_type2));
        assert!(function_type.matches(&function_type2));
        assert!(!function_type2.matches(&function_type));
        let function_type = FunctionType {
            params: [
                Type::Any,
                Type::Int,
                Type::Multi(TypeSet::from([Type::Float, Type::String, [Type::Any].into()]).into()),
            ]
            .into(),
            return_type: Type::Int,
        };
        let function_type2 = FunctionType {
            params: [Type::Int].into(),
            return_type: Type::Any,
        };
        let function_type3 = FunctionType {
            params: [
                Type::String,
                Type::Int,
                Type::Multi(TypeSet::from([Type::Float, Type::String]).into()),
            ]
            .into(),
            return_type: Type::Multi(TypeSet::from([Type::Float, Type::String, Type::Int]).into()),
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
