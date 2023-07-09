use super::{local_variable::LocalVariableMap, Exec, Instruction, Recreate};
use crate::{
    error::Error,
    interpreter::VariableMap,
    parse::Rule,
    variable::Variable,
    variable_type::{GetReturnType, Type},
};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct IfElse {
    condition: Box<Instruction>,
    if_true: Box<Instruction>,
    if_false: Box<Instruction>,
}

impl IfElse {
    pub fn new(
        pair: Pair<Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Self, Error> {
        let rule = pair.as_rule();
        let mut inner = pair.into_inner();
        let condition_pair = inner.next().unwrap();
        let condition = Box::new(Instruction::new(
            condition_pair,
            variables,
            local_variables,
        )?);
        if condition.get_return_type() != Type::Int {
            return Err(Error::WrongType("condition".to_owned(), Type::Int));
        }
        let true_pair = inner.next().unwrap();
        let if_true = Instruction::new(true_pair, variables, local_variables)?.into();
        let if_false = if rule == Rule::if_else {
            let false_pair = inner.next().unwrap();
            Instruction::new(false_pair, variables, local_variables)?
        } else {
            Instruction::Variable(Variable::Null)
        }
        .into();
        Ok(Self {
            condition,
            if_true,
            if_false,
        })
    }
}

impl Exec for IfElse {
    fn exec(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        local_variables: &mut crate::interpreter::VariableMap,
    ) -> Result<crate::variable::Variable, crate::error::Error> {
        if self.condition.exec(interpreter, local_variables)? == Variable::Int(0) {
            self.if_false.exec(interpreter, local_variables)
        } else {
            self.if_true.exec(interpreter, local_variables)
        }
    }
}

impl Recreate for IfElse {
    fn recreate(self, local_variables: &mut LocalVariableMap, args: &VariableMap) -> Instruction {
        let condition = self.condition.recreate(local_variables, args).into();
        let if_true = self.if_true.recreate(local_variables, args).into();
        let if_false = self.if_false.recreate(local_variables, args).into();
        Instruction::IfElse(Self {
            condition,
            if_true,
            if_false,
        })
    }
}

impl GetReturnType for IfElse {
    fn get_return_type(&self) -> crate::variable_type::Type {
        let true_return_type = self.if_true.get_return_type();
        let false_return_type = self.if_false.get_return_type();
        true_return_type.concat(false_return_type)
    }
}

impl From<IfElse> for Instruction {
    fn from(value: IfElse) -> Self {
        Self::IfElse(value)
    }
}
