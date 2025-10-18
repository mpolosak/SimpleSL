mod array;
mod function_type;
mod multi_type;
mod r#mut;
mod struct_type;
mod try_from;
mod r#type;
mod type_of;
use crate::{self as simplesl, Error, function::Function, interpreter::VariableMap};
use derive_more::{Display, From};
use enum_as_inner::EnumAsInner;
use itertools::Itertools;
use match_any::match_any;
use pest::{Parser, iterators::Pair};
use simplesl_macros::var;
use simplesl_parser::{Rule, SimpleSLParser, unexpected};
use std::{collections::HashMap, fmt, io, str::FromStr, sync::Arc};
pub use r#type::{ReturnType, Type, Typed};
use typle::typle;
pub use {
    array::Array, function_type::FunctionType, multi_type::MultiType, r#mut::Mut,
    struct_type::StructType, type_of::TypeOf,
};

#[derive(Clone, Display, EnumAsInner, From)]
#[display("{}", self.string(0))]
pub enum Variable {
    #[from]
    Bool(bool),
    #[from(i32, u32, i64)]
    Int(i64),
    #[from]
    Float(f64),
    #[from(Arc<str>, &str, String)]
    String(Arc<str>),
    #[from(Function, Arc<Function>)]
    Function(Arc<Function>),
    #[from(Array, Arc<Array>)]
    Array(Arc<Array>),
    Tuple(Arc<[Variable]>),
    #[from(Mut, Arc<Mut>)]
    Mut(Arc<Mut>),
    Struct(Arc<VariableMap>),
    #[from]
    Void,
}

impl Variable {
    fn string(&self, depth: u8) -> String {
        if depth > 5 {
            return "..".into();
        }
        match_any! {self,
            Variable::Bool(value)
            | Variable::Int(value)
            | Variable::Float(value)
            | Variable::String(value)
            | Variable::Function(value) => format!("{value}"),
            Variable::Array(value) => value.string(depth),
            Variable::Mut(value) => value.string(depth+1),
            Variable::Tuple(elements) => format!("({})", elements.iter().map(|v| v.debug(depth+1)).collect::<Box<[_]>>().join(", ")),
            Variable::Struct(vm) => {
                let elements = vm.iter().map(|(key, value)| format!("{}={}", key, value.debug(depth))).join(", ");
                format!("struct{{{elements}}}")
            },
            Variable::Void => format!("()")
        }
    }

    fn debug(&self, depth: u8) -> String {
        match_any! { self,
            Self::Int(value)
            | Self::Float(value)
            | Self::String(value) => format!("{value:?}"),
            _ => self.string(depth)
        }
    }

    pub fn of_type(var_type: &Type) -> Option<Self> {
        match var_type {
            Type::Bool => Some(false.into()),
            Type::Int => Some(0.into()),
            Type::Float => Some(0.0.into()),
            Type::String => Some("".into()),
            Type::Function(arc) => Some(Function::of_type(arc).into()),
            Type::Array(arc) => Some(
                Array {
                    element_type: arc.as_ref().clone(),
                    elements: [].into(),
                }
                .into(),
            ),
            Type::Tuple(arc) => {
                let elements = arc
                    .iter()
                    .map(Self::of_type)
                    .collect::<Option<Arc<[Variable]>>>()?;
                Some(Self::Tuple(elements))
            }
            Type::Void => Some(Variable::Void),
            Type::Multi(multi_type) => multi_type.iter().next().and_then(Self::of_type),
            Type::Mut(arc) => Some(
                Mut {
                    var_type: arc.as_ref().clone(),
                    variable: Variable::of_type(arc)?.into(),
                }
                .into(),
            ),
            Type::Struct(s) => {
                let vm: Option<VariableMap> =
                    s.0.iter()
                        .map(|(key, value)| Some((key.clone(), Variable::of_type(value)?)))
                        .collect();
                Some(Variable::Struct(vm?.into()))
            }
            Type::Any => Some(Variable::Void),
            Type::Never => None,
        }
    }
}

