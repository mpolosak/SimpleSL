use crate::{
    instruction::{
        local_variable::{LocalVariable, LocalVariables},
        recreate_instructions,
        traits::{ExecResult, ExecStop},
        Exec, Instruction, InstructionWithStr, Recreate,
    },
    interpreter::Interpreter,
    parse::{unexpected, Rule},
    variable::{ReturnType, Type, Typed, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;
use std::sync::Arc;

#[derive(Debug)]
pub enum MatchArm {
    Type {
        ident: Arc<str>,
        var_type: Type,
        instruction: Instruction,
    },
    Value(Arc<[InstructionWithStr]>, Instruction),
    Other(Instruction),
}

impl MatchArm {
    pub fn new(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Self, Error> {
        let match_rule = pair.as_rule();
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        match match_rule {
            Rule::match_type => {
                let ident: Arc<str> = pair.as_str().into();
                let var_type = Type::from(inner.next().unwrap());
                let pair = inner.next().unwrap();
                let mut local_variables = local_variables.create_layer();
                local_variables.insert(ident.clone(), LocalVariable::Other(var_type.clone()));
                let instruction = Instruction::new(pair, interpreter, &mut local_variables)?;
                Ok(Self::Type {
                    ident,
                    var_type,
                    instruction,
                })
            }
            Rule::match_value => {
                let inner_values = pair.into_inner();
                let values = inner_values
                    .map(|pair| InstructionWithStr::new(pair, interpreter, local_variables))
                    .collect::<Result<Arc<[InstructionWithStr]>, Error>>()?;
                let pair = inner.next().unwrap();
                let instruction = Instruction::new(pair, interpreter, local_variables)?;
                Ok(Self::Value(values, instruction))
            }
            Rule::match_other => {
                let instruction = Instruction::new(pair, interpreter, local_variables)?;
                Ok(Self::Other(instruction))
            }
            rule => unexpected(rule),
        }
    }
    pub fn is_covering_type(&self, checked_type: &Type) -> bool {
        match self {
            Self::Value(..) => false,
            Self::Other(_) => true,
            Self::Type { var_type, .. } => checked_type.matches(var_type),
        }
    }
    pub fn covers(
        &self,
        variable: &Variable,
        interpreter: &mut Interpreter,
    ) -> Result<bool, ExecStop> {
        Ok(match self {
            MatchArm::Other(_) => true,
            MatchArm::Type { var_type, .. } => variable.as_type().matches(var_type),
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
                ident, instruction, ..
            } => {
                let mut interpreter = interpreter.create_layer();
                interpreter.insert(ident.clone(), variable);
                instruction.exec(&mut interpreter)
            }
            MatchArm::Other(instruction) | MatchArm::Value(_, instruction) => {
                instruction.exec(interpreter)
            }
        }
    }
    pub fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Self, ExecError> {
        Ok(match self {
            Self::Type {
                ident,
                var_type,
                instruction,
            } => {
                let mut local_variables = local_variables.create_layer();
                local_variables.insert(ident.clone(), LocalVariable::Other(var_type.clone()));
                let instruction = instruction.recreate(&mut local_variables, interpreter)?;
                Self::Type {
                    ident: ident.clone(),
                    var_type: var_type.clone(),
                    instruction,
                }
            }
            Self::Value(values, instruction) => {
                let values = recreate_instructions(values, local_variables, interpreter)?;
                let instruction = instruction.recreate(local_variables, interpreter)?;
                Self::Value(values, instruction)
            }
            MatchArm::Other(instruction) => {
                Self::Other(instruction.recreate(local_variables, interpreter)?)
            }
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
