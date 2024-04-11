use super::match_arm::MatchArm;
use crate::{
    instruction::{
        local_variable::LocalVariables,
        traits::{ExecResult, MutCreateInstruction},
        Exec, Instruction, InstructionWithStr, Recreate,
    },
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type},
    Error, ExecError,
};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Match {
    expression: InstructionWithStr,
    arms: Box<[MatchArm]>,
}

impl MutCreateInstruction for Match {
    fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<InstructionWithStr, Error> {
        let str = pair.as_str().into();
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let expression = InstructionWithStr::new(pair, local_variables)?;
        let var_type = expression.return_type();
        let arms = inner
            .map(|pair| MatchArm::new(pair, local_variables))
            .collect::<Result<Box<[MatchArm]>, Error>>()?;
        let result = Self { expression, arms };
        if !result.is_covering_type(&var_type) {
            return Err(Error::MatchNotCovered);
        }
        let instruction = result.into();
        Ok(InstructionWithStr { instruction, str })
    }
}

impl Match {
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
