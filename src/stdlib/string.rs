use crate as simplesl;
use simplesl_macros::export;

#[export]
pub(crate) mod add_string {
    use crate as simplesl;
    use crate::variable::Variable;
    use match_any::match_any;
    use simplesl_macros::var;
    use std::sync::Arc;

    #[return_type([string])]
    pub fn split(string: &str, pat: &str) -> Arc<[Variable]> {
        string.split(pat).map(|slice| var!(slice)).collect()
    }

    pub fn replace(string: &str, from: &str, to: &str) -> String {
        string.replace(from, to.as_ref())
    }

    pub fn string_contains(string: &str, pat: &str) -> bool {
        string.contains(pat)
    }

    #[return_type([string])]
    pub fn chars(string: &str) -> Arc<[Variable]> {
        string
            .chars()
            .map(|char| Variable::from(char.to_string()))
            .collect()
    }

    #[return_type([int])]
    pub fn bytes(string: &str) -> Arc<[Variable]> {
        string
            .bytes()
            .map(|byte| Variable::from(byte as i64))
            .collect()
    }

    pub fn str_from_utf8(#[var_type([int])] array: &[Variable]) -> Option<String> {
        let bytes = array
            .iter()
            .map(Variable::as_int)
            .map(Option::unwrap)
            .map(|val| *val as u8)
            .collect();
        String::from_utf8(bytes).ok()
    }

    pub fn str_from_utf8_lossy(#[var_type([int])] array: &[Variable]) -> std::sync::Arc<str> {
        let bytes = array
            .iter()
            .map(Variable::as_int)
            .map(Option::unwrap)
            .map(|val| *val as u8)
            .collect::<Box<[u8]>>();
        String::from_utf8_lossy(&bytes).into()
    }

    pub fn to_lowercase(string: &str) -> String {
        string.to_lowercase()
    }

    pub fn to_uppercase(string: &str) -> String {
        string.to_uppercase()
    }

    pub fn len(#[var_type([any]|string)] variable: &Variable) -> usize {
        match_any! { variable,
            Variable::Array(var) | Variable::String(var) => var.len(),
            _ => unreachable!()
        }
    }
}
