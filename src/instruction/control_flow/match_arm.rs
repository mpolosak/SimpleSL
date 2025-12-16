use crate::{
    instruction::{
        local_variable::{LocalVariable, LocalVariables}, pattern::Pattern, recreate_instructions, Exec, ExecResult, ExecStop, InstructionWithStr
    }, interpreter::Interpreter, variable::{ReturnType, Type, Typed, Variable}, Error, ExecError
};
use pest::iterators::Pair;
use simplesl_parser::{Rule, unexpected};
use std::sync::Arc;

#[derive(Debug)]
pub enum MatchArm {
    Type {
        pattern: Pattern,
        instruction: InstructionWithStr,
    },
    Value(Arc<[InstructionWithStr]>, InstructionWithStr),
    Other(InstructionWithStr),
}

impl MatchArm {
    pub fn new(pair: Pair<Rule>, local_variables: &mut LocalVariables) -> Result<Self, Error> {
        let match_rule = pair.as_rule();
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        match match_rule {
            Rule::match_pattern => {
                let pattern = Pattern::create_instruction(pair, local_variables);
                let pair = inner.next().unwrap();
                let mut local_variables = local_variables.create_layer();
                let var_type = pattern.var_type.clone().unwrap();
                local_variables.insert(pattern.ident.clone(), LocalVariable::Other(var_type));
                let instruction = InstructionWithStr::new(pair, &mut local_variables)?;
                Ok(Self::Type {
                    pattern,
                    instruction,
                })
            }
            Rule::match_value => {
                let inner_values = pair.into_inner();
                let values = inner_values
                    .map(|pair| InstructionWithStr::new(pair, local_variables))
                    .collect::<Result<Arc<[InstructionWithStr]>, Error>>()?;
                let pair = inner.next().unwrap();
                let instruction = InstructionWithStr::new(pair, local_variables)?;
                Ok(Self::Value(values, instruction))
            }
            Rule::match_other => {
                let instruction = InstructionWithStr::new(pair, local_variables)?;
                Ok(Self::Other(instruction))
            }
            rule => unexpected!(rule),
        }
    }
    pub fn is_covering_type(&self, checked_type: &Type) -> bool {
        match self {
            Self::Value(..) => false,
            Self::Other(_) => true,
            Self::Type { pattern, .. } => pattern.is_matched(checked_type),
        }
    }
    pub fn covers(
        &self,
        variable: &Variable,
        interpreter: &mut Interpreter,
    ) -> Result<bool, ExecStop> {
        Ok(match self {
            MatchArm::Other(_) => true,
            MatchArm::Type { pattern, .. } => pattern.is_matched(&variable.as_type()),
            MatchArm::Value(instructions, _) => {
                for instruction in instructions.iter() {
                    let match_variable = instruction.exec(interpreter)?;
                    if match_variable == *variable {
                        return Ok(true);
                    }
                }
                false
            }
        })
    }
    pub fn exec(&self, variable: Variable, interpreter: &mut Interpreter) -> ExecResult {
        match self {
            MatchArm::Type {
                pattern, instruction, ..
            } => {
                let mut interpreter = interpreter.create_layer();
                interpreter.insert(pattern.ident.clone(), variable);
                instruction.exec(&mut interpreter)
            }
            MatchArm::Other(instruction) | MatchArm::Value(_, instruction) => {
                instruction.exec(interpreter)
            }
        }
    }
    pub fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Self, ExecError> {
        Ok(match self {
            Self::Type {
                pattern,
                instruction,
            } => {
                let mut local_variables = local_variables.create_layer();
                let var_type = pattern.var_type.clone().unwrap();
                local_variables.insert(pattern.ident.clone(), LocalVariable::Other(var_type));
                let instruction = instruction.recreate(&mut local_variables)?;
                Self::Type {
                    pattern: pattern.clone(),
                    instruction,
                }
            }
            Self::Value(values, instruction) => {
                let values = recreate_instructions(values, local_variables)?;
                let instruction = instruction.recreate(local_variables)?;
                Self::Value(values, instruction)
            }
            MatchArm::Other(instruction) => Self::Other(instruction.recreate(local_variables)?),
        })
    }
}

impl ReturnType for MatchArm {
    fn return_type(&self) -> Type {
        match self {
            MatchArm::Type { instruction, .. }
            | MatchArm::Value(_, instruction)
            | MatchArm::Other(instruction) => instruction.return_type(),
        }
    }
}
