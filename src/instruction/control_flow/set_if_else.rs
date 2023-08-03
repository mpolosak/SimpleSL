use std::rc::Rc;

use crate::instruction::{
    local_variable::{LocalVariable, LocalVariables},
    CreateInstruction, Exec, Instruction, Recreate,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, GetType, Type, Variable},
    Error,
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
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
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let rule = pair.as_rule();
        let mut inner = pair.into_inner();
        let ident: Rc<str> = inner.next().unwrap().as_str().into();
        let pair = inner.next().unwrap();
        let var_type = Type::from(pair);
        let pair = inner.next().unwrap();
        let expression = Instruction::new(pair, interpreter, local_variables)?;
        let pair = inner.next().unwrap();
        let if_match = {
            let mut local_variables = local_variables.create_layer();
            local_variables.insert(ident.clone(), LocalVariable::Other(var_type.clone()));
            Instruction::new(pair, interpreter, &mut local_variables)?
        };
        let else_instruction = if rule == Rule::set_if_else {
            let pair = inner.next().unwrap();
            Instruction::new(pair, interpreter, local_variables)?
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
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable, Error> {
        let expression_result = self.expression.exec(interpreter)?;
        let result_type = expression_result.get_type();
        if result_type.matches(&self.var_type) {
            let mut interpreter = interpreter.create_layer();
            interpreter.insert(self.ident.clone(), expression_result);
            self.if_match.exec(&mut interpreter)
        } else {
            self.else_instruction.exec(interpreter)
        }
    }
}

impl Recreate for SetIfElse {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, Error> {
        let expression = self.expression.recreate(local_variables, interpreter)?;
        let if_match = {
            let mut local_variables = local_variables.create_layer();
            local_variables.insert(
                self.ident.clone(),
                LocalVariable::Other(self.var_type.clone()),
            );
            self.if_match.recreate(&mut local_variables, interpreter)?
        };
        let else_instruction = self
            .else_instruction
            .recreate(local_variables, interpreter)?;
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
