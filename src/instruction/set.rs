use std::{sync::Arc};
use super::{
    Exec, ExecResult, Instruction, Recreate, local_variable::LocalVariables,
};
use crate::{ instruction::{function::Function, pattern::{destruct_pattern::DestructPattern, Pattern}, ExecStop}, interpreter::Interpreter, variable::{ReturnType, Type, Typed}, Error, ExecError
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct Set {
    pub pattern: Pattern,
    pub instruction: Instruction,
}

impl Set {
    pub fn create_standalone(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let (set, error) = Self::create(pair, local_variables)?;
        if !set.pattern.is_matched(&set.instruction.return_type()) {
            return Err(error)
        }
        Ok(set.into())
    }

    pub fn create_condition(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Self, Error> {
        Self::create(pair, local_variables).map(|t| t.0)
    }

    fn create(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<(Self, Error), Error> {
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
        let pattern = Pattern::create_instruction(pattern_pair, local_variables, &var_type)?;
        pattern.insert_local_variables(local_variables);
        Ok((Self{ pattern, instruction }, Error::SetPatternNotMatched { ins, var_type, pattern: pattern_str }))
    }

    fn create_function_declaration(pattern_pair: Pair<Rule>, function_pair: Pair<Rule>, local_variables: &mut LocalVariables) -> Result<(Self, Error), Error> { 
        let pattern_str = pattern_pair.as_str().into();
        let mut pattern = Pattern::create_instruction(pattern_pair, local_variables, &Type::Any)?;
        let ident = if let DestructPattern::Ident(ident) = &pattern.destruct_pattern { Some(ident) } else {
            None
        };
        let ins: Arc<str> = function_pair.as_str().into();
        let instruction = Function::create_instruction(function_pair, local_variables, ident.cloned())?;
        let var_type = instruction.return_type();
        pattern.var_type = pattern.var_type.conjoin(&instruction.return_type());
        pattern.insert_local_variables(local_variables);
        Ok((Self{ pattern, instruction }, Error::SetPatternNotMatched { ins, var_type, pattern: pattern_str }))
    }

    pub fn inner_recreate(&self, local_variables: &mut LocalVariables) -> Result<Self, ExecError> {
        let instruction = self.instruction.recreate(local_variables)?;
        self.pattern.insert_local_variables(local_variables);
        Ok(Self {
            pattern: self.pattern.clone(),
            instruction,
        })
    }

    pub fn check(&self, interpreter: &mut Interpreter) -> Result<bool, ExecStop> {
        let result = self.instruction.exec(interpreter)?;
        if !self.pattern.is_matched(&result.as_type()) {
            return Ok(false);
        }
        self.pattern.insert_variables(interpreter, result.clone());
        Ok(true)
    }
}

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
