use super::Instruction;
use crate::{
    error::Error,
    intepreter::{Intepreter, VariableMap},
    parse::Rule,
    variable::Variable,
};
use pest::iterators::Pair;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Line {
    pub result_var: Option<String>,
    pub instruction: Instruction,
}

impl Line {
    pub fn new(
        variables: &VariableMap,
        pair: Pair<Rule>,
        local_variables: &mut HashSet<String>,
    ) -> Result<Line, Error> {
        let pair_vec: Vec<Pair<Rule>> = pair.into_inner().collect();
        if pair_vec.len() == 3 {
            let result_var = pair_vec[0].as_str().to_string();
            let instruction = Instruction::new(variables, pair_vec[1].clone(), local_variables)?;
            local_variables.insert(result_var.clone());
            Ok(Self {
                result_var: Some(result_var),
                instruction,
            })
        } else {
            let instruction = Instruction::new(variables, pair_vec[0].clone(), local_variables)?;
            Ok(Self {
                result_var: None,
                instruction,
            })
        }
    }
    pub fn exec(
        &self,
        intepreter: &mut Intepreter,
        local_variables: &mut VariableMap,
    ) -> Result<Variable, Error> {
        let result = self.instruction.exec(intepreter, local_variables)?;
        if let Some(var) = &self.result_var {
            local_variables.insert(var, result.clone());
        }
        Ok(result)
    }
    pub fn exec_global(&self, intepreter: &mut Intepreter) -> Result<Variable, Error> {
        let result = self.instruction.exec(intepreter, &VariableMap::new())?;
        if let Some(var) = &self.result_var {
            intepreter.variables.insert(var, result);
            Ok(Variable::Null)
        } else {
            Ok(result)
        }
    }
}
