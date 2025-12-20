use crate::{
    function::Param, instruction::{
        local_variable::{LocalVariable, LocalVariables},
        pattern::destruct_pattern::DestructPattern,
    }, variable::{Type, Typed, Variable}, Error, Interpreter
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;
pub mod destruct_pattern;

#[derive(Debug, Clone)]
pub struct Pattern {
    pub destruct_pattern: DestructPattern,
    pub var_type: Type,
}

impl Pattern {
    pub fn new_ident_pattern(ident: Arc<str>, var_type: Type) -> Self {
        Pattern {
            destruct_pattern: DestructPattern::Ident(ident),
            var_type,
        }
    }

    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
        ex_type: &Type,
    ) -> Result<Self, Error> {
        let str = pair.as_str().into();
        let mut inner = pair.into_inner();
        let destruct_pattern_pair = inner.next().unwrap();
        let destruct_pattern =
            DestructPattern::create_instruction(destruct_pattern_pair, local_variables);
        let var_type = inner
            .next()
            .map(Type::from);
        let destruct_pattern_type = destruct_pattern.as_type();
        let Some(var_type) = var_type else {
            let ct = destruct_pattern_type.conjoin(ex_type);
            let var_type = if ct!=Type::Never {ct} else {destruct_pattern_type};
            return Ok(Self {
                destruct_pattern,
                var_type,
            })
        };
        if !var_type.matches(&destruct_pattern_type) {
            return Err(Error::SelfContradictoryPattern(str));
        }

        Ok(Self {
            destruct_pattern,
            var_type,
        })
    }

    pub fn is_matched(&self, var_type: &Type) -> bool {
        var_type.matches(&self.var_type)
    }

    pub fn insert_local_variables(&self, local_variables: &mut LocalVariables) {
        match &self.destruct_pattern {
            DestructPattern::Ident(ident) => {
                local_variables.insert(ident.clone(), LocalVariable::Other(self.var_type.clone()))
            }
            DestructPattern::Tuple(idents) => {
                let tuple = self.var_type.clone().flatten_tuple().unwrap();
                for (ident, var_type) in idents.iter().cloned().zip(tuple.iter().cloned()) {
                    local_variables.insert(ident, LocalVariable::Other(var_type));
                }
            }
        }
    }

    pub fn insert_variables(&self, interpreter: &mut Interpreter, variable: Variable) {
        match &self.destruct_pattern {
            DestructPattern::Ident(ident) => interpreter.insert(ident.clone(), variable),
            DestructPattern::Tuple(idents) =>  {
                let tuple = variable.into_tuple().unwrap();
                for (ident, var) in idents.iter().cloned().zip(tuple.iter().cloned()) {
                    interpreter.insert(ident, var);
                }
            },
        }
    }
}

impl From<Param> for Pattern {
    fn from(value: Param) -> Self {
        Self {
            destruct_pattern: DestructPattern::Ident(value.name),
            var_type: value.var_type,
        }
    }
}
