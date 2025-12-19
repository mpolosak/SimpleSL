use super::match_arm::MatchArm;
use crate::{
    Error, ExecError,
    instruction::{
        Exec, ExecResult, Instruction, InstructionWithStr, Recreate, local_variable::LocalVariables,
    },
    interpreter::Interpreter,
    variable::{ReturnType, Type},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct Match {
    expression: InstructionWithStr,
    arms: Box<[MatchArm]>,
}

impl Match {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let expression = InstructionWithStr::new(pair, local_variables)?;
        let var_type = expression.return_type();
        let arms = inner
            .map(|pair| MatchArm::new(pair, local_variables, &var_type))
            .collect::<Result<Box<[MatchArm]>, Error>>()?;
        let result = Self { expression, arms };
        if !result.is_covering_type(&var_type) {
            return Err(Error::MatchNotCovered);
        }
        Ok(result.into())
    }

    fn is_covering_type(&self, checked_type: &Type) -> bool {
        if let Type::Multi(types) = checked_type {
            return types.iter().all(|var_type| self.is_covering_type(var_type));
        }
        self.arms
            .iter()
            .any(|arm| arm.is_covering_type(checked_type))
    }
}

impl Exec for Match {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let variable = self.expression.exec(interpreter)?;
        for arm in self.arms.iter() {
            if arm.covers(&variable, interpreter)? {
                return arm.exec(variable, interpreter);
            }
        }
        panic!()
    }
}

impl Recreate for Match {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let expression = self.expression.recreate(local_variables)?;
        let arms = self
            .arms
            .iter()
            .map(|arm| arm.recreate(local_variables))
            .collect::<Result<Box<[MatchArm]>, ExecError>>()?;
        Ok(Self { expression, arms }.into())
    }
}

impl ReturnType for Match {
    fn return_type(&self) -> Type {
        self.arms
            .iter()
            .map(ReturnType::return_type)
            .reduce(Type::concat)
            .expect("match statement without arms")
    }
}
