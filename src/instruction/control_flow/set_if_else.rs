use std::rc::Rc;

use crate::instruction::{
    local_variable::{LocalVariable, LocalVariableMap},
    CreateInstruction, Exec, Instruction, Recreate,
};
use crate::{
    error::Error,
    interpreter::{Interpreter, VariableMap},
    parse::Rule,
    variable::{GetReturnType, GetType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct SetIfElse {
    ident: Rc<str>,
    var_type: Type,
    expression: Instruction,
    if_match: Instruction,
    else_instruction: Instruction,
}

impl CreateInstruction for SetIfElse {
    fn create_instruction(
        pair: Pair<Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Instruction, Error> {
        let rule = pair.as_rule();
        let mut inner = pair.into_inner();
        let ident: Rc<str> = inner.next().unwrap().as_str().into();
        let pair = inner.next().unwrap();
        let var_type = Type::from(pair);
        let pair = inner.next().unwrap();
        let expression = Instruction::new(pair, variables, local_variables)?;
        let pair = inner.next().unwrap();
        let mut match_locals = local_variables.clone();
        match_locals.insert(ident.clone(), LocalVariable::Other(var_type.clone()));
        let if_match = Instruction::new(pair, variables, &mut match_locals)?;
        let else_instruction = if rule == Rule::set_if_else {
            let pair = inner.next().unwrap();
            Instruction::new(pair, variables, local_variables)?
        } else {
            Instruction::Variable(Variable::Void)
        };
        Ok(Self {
            ident,
            var_type,
            expression,
            if_match,
            else_instruction,
        }
        .into())
    }
}

impl Exec for SetIfElse {
    fn exec(
        &self,
        interpreter: &mut Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<Variable, Error> {
        let expression_result = self.expression.exec(interpreter, local_variables)?;
        let result_type = expression_result.get_type();
        if result_type.matches(&self.var_type) {
            let mut local_variables = local_variables.clone();
            local_variables.insert(self.ident.clone(), expression_result);
            self.if_match.exec(interpreter, &mut local_variables)
        } else {
            self.else_instruction.exec(interpreter, local_variables)
        }
    }
}

impl Recreate for SetIfElse {
    fn recreate(
        self,
        local_variables: &mut LocalVariableMap,
        args: &VariableMap,
    ) -> Result<Instruction, Error> {
        let expression = self.expression.recreate(local_variables, args)?;
        let mut match_locals = local_variables.clone();
        match_locals.insert(
            self.ident.clone(),
            LocalVariable::Other(self.var_type.clone()),
        );
        let if_match = self.if_match.recreate(&mut match_locals, args)?;
        let else_instruction = self.else_instruction.recreate(local_variables, args)?;
        Ok(Self {
            expression,
            if_match,
            else_instruction,
            ..self
        }
        .into())
    }
}

impl GetReturnType for SetIfElse {
    fn get_return_type(&self) -> Type {
        let true_return_type = self.if_match.get_return_type();
        let false_return_type = self.else_instruction.get_return_type();
        true_return_type.concat(false_return_type)
    }
}

impl From<SetIfElse> for Instruction {
    fn from(value: SetIfElse) -> Self {
        Self::SetIfElse(value.into())
    }
}
