use crate as simplesl;
use simplesl_macros::export;

#[export]
mod add_string {
    use crate::variable::Variable;
    use match_any::match_any;
    use simplesl_macros::var;
    use std::sync::Arc;

    #[return_type([string])]
    fn split(string: &str, pat: &str) -> Arc<[Variable]> {
        string.split(pat).map(|slice| var!(slice)).collect()
    }

    fn replace(string: &str, from: &str, to: &str) -> String {
        string.replace(from, to.as_ref())
    }

    fn string_contains(string: &str, pat: &str) -> bool {
        string.contains(pat)
    }

    #[return_type([string])]
    fn chars(string: &str) -> Arc<[Variable]> {
        string
            .chars()
            .map(|char| Variable::from(char.to_string()))
            .collect()
    }

    fn to_lowercase(string: &str) -> String {
        string.to_lowercase()
    }

    fn to_uppercase(string: &str) -> String {
        string.to_uppercase()
    }

    fn len(#[var_type([any]|string)] variable: &Variable) -> usize {
        match_any! { variable,
            Variable::Array(var) | Variable::String(var) => var.len(),
            _ => unreachable!()
        }
    }
}
