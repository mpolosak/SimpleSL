use super::match_arm::MatchArm;
use crate::{
    error::Error,
    instruction::{
        local_variable::LocalVariableMap, CreateInstruction, Exec, Instruction, Recreate,
    },
    interpreter::{Interpreter, VariableMap},
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Match {
    expression: Instruction,
    arms: Box<[MatchArm]>,
}

impl CreateInstruction for Match {
    fn create_instruction(
        pair: Pair<Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let expression = Instruction::new(pair, variables, local_variables)?;
        let var_type = expression.get_return_type();
        let arms = inner
            .map(|pair| MatchArm::new(pair, variables, local_variables))
            .collect::<Result<Box<[MatchArm]>, Error>>()?;
        let result = Self { expression, arms };
        if result.is_covering_type(&var_type) {
            Ok(result.into())
        } else {
            Err(Error::MatchNotCovered)
        }
    }
}

impl Match {
    fn is_covering_type(&self, checked_type: &Type) -> bool {
        match checked_type {
            Type::Multi(types) => types
                .types
                .iter()
                .all(|var_type| self.is_covering_type(var_type)),
            checked_type => self
                .arms
                .iter()
                .any(|arm| arm.is_covering_type(checked_type)),
        }
    }
}

impl Exec for Match {
    fn exec(
        &self,
        interpreter: &mut Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<Variable, Error> {
        let variable = self.expression.exec(interpreter, local_variables)?;
        for arm in self.arms.iter() {
            if arm.covers(&variable, interpreter, local_variables)? {
                return arm.exec(variable, interpreter, local_variables);
            }
        }
        panic!()
    }
}

impl Recreate for Match {
    fn recreate(
        self,
        local_variables: &mut LocalVariableMap,
        args: &VariableMap,
    ) -> Result<Instruction, Error> {
        let expression = self.expression.recreate(local_variables, args)?;
        let arms = self
            .arms
            .iter()
            .map(|arm| arm.clone().recreate(local_variables, args))
            .collect::<Result<Box<[MatchArm]>, Error>>()?;
        Ok(Self { expression, arms }.into())
    }
}

impl GetReturnType for Match {
    fn get_return_type(&self) -> Type {
        todo!()
    }
}

impl From<Match> for Instruction {
    fn from(value: Match) -> Self {
        Self::Match(value.into())
    }
}
