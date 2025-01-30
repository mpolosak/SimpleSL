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
