mod array;
mod function_type;
mod multi_type;
mod try_from;
mod r#type;
use crate::{function::Function, join_debug, Error};
use enum_as_inner::EnumAsInner;
use match_any::match_any;
use pest::{iterators::Pair, Parser};
pub use r#type::{ReturnType, Type, Typed};
use simplesl_parser::{unexpected, Rule, SimpleSLParser};
use std::{fmt, io, str::FromStr, sync::Arc};
pub use typle::typle;
pub use {array::Array, function_type::FunctionType, multi_type::MultiType};

#[derive(Clone, EnumAsInner)]
pub enum Variable {
    Int(i64),
    Float(f64),
    String(Arc<str>),
    Function(Arc<Function>),
    Array(Arc<Array>),
    Tuple(Arc<[Variable]>),
    Void,
}

impl Typed for Variable {
    fn as_type(&self) -> Type {
        match_any! {self,
            Variable::Int(_) => Type::Int,
            Variable::Float(_) => Type::Float,
            Variable::String(_) => Type::String,
            Variable::Function(var) | Variable::Array(var) => var.as_type(),
            Variable::Tuple(elements) => {
                let types = elements.iter().map(Variable::as_type).collect();
                Type::Tuple(types)
            },
            Variable::Void => Type::Void
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match_any! {self,
            Variable::Int(value)
            | Variable::Float(value)
            | Variable::String(value)
            | Variable::Function(value)
            | Variable::Array(value) => write!(f, "{value}"),
            Variable::Tuple(elements) => write!(f, "({})", join_debug(elements.as_ref(), ", ")),
            Variable::Void => write!(f, "()")
        }
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match_any! { self,
            Self::Int(value)
            | Self::Float(value)
            | Self::String(value) => write!(f, "{value:?}"),
            other => write!(f, "{other}")
        }
    }
}

impl FromStr for Variable {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let s = s.trim();
        let parse = SimpleSLParser::parse(Rule::only_var, s)?;
        let pairs: Box<[Pair<Rule>]> = parse.collect();
        Self::try_from(pairs[0].clone())
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        match_any! {(self, other),
            (Variable::Array(value1), Variable::Array(value2))
            | (Variable::Int(value1), Variable::Int(value2))
            | (Variable::Float(value1), Variable::Float(value2))
            | (Variable::String(value1), Variable::String(value2))
            | (Variable::Tuple(value1), Variable::Tuple(value2)) => value1 == value2,
            (Variable::Function(value1), Variable::Function(value2)) => Arc::ptr_eq(value1, value2),
            (Variable::Void, Variable::Void) => true,
            _ => false
        }
    }
}

impl Eq for Variable {}

#[doc(hidden)]
impl TryFrom<Pair<'_, Rule>> for Variable {
    type Error = Error;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Error> {
        fn parse_int(pair: Pair<Rule>) -> Result<i64, Error> {
            let pair = pair.into_inner().next().unwrap();
            match pair.as_rule() {
                Rule::binary_int => parse_int_with_radix(pair, 2),
                Rule::octal_int => parse_int_with_radix(pair, 8),
                Rule::decimal_int => parse_int_with_radix(pair, 10),
                Rule::hexadecimal_int => parse_int_with_radix(pair, 16),
                rule => unexpected!(rule),
            }
        }
        fn parse_int_with_radix(pair: Pair<Rule>, radix: u32) -> Result<i64, Error> {
            let str = pair.as_str();
            let inner = pair
                .into_inner()
                .next()
                .unwrap()
                .as_str()
                .replace([' ', '_'], "");
            i64::from_str_radix(&inner, radix).map_err(|_| {
                return Error::IntegerOverflow(str.into());
            })
        }
        match pair.as_rule() {
            Rule::minus_int => {
                parse_int(pair.into_inner().next().unwrap()).map(|value| Variable::Int(-value))
            }
            Rule::int => parse_int(pair).map(Self::from),
            Rule::minus_float => {
                let Ok(value) = pair.as_str().replace([' ', '_'], "").parse::<f64>() else {
                    return Err(Error::CannotBeParsed(pair.as_str().into()));
                };
                Ok(Variable::Float(value))
            }
            Rule::float => {
                let Ok(value) = pair.as_str().replace([' ', '_'], "").parse::<f64>() else {
                    return Err(Error::CannotBeParsed(pair.as_str().into()));
                };
                Ok(Variable::Float(value))
            }
            Rule::string => {
                let value = pair.into_inner().next().unwrap().as_str();
                let value = unescaper::unescape(value)?;
                Ok(value.into())
            }
            Rule::array_from_str => {
                let elements = pair
                    .into_inner()
                    .map(Self::try_from)
                    .collect::<Result<Arc<[Variable]>, Error>>()?;
                let element_type = elements
                    .iter()
                    .map(Typed::as_type)
                    .reduce(Type::concat)
                    .unwrap_or(Type::Never);
                Ok(Array {
                    element_type,
                    elements,
                }
                .into())
            }
            Rule::array_repeat_from_str => {
                let mut inner = pair.into_inner();
                let value = Variable::try_from(inner.next().unwrap())?;
                let len_pair = inner.next().unwrap();
                let len = parse_int(len_pair)?;
                Ok(Array::new_repeat(value, len as usize).into())
            }
            Rule::void => Ok(Variable::Void),
            _ => Err(Error::CannotBeParsed(pair.as_str().into())),
        }
    }
}

