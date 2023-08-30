use super::match_arm::MatchArm;
use crate::{
    instruction::{
        local_variable::LocalVariables,
        traits::{BaseInstruction, MutCreateInstruction},
        Exec, Instruction, Recreate,
    },
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Match {
    expression: Instruction,
    arms: Box<[MatchArm]>,
}

impl MutCreateInstruction for Match {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let expression = Instruction::new(pair, interpreter, local_variables)?;
        let var_type = expression.return_type();
        let arms = inner
            .map(|pair| MatchArm::new(pair, interpreter, local_variables))
            .collect::<Result<Box<[MatchArm]>>>()?;
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
            Type::Multi(types) => types.iter().all(|var_type| self.is_covering_type(var_type)),
            checked_type => self
                .arms
                .iter()
                .any(|arm| arm.is_covering_type(checked_type)),
        }
    }
}

impl Exec for Match {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
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
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let expression = self.expression.recreate(local_variables, interpreter)?;
        let arms = self
            .arms
            .iter()
            .map(|arm| arm.recreate(local_variables, interpreter))
            .collect::<Result<Box<[MatchArm]>>>()?;
        Ok(Self { expression, arms }.into())
    }
}

impl ReturnType for Match {
    fn return_type(&self) -> Type {
        self.arms
            .iter()
            .map(ReturnType::return_type)
            .reduce(Type::concat)
            .unwrap_or_else(|| unreachable!("match statment without arms"))
    }
}

impl BaseInstruction for Match {}
