use crate as simplesl;
use simplesl_macros::export;

#[export(String)]
mod inner {
    use crate as simplesl;
    use crate::variable::Variable;
    use simplesl_macros::var;
    pub use std::string::String;
    use std::sync::Arc;

    #[return_type([string])]
    pub fn split(string: &str, pat: &str) -> Arc<[Variable]> {
        string.split(pat).map(|slice| var!(slice)).collect()
    }

    pub fn replace(string: &str, from: &str, to: &str) -> String {
        string.replace(from, to.as_ref())
    }

    pub fn contains(string: &str, pat: &str) -> bool {
        string.contains(pat)
    }

    pub fn starts_with(string: &str, prefix: &str) -> bool {
        string.starts_with(prefix)
    }

    pub fn ends_with(string: &str, suffix: &str) -> bool {
        string.ends_with(suffix)
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

    pub fn trim(string: &str) -> &str {
        string.trim()
    }

    pub fn trim_start(string: &str) -> &str {
        string.trim_start()
    }

    pub fn trim_end(string: &str) -> &str {
        string.trim_end()
    }
}
