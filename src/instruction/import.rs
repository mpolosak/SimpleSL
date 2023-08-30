use super::{
    local_variable::LocalVariables,
    recreate_instructions,
    traits::{BaseInstruction, MutCreateInstruction},
    Exec, Instruction, Recreate,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type, Variable},
    Result,
};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Import {
    instructions: Box<[Instruction]>,
}

impl MutCreateInstruction for Import {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let Ok(Variable::String(path)) = Variable::try_from(pair.into_inner().next().unwrap())
        else {
            unreachable!()
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

impl ReturnType for Import {
    fn return_type(&self) -> Type {
        self.instructions
            .last()
            .map_or(Type::Void, ReturnType::return_type)
    }
}

impl BaseInstruction for Import {}
