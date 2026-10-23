use crate::variable::{StructType, Type, Typed};
use pest::iterators::Pair;
use simplesl_parser::{Rule, unexpected};
use std::{collections::HashMap, iter::repeat_n, sync::Arc};

#[derive(Clone, Debug)]
pub enum DestructPattern {
    Ident(Arc<str>),
    Tuple(Arc<[Arc<str>]>),
    Struct(Arc<[(Arc<str>, Arc<str>)]>),
    Array(Arc<[Arc<str>]>),
}

impl DestructPattern {
    pub fn create_instruction(pair: Pair<Rule>) -> Self {
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
            Rule::destruct_array => {
                let idents = pair
                    .into_inner()
                    .map(|p| p.as_str())
                    .map(Arc::<str>::from)
                    .collect();
                Self::Array(idents)
            }
            Rule::destruct_struct => {
                let fields = pair
                    .into_inner()
                    .map(|field| {
                        let mut idents = field
                            .into_inner()
                            .map(|pair| Arc::<str>::from(pair.as_str()));
                        let field_name = idents.next().unwrap();
                        let binding = idents.next().unwrap_or_else(|| field_name.clone());
                        (field_name, binding)
                    })
                    .collect();
                Self::Struct(fields)
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
            DestructPattern::Array(_) => Type::Array(Type::Any.into()),
            DestructPattern::Struct(fields) => Type::Struct(StructType::from(
                fields
                    .iter()
                    .map(|(field, _)| (field.clone(), Type::Any))
                    .collect::<HashMap<_, _>>(),
            )),
        }
    }
}
