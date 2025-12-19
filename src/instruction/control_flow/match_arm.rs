use crate::{
    Error, ExecError,
    instruction::{
        Exec, ExecResult, ExecStop, InstructionWithStr,
        control_flow::match_pattern::MatchPattern,
        local_variable::{LocalVariables},
    },
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct MatchArm {
    pattern: MatchPattern,
    instruction: InstructionWithStr,
}

impl MatchArm {
    pub fn new(pair: Pair<Rule>, local_variables: &mut LocalVariables, exp_type: &Type) -> Result<Self, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let pattern = MatchPattern::new(pair, local_variables, exp_type)?;
        let pair = inner.next().unwrap();
        let MatchPattern::Pattern(p) = &pattern else {
            let instruction = InstructionWithStr::new(pair, local_variables)?;
            return Ok(MatchArm {
                pattern,
                instruction,
            });
        };
        let mut local_variables = local_variables.create_layer();
        p.insert_local_variables(&mut local_variables);
        let instruction = InstructionWithStr::new(pair, &mut local_variables)?;
        Ok(MatchArm {
            pattern,
            instruction,
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
        let MatchPattern::Pattern(pattern) = &self.pattern else {
            return self.instruction.exec(interpreter);
        };
        let mut interpreter = interpreter.create_layer();
        interpreter.insert(pattern.ident.clone(), variable);
        self.instruction.exec(&mut interpreter)
    }
    pub fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Self, ExecError> {
        let pattern = self.pattern.recreate(local_variables)?;
        let MatchPattern::Pattern(p) = &self.pattern else {
            let instruction = self.instruction.recreate(local_variables)?;
            return Ok(Self {
                pattern,
                instruction,
            });
        };
        let mut local_variables = local_variables.create_layer();
        p.insert_local_variables(&mut local_variables);
        let instruction = self.instruction.recreate(&mut local_variables)?;
        Ok(Self {
            pattern,
            instruction,
        })
    }
}

impl ReturnType for MatchArm {
    fn return_type(&self) -> Type {
        self.instruction.return_type()
    }
}
