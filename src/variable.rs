mod array;
mod from_str;
mod function_type;
mod multi_type;
mod r#mut;
mod struct_type;
mod try_from;
mod r#type;
mod type_of;
use crate::{self as simplesl, function::Function, interpreter::VariableMap};
use derive_more::{Display, From};
use enum_as_inner::EnumAsInner;
use itertools::Itertools;
use match_any::match_any;
use pest::Parser;
use simplesl_macros::var;
use simplesl_parser::{Rule, SimpleSLParser};
use std::{collections::HashMap, fmt, io, sync::Arc};
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
                let elements = vm.iter().map(|(key, value)| format!("{}:={}", key, value.debug(depth))).join(", ");
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
    use crate as simplesl;
    use crate::variable::Variable;
    use proptest::proptest;
    use simplesl_macros::var;

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
    fn display_true() {
        assert_eq!(format!("{}", Variable::Bool(true)), "true")
    }

    #[test]
    fn display_false() {
        assert_eq!(format!("{}", Variable::Bool(false)), "false")
    }

    #[test]
    fn display_void() {
        assert_eq!(format!("{}", Variable::Void), "()")
    }

    #[test]
    fn display_tuple() {
        assert_eq!(
            format!("{}", var!((5, "a", (4.5, false)))),
            r#"(5, "a", (4.5, false))"#
        )
    }

    #[test]
    fn display_array() {
        assert_eq!(
            format!("{}", var!([5, "a", 4.5, false])),
            r#"[5, "a", 4.5, false]"#
        )
    }

    #[test]
    fn display_empty_array() {
        assert_eq!(format!("{}", var!([])), r#"[]"#)
    }

    #[test]
    fn display_empty_struct() {
        assert_eq!(format!("{}", var!(struct{})), r#"struct{}"#)
    }

    #[test]
    fn display_one_field_struct() {
        assert_eq!(format!("{}", var!(struct{a:=5})), r#"struct{a:=5}"#)
    }

    proptest! {
        #[test]
        fn display_int(int: i64) {
            assert_eq!(format!("{}", Variable::Int(int)), format!("{int}"))
        }

        #[test]
        fn display_float(float: f64) {
            assert_eq!(format!("{}", Variable::Float(float)), format!("{float}"))
        }

        #[test]
        fn display_string(s in "\\PC*") {
            assert_eq!(format!("{}", Variable::String(s.clone().into())), format!("{s}"))
        }
    }
}