impl Typed for Variable {
    fn as_type(&self) -> Type {
        match_any! {self,
            Variable::Bool(_) => Type::Bool,
            Variable::Int(_) => Type::Int,
            Variable::Float(_) => Type::Float,
            Variable::String(_) => Type::String,
            Variable::Function(var) | Variable::Array(var) | Variable::Mut(var) => var.as_type(),
            Variable::Tuple(elements) => {
                let types = elements.iter().map(Variable::as_type).collect();
                Type::Tuple(types)
            },
            Variable::Struct(vm) => {
                let tm: HashMap<Arc<str>, Type> = vm.iter().map(|(key, value)|{
                    (key.clone(), value.as_type())
                }).collect();
                StructType(Arc::from(tm)).into()
            },
            Variable::Void => Type::Void
        }
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.debug(0))
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
            | (Variable::Bool(value1), Variable::Bool(value2))
            | (Variable::Int(value1), Variable::Int(value2))
            | (Variable::Float(value1), Variable::Float(value2))
            | (Variable::String(value1), Variable::String(value2))
            | (Variable::Tuple(value1), Variable::Tuple(value2))
            | (Variable::Struct(value1), Variable::Struct(value2)) => value1 == value2,
            (Variable::Function(value1), Variable::Function(value2))
            | (Variable::Mut(value1), Variable::Mut(value2)) => Arc::ptr_eq(value1, value2),
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
            i64::from_str_radix(&inner, radix).map_err(|_| Error::IntegerOverflow(str.into()))
        }
        match pair.as_rule() {
            Rule::r#true => Ok(Variable::Bool(true)),
            Rule::r#false => Ok(Variable::Bool(false)),
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
            Rule::struct_from_str => {
                let vm = pair
                    .into_inner()
                    .tuples()
                    .map(|(ident, value)| {
                        let ident: Arc<str> = ident.as_str().into();
                        let value = Variable::try_from(value)?;
                        Ok((ident, value))
                    })
                    .collect::<Result<VariableMap, Error>>()?;
                Ok(Variable::Struct(vm.into()))
            }
            Rule::void => Ok(Variable::Void),
            _ => Err(Error::CannotBeParsed(pair.as_str().into())),
        }
    }
}

impl From<usize> for Variable {
    fn from(value: usize) -> Self {
        Self::Int(value as i64)
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
        let error_code = value.kind();
        let msg = value.to_string();
        var!(
            struct{
                error_code,
                msg
            }
        )
    }
}

impl From<io::ErrorKind> for Variable {
    fn from(value: io::ErrorKind) -> Self {
        (value as i64).into()
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
    use std::{collections::HashMap, str::FromStr};

    use crate::variable::{Array, Variable};
    use proptest::prelude::*;

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
    fn from_str() {
        use crate::variable::Variable;
        use std::str::FromStr;
        assert_eq!(Variable::from_str("true"), Ok(Variable::Bool(true)));
        assert_eq!(Variable::from_str("false"), Ok(Variable::Bool(false)));
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
        assert_eq!(Variable::from_str("[]"), Ok(Variable::from([])));
        let empty_struct = Variable::from_str("struct{}");
        assert_eq!(empty_struct, Ok(Variable::Struct(HashMap::from([]).into())));
        assert_eq!(
            Variable::from_str("struct{a:=5}"),
            Ok(Variable::Struct(
                HashMap::from([("a".into(), Variable::Int(5))]).into()
            ))
        );
        assert_eq!(
            Variable::from_str(r#"struct{a:="hello", b:=struct{}}"#),
            Ok(Variable::Struct(
                HashMap::from([
                    ("a".into(), Variable::String("hello".into())),
                    ("b".into(), empty_struct.unwrap())
                ])
                .into()
            ))
        )
    }

    proptest! {
        #[test]
        fn variable_from_str_doesnt_crash(s in "\\PC*"){
            let _ = Variable::from_str(&s);
        }

        #[test]
        fn variable_from_str_int(a: i64){
            assert_eq!(Variable::from_str(&a.to_string()), Ok(Variable::Int(a)))
        }

        #[test]
        fn variable_from_str_float(a: f64){
            assert_eq!(Variable::from_str(&format!("{a:?}")), Ok(Variable::Float(a)))
        }

        #[test]
        fn variable_from_str_string(s in "\\PC*"){
            assert_eq!(Variable::from_str(&format!("{:?}", Variable::String(s.clone().into()))), Ok(Variable::String(s.into())))
        }
    }
}
