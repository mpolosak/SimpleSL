use crate::{
    instruction::{local_variable::LocalVariables},
    variable::Type,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Pattern {
    pub ident: Arc<str>,
    pub var_type: Option<Type>,
}

impl Pattern {
    pub fn create_instruction(pair: Pair<Rule>, _local_variables: &mut LocalVariables) -> Self {
        let mut inner = pair.into_inner();
        let ident = inner.next().unwrap().as_str().into();
        let var_type = inner.next().map(Type::from);
        Self { ident, var_type }
    }

    pub fn is_matched(&self, var_type: &Type) -> bool {
        self.var_type.as_ref().is_none_or(|st| var_type.matches(st))
    }
}

impl From<Arc<str>> for Pattern {
    fn from(value: Arc<str>) -> Self {
        Pattern {
            ident: value,
            var_type: None,
        }
    }
}
