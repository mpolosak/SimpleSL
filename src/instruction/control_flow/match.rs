use super::{if_else::return_type, match_arm::MatchArm};
use crate::{
    instruction::{
        local_variable::LocalVariables, Exec, ExecResult, Instruction, InstructionWithStr, Recreate
    },
    interpreter::Interpreter,
    variable::{ReturnType, Type},
    Error, ExecError,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct Match {
    arms: Box<[MatchArm]>,
}

impl Match {
    pub fn create(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
        instructions: &mut Vec<InstructionWithStr>,
    ) -> Result<(), Error> {
        let str = pair.as_str().into();
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        InstructionWithStr::create(pair, local_variables, instructions)?;
        let var_type = return_type(instructions);
        let arms = inner
            .map(|pair| MatchArm::new(pair, local_variables))
            .collect::<Result<Box<[MatchArm]>, Error>>()?;
        let result = Self { arms };
        if !result.is_covering_type(&var_type) {
            return Err(Error::MatchNotCovered);
        }
        let instruction = result.into();
        instructions.push(InstructionWithStr{instruction, str});
        Ok(())
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
        let variable = interpreter.result().unwrap().clone();
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
        let arms = self
            .arms
            .iter()
            .map(|arm| arm.recreate(local_variables))
            .collect::<Result<Box<[MatchArm]>, ExecError>>()?;
        Ok(Self { arms }.into())
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
