use crate::{
    Error,
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, Recreate, local_variable::LocalVariables,
        recreate_instructions, set::ConditionSet,
    },
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct If {
    pub conditions: Box<[Instruction]>,
    pub if_true: Instruction,
    pub else_instruction: Instruction,
}

impl If {
    pub fn create(pair: Pair<Rule>, local_variables: &mut LocalVariables) -> Result<Self, Error> {
        let mut inner = pair.into_inner();
        let (conditions, if_true) = {
            let mut local_variables = local_variables.create_layer();
            let conditions_pair = inner.next().unwrap();
            let conditions = conditions_pair
                .into_inner()
                .map(|pair| {
                    if pair.as_rule() == Rule::set_expr {
                        return Ok(ConditionSet::create(pair, &mut local_variables)?.into());
                    }
                    let condition_str = pair.as_str().into();
                    let condition = Instruction::new(pair, &mut local_variables)?;
                    let return_type = condition.return_type();
                    if return_type != Type::Bool {
                        return Err(Error::WrongCondition(condition_str, return_type));
                    }
                    Ok(condition)
                })
                .collect::<Result<_, _>>()?;

            let true_pair = inner.next().unwrap();
            let if_true = Instruction::new(true_pair, &mut local_variables)?;
            (conditions, if_true)
        };
        let if_false = inner.next().map_or_else(
            || Ok(Variable::Void.into()),
            |pair| Instruction::new(pair, local_variables),
        )?;
        Ok(Self {
            conditions,
            if_true,
            else_instruction: if_false,
        })
    }
}

impl BaseInstruction for If {}

impl Exec for If {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let mut interpreter2 = interpreter.create_layer();
        for condition in self.conditions.iter() {
            if let Variable::Bool(false) = condition.exec(&mut interpreter2)? {
                return self.else_instruction.exec(interpreter);
            }
        }
        self.if_true.exec(&mut interpreter2)
    }
}

impl Recreate for If {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let (conditions, if_true) = {
            let mut local_variables = local_variables.create_layer();
            let conditions = recreate_instructions(&self.conditions, &mut local_variables);
            let if_true = self.if_true.recreate(&mut local_variables);
            (conditions, if_true)
        };
        let if_false = self.else_instruction.recreate(local_variables);
        Self {
            conditions,
            if_true,
            else_instruction: if_false,
        }
        .into()
    }
}

impl ReturnType for If {
    fn return_type(&self) -> Type {
        let true_return_type = self.if_true.return_type();
        let false_return_type = self.else_instruction.return_type();
        true_return_type | false_return_type
    }
}
