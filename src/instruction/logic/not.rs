use crate::instruction::{
    local_variable::LocalVariableMap, traits::CreateInstruction, Exec, Instruction, Recreate,
};
use crate::{
    error::Error,
    interpreter::VariableMap,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct Not {
    pub instruction: Instruction,
}

impl CreateInstruction for Not {
    fn create_instruction(
        pair: Pair<crate::parse::Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Instruction, Error> {
        let pair = pair.into_inner().next().unwrap();
        let instruction = Instruction::new(pair, variables, local_variables)?;
        if instruction.get_return_type() == Type::Int {
            Ok(Self::create_from_instruction(instruction))
        } else {
            Err(Error::OperandMustBeInt("!"))
        }
    }
}

impl Not {
    fn create_from_instruction(instruction: Instruction) -> Instruction {
        match instruction {
            Instruction::Variable(Variable::Int(value)) => {
                Instruction::Variable((value == 0).into())
            }
            instruction => Self { instruction }.into(),
        }
    }
}

impl Exec for Not {
    fn exec(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<Variable, Error> {
        let Variable::Int(result) = self.instruction.exec(interpreter, local_variables)? else {
            panic!()
        };
        Ok((result == 0).into())
    }
}

impl Recreate for Not {
    fn recreate(
        self,
        local_variables: &mut LocalVariableMap,
        args: &VariableMap,
    ) -> Result<Instruction, Error> {
        let instruction = self.instruction.recreate(local_variables, args)?;
        Ok(Self::create_from_instruction(instruction))
    }
}

impl From<Not> for Instruction {
    fn from(value: Not) -> Self {
        Self::Not(value.into())
    }
}
