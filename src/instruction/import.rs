use super::{
    local_variable::LocalVariables, recreate_instructions, CreateInstruction, Exec, Instruction,
    Recreate,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
    Result,
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Import {
    instructions: Box<[Instruction]>,
}

impl CreateInstruction for Import {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let Instruction::Variable(Variable::String(path)) = Instruction::new(
            pair.into_inner().next().unwrap(),
            interpreter,
            local_variables,
        )? else {
            panic!()
        };
        let instructions = interpreter.load(&path, local_variables)?;
        if instructions
            .iter()
            .all(|instruction| matches!(instruction, Instruction::Variable(_)))
        {
            if let Some(Instruction::Variable(variable)) = instructions.last() {
                Ok(Instruction::Variable(variable.clone()))
            } else {
                Ok(Instruction::Variable(Variable::Void))
            }
        } else {
            Ok(Self { instructions }.into())
        }
    }
}

impl Exec for Import {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        interpreter.exec(&self.instructions)
    }
}

impl Recreate for Import {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let instructions = recreate_instructions(&self.instructions, local_variables, interpreter)?;
        Ok(Self { instructions }.into())
    }
}

impl GetReturnType for Import {
    fn get_return_type(&self) -> Type {
        match self.instructions.last() {
            Some(last) => last.get_return_type(),
            None => Type::Void,
        }
    }
}

impl From<Import> for Instruction {
    fn from(value: Import) -> Self {
        Self::Import(value)
    }
}
