use crate::{
    instruction::local_variable::LocalVariables,
    variable::{Type, Typed},
};
use pest::iterators::Pair;
use simplesl_parser::{Rule, unexpected};
use std::{iter::repeat_n, sync::Arc};

#[derive(Clone, Debug)]
pub enum DestructPattern {
    Ident(Arc<str>),
    Tuple(Arc<[Arc<str>]>),
}

impl DestructPattern {
    pub fn create_instruction(pair: Pair<Rule>, _local_variables: &mut LocalVariables) -> Self {
        let rule = pair.as_rule();
        match rule {
            Rule::ident => Self::Ident(pair.as_str().into()),
            Rule::destruct_tuple => {
                let idents = pair
                    .into_inner()
                    .map(|p| p.as_str())
                    .map(Arc::<str>::from)
                    .collect();
                Self::Tuple(idents)
            }
            rule => unexpected!(rule),
        }
    }
}

impl Typed for DestructPattern {
    fn as_type(&self) -> crate::variable::Type {
        match self {
            DestructPattern::Ident(_) => Type::Any,
            DestructPattern::Tuple(items) => {
                Type::Tuple(repeat_n(Type::Any, items.len()).collect())
            }
        }
    }
}
