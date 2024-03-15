use super::{function_type::FunctionType, multi_type::MultiType};
use crate::{
    errors::ParseTypeError,
    join,
    parse::{Rule, SimpleSLParser},
};
use match_any::match_any;
use pest::{iterators::Pair, Parser};
use std::{
    fmt::Display,
    hash::Hash,
    iter::zip,
    ops::{BitOr, BitOrAssign},
    str::FromStr,
    sync::Arc,
};
use typle::typle;
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Function(Arc<FunctionType>),
    Array(Arc<Type>),
    Tuple(Arc<[Type]>),
    EmptyArray,
    Void,
    Multi(MultiType),
    Any,
    Never,
}

impl Type {
    #[must_use]
    pub fn matches(&self, other: &Self) -> bool {
        match_any! { (self, other),
            (Type::Never, _) => true,
            (Self::Function(var_type), Self::Function(var_type2))
            | (Self::Array(var_type), Self::Array(var_type2)) => {
                var_type.matches(var_type2)
            },
            (Self::Multi(types), other) => types.iter().all(|var_type| var_type.matches(other)),
            (_, Self::Multi(types)) => types.iter().any(|var_type| self.matches(var_type)),
            (_, Self::Any) | (Self::EmptyArray, Self::Array(_)) => true,
            (Self::Tuple(types), Self::Tuple(types2)) => {
                types.len() == types2.len()
                    && zip(types.iter(), types2.iter())
                        .all(|(var_type, var_type2)| var_type.matches(var_type2))
            },
            _ => self == other
        }
    }
    #[must_use]
    pub fn concat(self, other: Self) -> Self {
        match (self, other) {
            (Type::Never, other) | (other, Type::Never) => other,
            (Type::Any, _) | (_, Type::Any) => Type::Any,
            (first, second) if first == second => first,
            (Type::Multi(mut types), Type::Multi(types2)) => {
                Arc::make_mut(&mut types).extend(types2.iter().cloned());
                Type::Multi(types)
            }
            (Type::Multi(mut types), var_type) | (var_type, Type::Multi(mut types)) => {
                Arc::make_mut(&mut types).insert(var_type);
                Type::Multi(types)
            }
            (first, second) => Type::Multi(MultiType::from([first, second])),
        }
    }

    /// Flatten self to single tuple
    pub fn flatten_tuple(self) -> Option<Arc<[Type]>> {
        match self {
            Type::Tuple(tuple) => Some(tuple),
            Type::Multi(multi) => {
                let mut iter = multi.iter().cloned();
                let first = iter.next().unwrap().flatten_tuple()?;
                iter.map(Self::flatten_tuple).try_fold(first, |acc, curr| {
                    let curr = curr?;
                    if acc.len() != curr.len() {
                        return None;
                    }
                    Some(
                        zip(acc.iter().cloned(), curr.iter().cloned())
                            .map(|(a, b)| a.concat(b))
                            .collect(),
                    )
                })
            }
            _ => None,
        }
    }

    /// Returns type of element returned when indexing into
    pub fn index_result(&self) -> Option<Type> {
        match self {
            Type::Array(element) => Some(element.as_ref().clone()),
            Type::EmptyArray => Some(Type::Never),
            Type::Multi(multi) => {
                let mut iter = multi.iter();
                let first = iter.next().unwrap().index_result()?;
                iter.map(Self::index_result)
                    .try_fold(first, |acc, curr| Some(acc | curr?))
            }
            Type::String => Some(Type::String),
            _ => None,
        }
    }

    pub fn return_type(&self) -> Option<Type> {
        match self {
            Type::Function(function) => Some(function.return_type()),
            Type::Multi(multi) => {
                let mut iter = multi.iter();
                let first = iter.next().unwrap().return_type()?;
                iter.map(Self::return_type)
                    .try_fold(first, |acc, curr| Some(acc | curr?))
            }
            _ => None,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::String => write!(f, "string"),
            Self::Function(function_type) => write!(f, "{function_type}"),
            Self::Array(var_type) => write!(f, "[{var_type}]"),
            Self::EmptyArray => write!(f, "[]"),
            Self::Tuple(types) => write!(f, "({})", join(types, ", ")),
            Self::Void => write!(f, "()"),
            Self::Multi(types) => write!(f, "{types}"),
            Self::Any => write!(f, "any"),
            Self::Never => write!(f, "!"),
        }
    }
}

impl FromStr for Type {
    type Err = ParseTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Ok(mut pairs) = SimpleSLParser::parse(Rule::r#type, s) else {
            return Err(ParseTypeError);
        };
        Ok(Type::from(pairs.next().unwrap()))
    }
}

