use crate::{
    Error,
    instruction::pattern::Pattern,
    interpreter::VariableMap,
    variable::{Array, Type, Typed, Variable},
};
use itertools::Itertools;
use pest::{Parser, iterators::Pair};
use simplesl_parser::{Rule, SimpleSLParser, unexpected};
use std::{str::FromStr, sync::Arc};

impl FromStr for Variable {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let s = s.trim();
        let mut parse = SimpleSLParser::parse(Rule::only_var, s)?;
        let pair = parse.next().unwrap();
        Self::try_from(pair)
    }
}

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
            Rule::tuple_from_str => {
                let elements = pair
                    .into_inner()
                    .map(Self::try_from)
                    .collect::<Result<Arc<[Variable]>, Error>>()?;
                Ok(Self::Tuple(elements))
            }
            Rule::struct_from_str => {
                let mut vm = VariableMap::new();
                for (pattern, value) in pair.into_inner().tuples() {
                    let value_str = value.as_str().into();
                    let value = Self::try_from(value)?;
                    let var_type = value.as_type();
                    let pattern_str = pattern.as_str().into();
                    let pattern = Pattern::create_instruction(pattern, &var_type)?;
                    if !pattern.is_matched(&var_type) {
                        return Err(Error::SetPatternNotMatched {
                            ins: value_str,
                            var_type,
                            pattern: pattern_str,
                        });
                    }
                    pattern.insert_variables_into_vm(&mut vm, value);
                }
                Ok(Variable::Struct(vm.into()))
            }
            Rule::void => Ok(Variable::Void),
            _ => Err(Error::CannotBeParsed(pair.as_str().into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate as simplesl;
    use crate::{
        Error,
        variable::{Array, Type, Variable},
    };
    use proptest::prelude::*;
    use simplesl_macros::var_type;
    use std::{collections::HashMap, str::FromStr};

    #[test]
    fn from_str() {
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
    }

    #[test]
    fn struct_from_str() {
        let empty_struct = Variable::Struct(HashMap::from([]).into());
        assert_eq!(Variable::from_str("struct{}"), Ok(empty_struct.clone()));
        assert_eq!(
            Variable::from_str("struct{a:=5}"),
            Ok(Variable::Struct(
                HashMap::from([("a".into(), Variable::Int(5))]).into()
            ))
        );
        assert_eq!(
            Variable::from_str("struct{a:int=5}"),
            Ok(Variable::Struct(
                HashMap::from([("a".into(), Variable::Int(5))]).into()
            ))
        );
        assert_eq!(
            Variable::from_str(r#"struct{a:="hello", b:=struct{}}"#),
            Ok(Variable::Struct(
                HashMap::from([
                    ("a".into(), Variable::String("hello".into())),
                    ("b".into(), empty_struct.clone())
                ])
                .into()
            ))
        );
        assert_eq!(
            Variable::from_str(r#"struct{(a, b):=("hello", struct{})}"#),
            Ok(Variable::Struct(
                HashMap::from([
                    ("a".into(), Variable::String("hello".into())),
                    ("b".into(), empty_struct)
                ])
                .into()
            ))
        );
        assert_eq!(
            Variable::from_str(r#"struct{(a, b, c):(int, float, any)=(5, 5.5, "")}"#),
            Ok(Variable::Struct(
                HashMap::from([
                    ("a".into(), Variable::Int(5)),
                    ("b".into(), Variable::Float(5.5)),
                    ("c".into(), Variable::String("".into()))
                ])
                .into()
            ))
        );
    }

    #[test]
    fn struct_from_str_pattern_not_matched() {
        assert_eq!(
            Variable::from_str("struct{a:float=5}"),
            Err(Error::SetPatternNotMatched {
                ins: "5".into(),
                var_type: Type::Int,
                pattern: "a:float".into()
            })
        );
        assert_eq!(
            Variable::from_str(r#"struct{(a, b, c):(int, float, int)=(5, 5.5, "")}"#),
            Err(Error::SetPatternNotMatched {
                ins: r#"(5, 5.5, "")"#.into(),
                var_type: var_type!((int, float, string)),
                pattern: "(a, b, c):(int, float, int)".into()
            })
        );
        assert_eq!(
            Variable::from_str(r#"struct{(a, b, c):=(5, 5.5)}"#),
            Err(Error::SetPatternNotMatched {
                ins: r#"(5, 5.5)"#.into(),
                var_type: var_type!((int, float)),
                pattern: "(a, b, c):".into()
            })
        );
    }

    #[test]
    fn tuple_from_str() {
        assert_eq!(
            Variable::from_str(r#"(10, "45")"#),
            Ok(Variable::Tuple([10.into(), "45".into()].into()))
        )
    }

    #[test]
    fn struct_from_str_self_contradictory_pattern() {
        assert_eq!(
            Variable::from_str("struct{(a, b):float=5}"),
            Err(Error::SelfContradictoryPattern("(a, b):float".into()))
        );
        assert_eq!(
            Variable::from_str(r#"struct{(a, b, c):(int, float)=(5, 5.5, "")}"#),
            Err(Error::SelfContradictoryPattern(
                "(a, b, c):(int, float)".into()
            ))
        );
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
