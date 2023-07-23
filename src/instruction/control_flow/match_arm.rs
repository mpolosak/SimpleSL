use crate::{
    error::Error,
    instruction::{
        local_variable::{LocalVariable, LocalVariableMap},
        recreate_instructions, Exec, Instruction, Recreate,
    },
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetType, Type, Variable},
};
use pest::iterators::Pair;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum MatchArm {
    Type {
        ident: Rc<str>,
        var_type: Type,
        instruction: Instruction,
    },
    Value(Box<[Instruction]>, Instruction),
    Other(Instruction),
}

impl MatchArm {
    pub fn new(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Self, Error> {
        match pair.as_rule() {
            Rule::match_type => {
                let mut inner = pair.into_inner();
                let ident: Rc<str> = inner.next().unwrap().as_str().into();
                let var_type = Type::from(inner.next().unwrap());
                let pair = inner.next().unwrap();
                let mut local_variables = local_variables.clone();
                local_variables.insert(ident.clone(), LocalVariable::Other(var_type.clone()));
                let instruction = Instruction::new(pair, interpreter, &mut local_variables)?;
                Ok(Self::Type {
                    ident,
                    var_type,
                    instruction,
                })
            }
            Rule::match_value => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let inner_values = pair.into_inner();
                let values = inner_values
                    .map(|pair| Instruction::new(pair, interpreter, local_variables))
                    .collect::<Result<Box<[Instruction]>, Error>>()?;
                let pair = inner.next().unwrap();
                let instruction = Instruction::new(pair, interpreter, local_variables)?;
                Ok(Self::Value(values, instruction))
            }
            Rule::match_other => {
                let mut inner = pair.into_inner();
                let pair = inner.next().unwrap();
                let instruction = Instruction::new(pair, interpreter, local_variables)?;
                Ok(Self::Other(instruction))
            }
            _ => panic!(),
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
    ) -> Result<bool, Error> {
        Ok(match self {
            MatchArm::Other(_) => true,
            MatchArm::Type { var_type, .. } => variable.get_type().matches(var_type),
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
    pub fn exec(
        &self,
        variable: Variable,
        interpreter: &mut Interpreter,
    ) -> Result<Variable, Error> {
        match self {
            MatchArm::Type {
                ident, instruction, ..
            } => {
                interpreter.add_layer();
                interpreter.insert(ident.clone(), variable);
                let result = instruction.exec(interpreter);
                interpreter.remove_layer();
                result
            }
            MatchArm::Other(instruction) | MatchArm::Value(_, instruction) => {
                instruction.exec(interpreter)
            }
        }
    }
    pub fn recreate(
        self,
        local_variables: &mut LocalVariableMap,
        interpreter: &Interpreter,
    ) -> Result<Self, Error> {
        Ok(match self {
            Self::Type {
                ident,
                var_type,
                instruction,
            } => {
                let mut local_variable = local_variables.clone();
                local_variable.insert(ident.clone(), LocalVariable::Other(var_type.clone()));
                let instruction = instruction.recreate(local_variables, interpreter)?;
                Self::Type {
                    ident,
                    var_type,
                    instruction,
                }
            }
            Self::Value(values, instruction) => {
                let values = recreate_instructions(&values, local_variables, interpreter)?;
                let instruction = instruction.recreate(local_variables, interpreter)?;
                Self::Value(values, instruction)
            }
            MatchArm::Other(instruction) => {
                Self::Other(instruction.recreate(local_variables, interpreter)?)
            }
        })
    }
}