#[doc(hidden)]
impl From<Pair<'_, Rule>> for Type {
    fn from(pair: Pair<'_, Rule>) -> Self {
        match pair.as_rule() {
            Rule::int_type => Self::Int,
            Rule::float_type => Self::Float,
            Rule::string_type => Self::String,
            Rule::void => Self::Void,
            Rule::function_type => FunctionType::from(pair).into(),
            Rule::array_type => {
                let pair = pair.into_inner().next().unwrap();
                Self::Array(Self::from(pair).into())
            }
            Rule::tuple_type => {
                let types = pair.into_inner().map(Type::from).collect();
                Self::Tuple(types)
            }
            Rule::multi => pair
                .into_inner()
                .map(Type::from)
                .reduce(Type::concat)
                .unwrap(),
            Rule::any => Self::Any,
            rule => panic!("Type cannot be built from rule: {rule:?}"),
        }
    }
}

impl<T: Into<Type>> BitOr<T> for Type {
    type Output = Self;

    fn bitor(self, rhs: T) -> Self::Output {
        self.concat(rhs.into())
    }
}

impl BitOr<Type> for [Type; 1] {
    type Output = Type;

    fn bitor(self, rhs: Type) -> Self::Output {
        Type::from(self).concat(rhs)
    }
}

impl BitOrAssign for Type {
    fn bitor_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (first, second) if second.matches(first) => (),
            (first, second) if first.matches(&second) => *first = second,
            (Type::Multi(typeset), Type::Multi(typeset2)) => {
                Arc::make_mut(typeset).extend(typeset2.iter().cloned())
            }
            (Type::Multi(typeset), second) => {
                Arc::make_mut(typeset).insert(second);
            }
            (first, second) => *first = first.clone() | second,
        }
    }
}

impl From<[Type; 1]> for Type {
    fn from([value]: [Type; 1]) -> Self {
        Type::Array(value.into())
    }
}

#[typle(Tuple for 2..=12)]
impl<T: Tuple<Type>> From<T> for Type {
    fn from(value: T) -> Self {
        let types: [Type; Tuple::LEN] = value.into();
        Type::Tuple(types.into())
    }
}
pub trait Typed {
    /// Returns the Type of the &self
    fn as_type(&self) -> Type;
}

pub trait ReturnType {
    /// Returns the Type of Variable returned after &self gets executed
    fn return_type(&self) -> Type;
}

#[cfg(test)]
macro_rules! parse_type {
    ($s: literal) => {
        $s.parse::<Type>().unwrap()
    };
}

