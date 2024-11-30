use crate::{
    instruction::{
        local_variable::{LocalVariable, LocalVariables}, recreate_instructions, Exec, ExecResult, Instruction, InstructionWithStr, Recreate
    },
    Error, ExecError,
};
use crate::{
    interpreter::Interpreter,
    variable::{ReturnType, Type, Typed, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

use super::if_else::return_type;

#[derive(Debug)]
pub struct SetIfElse {
    pub ident: Arc<str>,
    pub var_type: Type,
    pub if_match: Arc<[InstructionWithStr]>,
    pub else_instructions: Arc<[InstructionWithStr]>,
}

impl SetIfElse {
    pub fn create(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
        instructions: &mut Vec<InstructionWithStr>,
    ) -> Result<(), Error> {
        let mut inner = pair.into_inner();
        let ident: Arc<str> = inner.next().unwrap().as_str().into();
        let pair = inner.next().unwrap();
        let var_type = Type::from(pair);
        let pair = inner.next().unwrap();
        InstructionWithStr::create(pair, local_variables, instructions)?;
        
        let if_match_pair = inner.next().unwrap();
        let mut if_match = Vec::<InstructionWithStr>::new();
        local_variables.new_layer();
        local_variables.insert(ident.clone(), LocalVariable::Other(var_type.clone()));
        InstructionWithStr::create(if_match_pair, local_variables, &mut if_match)?;
        local_variables.drop_layer();
        let if_match = if_match.into();

        let mut else_instructions = Vec::<InstructionWithStr>::new();
        if let Some(pair) = inner.next() {
            InstructionWithStr::create(pair, local_variables, &mut else_instructions)?;
        } else {
            else_instructions.push(Variable::Void.into());
        }
        let else_instructions = else_instructions.into();

        let instruction = Self{ ident, var_type, if_match, else_instructions }.into();
        let instruction = InstructionWithStr{ instruction, str: "if else".into() };
        instructions.push(instruction);
        Ok(())
    }
}

impl Exec for SetIfElse {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let expression_result = interpreter.result().unwrap().clone();
        let result_type = expression_result.as_type();
        if result_type.matches(&self.var_type) {
            interpreter.push_layer();
            interpreter.insert(self.ident.clone(), expression_result);
            interpreter.exec_all(&self.if_match)?;
            interpreter.pop_layer(); 
        } else {
            interpreter.exec_all(&self.else_instructions)?;
        }
        
        Ok(interpreter.result().unwrap().clone())
    }
}

impl Recreate for SetIfElse {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        local_variables.new_layer();
        local_variables.insert(
            self.ident.clone(),
            LocalVariable::Other(self.var_type.clone()),
        );
        let if_match =  recreate_instructions(&self.if_match, local_variables)?;
        local_variables.drop_layer();

        let else_instructions = recreate_instructions(&self.else_instructions, local_variables)?;
        Ok(Self {
            ident: self.ident.clone(),
            var_type: self.var_type.clone(),
            if_match,
            else_instructions,
        }
        .into())
    }
}

impl ReturnType for SetIfElse {
    fn return_type(&self) -> Type {
        let true_return_type = return_type(&self.if_match);
        let false_return_type = return_type(&self.else_instructions);
        true_return_type | false_return_type
    }
}
