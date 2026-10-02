use crate::{
    instruction::{
        local_variable::LocalVariables, set::Set, Exec, ExecResult, Instruction, InstructionWithStr, Recreate
    }, interpreter::Interpreter, variable::{ReturnType, Type, Variable}, Error, ExecError
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct SetIfElse {
    set: Set,
    if_match: InstructionWithStr,
    pub else_instruction: InstructionWithStr,
}

impl SetIfElse {
    pub fn create(pair: Pair<Rule>, local_variables: &mut LocalVariables) -> Result<Self, Error> {
        let mut inner = pair.into_inner();
        let set_pair = inner.next().unwrap();
        let pair = inner.next().unwrap();
        let (set, if_match) = {
            let mut local_variables = local_variables.create_layer();
            (
                Set::create_condition(set_pair, &mut local_variables)?,
                InstructionWithStr::new(pair, &mut local_variables)?
            )
        };
        let else_instruction = inner
            .next()
            .map(|pair| InstructionWithStr::new(pair, local_variables))
            .unwrap_or(Ok(Variable::Void.into()))?;
        Ok(Self {
            set,
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
        {
            let mut interpreter = interpreter.create_layer();
            if self.set.check(&mut interpreter)? {
                return self.if_match.exec(&mut interpreter)
            }
        }
        self.else_instruction.exec(interpreter)
    }
}

impl Recreate for SetIfElse {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let (set, if_match) = {
            let mut local_variables = local_variables.create_layer();
            (
                self.set.inner_recreate(&mut local_variables)?,
                self.if_match.recreate(&mut local_variables)?
            )
        };
        let else_instruction = self.else_instruction.recreate(local_variables)?;
        Ok(Self {
            set,
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
