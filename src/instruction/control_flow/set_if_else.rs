use crate::{
    instruction::{
        local_variable::{LocalVariable, LocalVariables},
        Exec, ExecResult, Instruction, InstructionWithStr, Recreate,
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

#[derive(Debug)]
pub struct SetIfElse {
    ident: Arc<str>,
    var_type: Type,
    expression: InstructionWithStr,
    if_match: InstructionWithStr,
    pub else_instruction: InstructionWithStr,
}

impl SetIfElse {
    pub fn create(pair: Pair<Rule>, local_variables: &mut LocalVariables) -> Result<Self, Error> {
        let mut inner = pair.into_inner();
        let ident: Arc<str> = inner.next().unwrap().as_str().into();
        let pair = inner.next().unwrap();
        let var_type = Type::from(pair);
        let pair = inner.next().unwrap();
        let expression = InstructionWithStr::new(pair, local_variables)?;
        let pair = inner.next().unwrap();
        let if_match = {
            local_variables.new_layer();
            local_variables.insert(ident.clone(), LocalVariable::Other(var_type.clone()));
            let iws = InstructionWithStr::new(pair, local_variables)?;
            local_variables.drop_layer();
            iws
        };
        let else_instruction = inner
            .next()
            .map(|pair| InstructionWithStr::new(pair, local_variables))
            .unwrap_or(Ok(Variable::Void.into()))?;
        Ok(Self {
            ident,
            var_type,
            expression,
            if_match,
            else_instruction,
        })
    }

    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        Ok(Self::create(pair, local_variables)?.into())
    }
}

impl Exec for SetIfElse {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let expression_result = self.expression.exec(interpreter)?;
        let result_type = expression_result.as_type();
        if !result_type.matches(&self.var_type) {
            return self.else_instruction.exec(interpreter);
        }
        interpreter.push_layer();
        interpreter.insert(self.ident.clone(), expression_result);
        let result = self.if_match.exec(interpreter);
        interpreter.pop_layer();
        return result;
    }
}

impl Recreate for SetIfElse {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let expression = self.expression.recreate(local_variables)?;
        let if_match = {
            local_variables.new_layer();
            local_variables.insert(
                self.ident.clone(),
                LocalVariable::Other(self.var_type.clone()),
            );
            let ins = self.if_match.recreate(local_variables)?;
            local_variables.drop_layer();
            ins
        };
        let else_instruction = self.else_instruction.recreate(local_variables)?;
        Ok(Self {
            ident: self.ident.clone(),
            var_type: self.var_type.clone(),
            expression,
            if_match,
            else_instruction,
        }
        .into())
    }
}

impl ReturnType for SetIfElse {
    fn return_type(&self) -> Type {
        let true_return_type = self.if_match.return_type();
        let false_return_type = self.else_instruction.return_type();
        true_return_type | false_return_type
    }
}
