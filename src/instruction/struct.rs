use super::{Exec, ExecResult, Instruction, Recreate, local_variable::LocalVariables};
use crate::{
    errors::ErrorWithRange,
    instruction::{BaseInstruction, ident::Ident, parsing_context::ParsingContext, set::Set},
    interpreter::{Interpreter, VariableMap},
    range_from_pest_span,
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone)]
pub struct Struct {
    pub sets: Arc<[Set]>,
}

impl BaseInstruction for Struct {}

impl Struct {
    pub fn create_instruction(
        pair: Pair<Rule>,
        parsing_context: &ParsingContext,
        errors: &mut Vec<ErrorWithRange>,
    ) -> Instruction {
        let sets = pair
            .into_inner()
            .map(|pair| {
                if pair.as_rule() == Rule::ident {
                    let pest_span = pair.as_span();
                    let span = range_from_pest_span(&pest_span);
                    let instruction = Ident::create_instruction(span, parsing_context, errors);
                    let ident = pest_span.as_str().into();
                    return Set::new_ident(ident, instruction);
                }
                let mut local_variables = parsing_context.create_layer();
                Set::create(pair, &mut local_variables, errors)
            })
            .collect();
        Self { sets }.into()
    }
}

impl Exec for Struct {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let mut vm = VariableMap::new();
        for set in self.sets.iter() {
            let mut interpreter = interpreter.create_layer();
            set.exec(&mut interpreter)?;
            vm.extend(interpreter.drop_layer());
        }
        Ok(Variable::Struct(vm.into()))
    }
}

impl Recreate for Struct {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let sets = self
            .sets
            .iter()
            .map(|set| {
                let mut local_variables = local_variables.create_layer();
                set.inner_recreate(&mut local_variables)
            })
            .collect();
        Self { sets }.into()
    }
}

impl ReturnType for Struct {
    fn return_type(&self) -> Type {
        let mut tm = HashMap::new();
        for set in self.sets.iter() {
            set.pattern.insert_types(&mut tm);
        }
        Type::Struct(tm.into())
    }
}
