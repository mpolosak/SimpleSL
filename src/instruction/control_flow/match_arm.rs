use crate::{
    instruction::{
        local_variable::{LocalVariable, LocalVariables},
        recreate_instructions, ExecResult, ExecStop, InstructionWithStr,
    },
    interpreter::Interpreter,
    variable::{ReturnType, Type, Typed, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;
use simplesl_parser::{unexpected, Rule};
use std::sync::Arc;

use super::if_else::return_type;

#[derive(Debug)]
pub struct MatchArm {
    pattern: Pattern,
    instructions: Arc<[InstructionWithStr]>,
}

impl MatchArm {
    pub fn new(pair: Pair<Rule>, local_variables: &mut LocalVariables) -> Result<Self, Error> {
        let match_rule = pair.as_rule();
        let mut inner = pair.into_inner();
        let mut pair = inner.next().unwrap();
        let pattern = match match_rule {
            Rule::match_type => {
                let ident: Arc<str> = pair.as_str().into();
                let var_type = Type::from(inner.next().unwrap());
                let pair = inner.next().unwrap();
                local_variables.new_layer();
                local_variables.insert(ident.clone(), LocalVariable::Other(var_type.clone()));
                let mut instructions = Vec::<InstructionWithStr>::new();
                InstructionWithStr::create(pair, local_variables, &mut instructions)?;
                let instructions = instructions.into();
                local_variables.drop_layer();
                let pattern = Pattern::Type { ident, var_type };
                return Ok(Self {
                    pattern,
                    instructions,
                });
            }
            Rule::match_value => {
                let inner_values = pair.into_inner();
                let values = inner_values
                    .map(|pair| {
                        let mut instructions = Vec::<InstructionWithStr>::new();
                        InstructionWithStr::create(pair, local_variables, &mut instructions)?;
                        Ok(Arc::<[InstructionWithStr]>::from(instructions))
                    })
                    .collect::<Result<Arc<[Arc<[InstructionWithStr]>]>, Error>>()?;
                pair = inner.next().unwrap();
                Pattern::Value(values)
            }
            Rule::match_other => Pattern::Other,
            rule => unexpected!(rule),
        };
        let mut instructions = Vec::<InstructionWithStr>::new();
        InstructionWithStr::create(pair, local_variables, &mut instructions)?;
        let instructions = instructions.into();
        Ok(Self {
            pattern,
            instructions,
        })
    }

    pub fn is_covering_type(&self, checked_type: &Type) -> bool {
        self.pattern.is_covering_type(checked_type)
    }

    pub fn covers(
        &self,
        variable: &Variable,
        interpreter: &mut Interpreter,
    ) -> Result<bool, ExecStop> {
        self.pattern.covers(variable, interpreter)
    }
    pub fn exec(&self, variable: Variable, interpreter: &mut Interpreter) -> ExecResult {
        if let Pattern::Type { ident, .. } = &self.pattern {
            interpreter.push_layer();
            interpreter.insert(ident.clone(), variable);
            interpreter.exec_all(&self.instructions)?;
            interpreter.pop_layer();
        } else {
            interpreter.exec_all(&self.instructions)?;
        }
        Ok(interpreter.result().unwrap().clone())
    }
    pub fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Self, ExecError> {
        let pattern = match &self.pattern {
            Pattern::Type { ident, var_type } => {
                local_variables.new_layer();
                local_variables.insert(ident.clone(), LocalVariable::Other(var_type.clone()));
                let instructions = recreate_instructions(&self.instructions, local_variables)?;
                local_variables.drop_layer();
                let pattern = Pattern::Type {
                    ident: ident.clone(),
                    var_type: var_type.clone(),
                };
                return Ok(Self {
                    pattern,
                    instructions,
                });
            }
            Pattern::Value(values) => {
                let values = values
                    .iter()
                    .map(|value| recreate_instructions(&value, local_variables))
                    .collect::<Result<Arc<[Arc<[InstructionWithStr]>]>, ExecError>>()?;
                Pattern::Value(values)
            }
            Pattern::Other => Pattern::Other,
        };
        let instructions = recreate_instructions(&self.instructions, local_variables)?;
        Ok(Self {
            pattern,
            instructions,
        })
    }
}

impl ReturnType for MatchArm {
    fn return_type(&self) -> Type {
        return_type(&self.instructions)
    }
}

#[derive(Debug)]
pub enum Pattern {
    Type { ident: Arc<str>, var_type: Type },
    Value(Arc<[Arc<[InstructionWithStr]>]>),
    Other,
}

impl Pattern {
    pub fn is_covering_type(&self, checked_type: &Type) -> bool {
        match self {
            Pattern::Type { var_type, .. } => checked_type.matches(var_type),
            Pattern::Value(_) => false,
            Pattern::Other => true,
        }
    }

    pub fn covers(
        &self,
        variable: &Variable,
        interpreter: &mut Interpreter,
    ) -> Result<bool, ExecStop> {
        Ok(match &self {
            Pattern::Other => true,
            Pattern::Type { var_type, .. } => variable.as_type().matches(var_type),
            Pattern::Value(values) => {
                for instructions in values.iter() {
                    interpreter.exec_all(instructions)?;
                    let match_variable = interpreter.result().unwrap();
                    if match_variable == variable {
                        return Ok(true);
                    }
                }
                false
            }
        })
    }
}