impl From<i64> for Variable {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<bool> for Variable {
    fn from(value: bool) -> Self {
        Self::Int(value.into())
    }
}

impl From<usize> for Variable {
    fn from(value: usize) -> Self {
        Self::Int(value as i64)
    }
}

impl From<f64> for Variable {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<Arc<str>> for Variable {
    fn from(value: Arc<str>) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Variable {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<String> for Variable {
    fn from(value: String) -> Self {
        Self::String(value.into())
    }
}

impl From<Function> for Variable {
    fn from(value: Function) -> Self {
        Self::Function(value.into())
    }
}

impl From<Arc<Function>> for Variable {
    fn from(value: Arc<Function>) -> Self {
        Self::Function(value)
    }
}

impl From<()> for Variable {
    fn from(_value: ()) -> Self {
        Self::Void
    }
}

impl<T: Into<Variable>> From<Option<T>> for Variable {
    fn from(value: Option<T>) -> Self {
        value.map_or(Variable::Void, Into::into)
    }
}

impl<T: Into<Variable>> From<io::Result<T>> for Variable {
    fn from(value: io::Result<T>) -> Self {
        value.map_or_else(Into::into, Into::into)
    }
}

impl From<io::Error> for Variable {
    fn from(value: io::Error) -> Self {
        (value.kind().into(), value.to_string().into()).into()
    }
}

impl From<io::ErrorKind> for Variable {
    fn from(value: io::ErrorKind) -> Self {
        (value as i64).into()
    }
}

impl From<Array> for Variable {
    fn from(value: Array) -> Self {
        Variable::Array(value.into())
    }
}

impl From<Arc<Array>> for Variable {
    fn from(value: Arc<Array>) -> Self {
        Variable::Array(value)
    }
}

impl From<Arc<[Variable]>> for Variable {
    fn from(value: Arc<[Variable]>) -> Self {
        Array::from(value).into()
    }
}

impl From<Vec<Variable>> for Variable {
    fn from(value: Vec<Variable>) -> Self {
        Array::from(value).into()
    }
}

impl<const N: usize> From<[Variable; N]> for Variable {
    fn from(value: [Variable; N]) -> Self {
        Array::from(value).into()
    }
}

#[typle(Tuple for 2..=12)]
impl<T: Tuple<Variable>> From<T> for Variable {
    fn from(value: T) -> Self {
        let vars: [Variable; Tuple::LEN] = value.into();
        Variable::Tuple(vars.into())
    }
}

pub fn is_correct_variable_name(name: &str) -> bool {
    let Ok(parse) = SimpleSLParser::parse(Rule::ident, name) else {
        return false;
    };
    parse.as_str() == name
}

#[cfg(test)]
mod tests {
    use crate::variable::{Array, Variable};

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Variable>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Variable>();
    }
    #[test]
    fn check_is_correct_variable_name() {
        use crate::variable::is_correct_variable_name;
        assert!(is_correct_variable_name("aDd"));
        assert!(is_correct_variable_name("Ad_d5"));
        assert!(is_correct_variable_name("_ad_d"));
        assert!(!is_correct_variable_name("5add"));
        assert!(!is_correct_variable_name("^$$ddd"));
        assert!(!is_correct_variable_name(""));
        assert!(!is_correct_variable_name("12"));
        assert!(!is_correct_variable_name("%"));
        assert!(!is_correct_variable_name("return"));
        assert!(is_correct_variable_name("return5"));
        assert!(is_correct_variable_name("areturn"));
    }
    #[test]
    fn check_variable_from_str() {
        use crate::variable::Variable;
        use std::str::FromStr;
        assert_eq!(Variable::from_str(" 15"), Ok(Variable::Int(15)));
        assert_eq!(Variable::from_str(" -7"), Ok(Variable::Int(-7)));
        assert_eq!(Variable::from_str(" 1__00_5__"), Ok(Variable::Int(1005)));
        assert_eq!(Variable::from_str(" 0b111 "), Ok(Variable::Int(0b111)));
        assert_eq!(Variable::from_str(" 0b_1_11_ "), Ok(Variable::Int(0b111)));
        assert_eq!(Variable::from_str(" 0o176 "), Ok(Variable::Int(0o176)));
        assert_eq!(Variable::from_str(" 0o_17__6_ "), Ok(Variable::Int(0o176)));
        assert_eq!(Variable::from_str(" 0xFA6 "), Ok(Variable::Int(0xFA6)));
        assert_eq!(
            Variable::from_str(" 0x__FA___6___ "),
            Ok(Variable::Int(0xFA6))
        );
        assert_eq!(Variable::from_str(" 7.5 "), Ok(Variable::Float(7.5)));
        assert_eq!(Variable::from_str(" -5.0 "), Ok(Variable::Float(-5.0)));
        assert_eq!(Variable::from_str(" 5e25 "), Ok(Variable::Float(5e25)));
        assert_eq!(Variable::from_str(" 6E_25 "), Ok(Variable::Float(6E25)));
        assert_eq!(Variable::from_str(" 6E-25 "), Ok(Variable::Float(6E-25)));
        assert_eq!(Variable::from_str(" 6.5e-5 "), Ok(Variable::Float(6.5e-5)));
        assert_eq!(Variable::from_str("()"), Ok(Variable::Void));
        assert_eq!(
            Variable::from_str(r#""print \"""#),
            Ok(Variable::String("print \"".into()))
        );
        assert!(Variable::from_str(r#""print" """#).is_err());
        assert_eq!(
            Variable::from_str("[14; 5]"),
            Ok(Array::new_repeat(Variable::Int(14), 5).into())
        );
        assert!(Variable::from_str("[14; 5.5]").is_err());
        assert_eq!(
            Variable::from_str("[45, 4, 3.5]"),
            Ok(Variable::from([
                Variable::Int(45),
                Variable::Int(4),
                Variable::Float(3.5)
            ]))
        );
        assert_eq!(Variable::from_str("[]"), Ok(Variable::from([])))
    }
}
