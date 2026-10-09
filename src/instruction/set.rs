use super::{Exec, ExecResult, Instruction, Recreate, local_variable::LocalVariables};
use crate::{
    Error, ExecError,
    instruction::{
        BaseInstruction,
        function::Function,
        pattern::{Pattern, destruct_pattern::DestructPattern},
    },
    interpreter::Interpreter,
    variable::{ReturnType, Type, Typed},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug)]
pub struct Set {
    pub pattern: Pattern,
    pub instruction: Instruction,
}

impl Set {
    pub fn create(pair: Pair<Rule>, local_variables: &mut LocalVariables) -> Result<Self, Error> {
        let (set, error) = Self::new(pair, local_variables)?;
        if !set.pattern.is_matched(&set.instruction.return_type()) {
            return Err(error);
        }
        Ok(set)
    }

    fn new(pair: Pair<Rule>, local_variables: &mut LocalVariables) -> Result<(Self, Error), Error> {
        let mut inner = pair.into_inner();
        let pattern_pair = inner.next().unwrap();
        let pair = inner.next().unwrap();
        let ins = pair.as_str().into();
        if pair.as_rule() == Rule::function {
            return Self::create_function_declaration(pattern_pair, pair, local_variables);
        }
        let instruction = Instruction::new(pair, local_variables)?;
        let var_type = instruction.return_type();
        let pattern_str = pattern_pair.as_str().into();
        let pattern = Pattern::create_instruction(pattern_pair, &var_type)?;
        pattern.insert_local_variables(local_variables);
        Ok((
            Self {
                pattern,
                instruction,
            },
            Error::SetPatternNotMatched {
                ins,
                var_type,
                pattern: pattern_str,
            },
        ))
    }

    fn create_function_declaration(
        pattern_pair: Pair<Rule>,
        function_pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<(Self, Error), Error> {
        let pattern_str = pattern_pair.as_str().into();
        let mut pattern = Pattern::create_instruction(pattern_pair, &Type::Any)?;
        let ident = if let DestructPattern::Ident(ident) = &pattern.destruct_pattern {
            Some(ident)
        } else {
            None
        };
        let ins: Arc<str> = function_pair.as_str().into();
        let instruction =
            Function::create_instruction(function_pair, local_variables, ident.cloned())?;
        let var_type = instruction.return_type();
        pattern.var_type = pattern.var_type.conjoin(&instruction.return_type());
        pattern.insert_local_variables(local_variables);
        Ok((
            Self {
                pattern,
                instruction,
            },
            Error::SetPatternNotMatched {
                ins,
                var_type,
                pattern: pattern_str,
            },
        ))
    }

    pub fn inner_recreate(&self, local_variables: &mut LocalVariables) -> Result<Self, ExecError> {
        let instruction = self.instruction.recreate(local_variables)?;
        self.pattern.insert_local_variables(local_variables);
        Ok(Self {
            pattern: self.pattern.clone(),
            instruction,
        })
    }

    pub fn new_ident(ident: Arc<str>, instruction: Instruction) -> Set {
        Set {
            pattern: Pattern::new_ident_pattern(ident, instruction.return_type()),
            instruction,
        }
    }
}

impl BaseInstruction for Set {}

impl Exec for Set {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let result = self.instruction.exec(interpreter)?;
        self.pattern.insert_variables(interpreter, result.clone());
        Ok(result)
    }
}

impl Recreate for Set {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        Ok(self.inner_recreate(local_variables)?.into())
    }
}

impl ReturnType for Set {
    fn return_type(&self) -> Type {
        self.instruction.return_type()
    }
}

#[derive(Debug)]
pub struct ConditionSet(Set);

impl ConditionSet {
    pub fn create(pair: Pair<Rule>, local_variables: &mut LocalVariables) -> Result<Self, Error> {
        Ok(ConditionSet(Set::new(pair, local_variables).map(|t| t.0)?))
    }
}

impl BaseInstruction for ConditionSet {}

impl Exec for ConditionSet {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let result = self.0.instruction.exec(interpreter)?;
        if !self.0.pattern.is_matched(&result.as_type()) {
            return Ok(false.into());
        }
        self.0.pattern.insert_variables(interpreter, result.clone());
        Ok(true.into())
    }
}

impl Recreate for ConditionSet {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        Ok(Self(self.0.inner_recreate(local_variables)?).into())
    }
}

impl ReturnType for ConditionSet {
    fn return_type(&self) -> Type {
        unreachable!()
    }
}