#[cfg(test)]
pub(crate) use parse_type;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        errors::ParseTypeError,
        variable::{parse_type, FunctionType, Type},
    };

    #[test]
    fn from_str() {
        assert_eq!(Type::from_str("int"), Ok(Type::Int));
        assert_eq!(Type::from_str("float"), Ok(Type::Float));
        assert_eq!(Type::from_str("string"), Ok(Type::String));
        assert_eq!(Type::from_str("any"), Ok(Type::Any));
        assert_eq!(Type::from_str("int"), Ok(Type::Int));
        assert_eq!(Type::from_str("()"), Ok(Type::Void));
        assert_eq!(
            Type::from_str("function()->()"),
            Ok(FunctionType {
                params: [].into(),
                return_type: Type::Void
            }
            .into())
        );
        assert_eq!(
            Type::from_str("function(string, int|float)->(int|string)"),
            Ok(FunctionType {
                params: [Type::String, Type::Int | Type::Float].into(),
                return_type: Type::Int | Type::String
            }
            .into())
        );
        assert_eq!(Type::from_str("[int]"), Ok([Type::Int].into()));
        assert_eq!(Type::from_str("[float]"), Ok([Type::Float].into()));
        assert_eq!(Type::from_str("[string]"), Ok([Type::String].into()));
        assert_eq!(Type::from_str("[any]"), Ok([Type::Any].into()));
        assert_eq!(Type::from_str("[int]"), Ok([Type::Int].into()));
        assert_eq!(Type::from_str("[()]"), Ok([Type::Void].into()));
        assert_eq!(
            Type::from_str("[int | float]"),
            Ok([Type::Int | Type::Float].into())
        );
        assert_eq!(
            Type::from_str("(int, string)"),
            Ok((Type::Int, Type::String).into())
        );
        assert_eq!(
            Type::from_str("(int, string) | float"),
            Ok(Type::from((Type::Int, Type::String)) | Type::Float)
        );
        assert_eq!(Type::from_str("(int)"), Err(ParseTypeError));
        assert_eq!(
            Type::from_str("function(string, int|float)->int|string"),
            Ok(FunctionType {
                params: [Type::String, Type::Int | Type::Float].into(),
                return_type: Type::Int
            } | Type::String)
        );
        assert_eq!(Type::from_str("any | float"), Ok(Type::Any));
        assert_eq!(Type::from_str("[any | float]"), Ok([Type::Any].into()));
    }

    #[test]
    fn check_type_matches() {
        let types = [
            Type::Int,
            Type::Float,
            Type::Int,
            Type::String,
            Type::Int | Type::String,
            [Type::Any].into(),
            (Type::Float, Type::Int, Type::Int | Type::Float).into(),
            Type::Void,
            Type::EmptyArray,
        ];
        for var_type in types {
            assert!(Type::Never.matches(&var_type));
            assert!(var_type.matches(&var_type));
            assert!(var_type.matches(&Type::Any));
            assert!(var_type.matches(&(var_type.clone() | Type::Int)));
        }
        let var_type = Type::Array(Type::Int.into()) | [Type::Float];
        let var_type2 = [Type::Float | Type::Int].into();
        assert!(var_type.matches(&var_type));
        assert!(var_type.matches(&var_type2));
        assert!(var_type2.matches(&var_type2));
        assert!(!var_type2.matches(&var_type));
        let var_type = Type::Int | [Type::Float] | [Type::String];
        let var_type2 = Type::Int | [Type::Float | Type::String];
        assert!(var_type.matches(&var_type));
        assert!(var_type.matches(&var_type2));
        assert!(var_type2.matches(&var_type2));
        assert!(!var_type2.matches(&var_type));
    }

    #[test]
    fn check_flatten_tuple() {
        assert_eq!(
            parse_type!("(int, string)").flatten_tuple(),
            Some([Type::Int, Type::String].into())
        );
        assert_eq!(
            parse_type!("(int, string)|(string, int)").flatten_tuple(),
            Some([Type::Int | Type::String, Type::Int | Type::String].into())
        );

        assert_eq!(
            parse_type!("(int, string)|(string, int)|(any, (int, string))").flatten_tuple(),
            Some(
                [
                    Type::Any,
                    Type::Int | Type::String | (Type::Int, Type::String)
                ]
                .into()
            )
        );

        assert_eq!(
            parse_type!("(int, string, float)|(string, int, float)").flatten_tuple(),
            Some(
                [
                    Type::Int | Type::String,
                    Type::Int | Type::String,
                    Type::Float
                ]
                .into()
            )
        );
        assert_eq!(
            parse_type!("int|(string, int, float)").flatten_tuple(),
            None
        );
        assert_eq!(
            parse_type!("(int, string)|(string, int, float)").flatten_tuple(),
            None
        );
    }

    #[test]
    fn check_element_type() {
        assert_eq!(Type::EmptyArray.index_result(), Some(Type::Never));
        assert_eq!(
            (Type::EmptyArray | parse_type!("[int]")).index_result(),
            Some(Type::Int)
        );
        assert_eq!(parse_type!("[int]").index_result(), Some(Type::Int));
        assert_eq!(parse_type!("string").index_result(), Some(Type::String));
        assert_eq!(
            parse_type!("string|[string]").index_result(),
            Some(Type::String)
        );
        assert_eq!(
            parse_type!("string | [int]").index_result(),
            Some(Type::Int | Type::String)
        );
        assert_eq!(
            parse_type!("[float] | [int]").index_result(),
            Some(Type::Int | Type::Float)
        );
        assert_eq!(
            parse_type!("[int] | [string|float]").index_result(),
            Some(Type::Int | Type::String | Type::Float)
        );
        assert_eq!(parse_type!("[any]|float").index_result(), None);
        assert_eq!(parse_type!("int").index_result(), None);
        assert_eq!(parse_type!("[int]|float").index_result(), None);
        assert_eq!(parse_type!("any").index_result(), None);
        assert_eq!(parse_type!("string | (int, float)").index_result(), None);
    }

    #[test]
    fn function_return_type() {
        assert_eq!(
            parse_type!("function()->int").return_type(),
            Some(Type::Int)
        );
        assert_eq!(
            parse_type!("function()->int | function(int) -> float").return_type(),
            Some(Type::Int | Type::Float)
        );
        assert_eq!(
            parse_type!("function()->int | function(int) -> (float|string)").return_type(),
            Some(Type::Float | Type::Int | Type::String)
        );
        assert_eq!(
            parse_type!("function()->int | function(int) -> float | function(int, int)->any")
                .return_type(),
            Some(Type::Any)
        );
        assert_eq!(parse_type!("float").return_type(), None);
        assert_eq!(parse_type!("int").return_type(), None);
        assert_eq!(parse_type!("string").return_type(), None);
        assert_eq!(parse_type!("function()->int | float").return_type(), None);
    }
}
