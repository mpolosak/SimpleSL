use crate::{
    Error,
    instruction::{
        Exec, ExecResult, ExecStop, Instruction, Recreate,
        control_flow::match_pattern::MatchPattern, local_variable::LocalVariables,
        recreate_instructions, set::ConditionSet,
    },
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct MatchArm {
    pub pattern: MatchPattern,
    pub guard: Option<Box<[Instruction]>>,
    pub instruction: Instruction,
}

impl MatchArm {
    pub fn new(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
        exp_type: &Type,
    ) -> Result<Self, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let pattern = MatchPattern::new(pair, local_variables, exp_type)?;
        let pair = inner.next().unwrap();
        let (guard_pair, body_pair) = if pair.as_rule() == Rule::match_guard {
            (Some(pair), inner.next().unwrap())
        } else {
            (None, pair)
        };
        let MatchPattern::Pattern(p) = &pattern else {
            let guard = guard_pair
                .map(|pair| create_guard(pair, local_variables))
                .transpose()?;
            let instruction = Instruction::new(body_pair, local_variables)?;
            return Ok(MatchArm {
                pattern,
                guard,
                instruction,
            });
        };
        let mut local_variables = local_variables.create_layer();
        p.insert_local_variables(&mut local_variables);
        let guard = guard_pair
            .map(|pair| create_guard(pair, &mut local_variables))
            .transpose()?;
        let instruction = Instruction::new(body_pair, &mut local_variables)?;
        Ok(MatchArm {
            pattern,
            guard,
            instruction,
        })
    }

    pub fn is_covering_type(&self, checked_type: &Type) -> bool {
        self.guard.is_none() && self.pattern.is_covering_type(checked_type)
    }
    pub fn covers(
        &self,
        variable: &Variable,
        interpreter: &mut Interpreter,
    ) -> Result<bool, ExecStop> {
        if !self.pattern.covers(variable, interpreter)? {
            return Ok(false);
        }
        let Some(guard) = &self.guard else {
            return Ok(true);
        };
        let mut interpreter = interpreter.create_layer();
        if let MatchPattern::Pattern(pattern) = &self.pattern {
            pattern.insert_variables(&mut interpreter, variable.clone());
        }
        for condition in guard.iter() {
            if matches!(condition.exec(&mut interpreter)?, Variable::Bool(false)) {
                return Ok(false);
            }
        }
        Ok(true)
    }
    pub fn exec(&self, variable: Variable, interpreter: &mut Interpreter) -> ExecResult {
        let MatchPattern::Pattern(pattern) = &self.pattern else {
            return self.instruction.exec(interpreter);
        };
        let mut interpreter = interpreter.create_layer();
        pattern.insert_variables(&mut interpreter, variable);
        self.instruction.exec(&mut interpreter)
    }
    pub fn recreate(&self, local_variables: &mut LocalVariables) -> Self {
        let pattern = self.pattern.recreate(local_variables);
        let MatchPattern::Pattern(p) = &self.pattern else {
            let instruction = self.instruction.recreate(local_variables);
            return Self {
                pattern,
                guard: self
                    .guard
                    .as_ref()
                    .map(|guard| recreate_instructions(guard, local_variables)),
                instruction,
            };
        };
        let mut local_variables = local_variables.create_layer();
        p.insert_local_variables(&mut local_variables);
        let guard = self
            .guard
            .as_ref()
            .map(|guard| recreate_instructions(guard, &mut local_variables));
        let instruction = self.instruction.recreate(&mut local_variables);
        Self {
            pattern,
            guard,
            instruction,
        }
    }
}

fn create_guard(
    pair: pest::iterators::Pair<Rule>,
    local_variables: &mut LocalVariables,
) -> Result<Box<[Instruction]>, Error> {
    let conditions_pair = pair.into_inner().next().unwrap();
    conditions_pair
        .into_inner()
        .map(|pair| {
            if pair.as_rule() == Rule::set_expr {
                return Ok(ConditionSet::create(pair, local_variables)?.into());
            }
            let condition_str = pair.as_str().trim().into();
            let condition = Instruction::new(pair, local_variables)?;
            let return_type = condition.return_type();
            if return_type != Type::Bool {
                return Err(Error::WrongCondition(condition_str, return_type));
            }
            Ok(condition)
        })
        .collect()
}

impl ReturnType for MatchArm {
    fn return_type(&self) -> Type {
        self.instruction.return_type()
    }
}
