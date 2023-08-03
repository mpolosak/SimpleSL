use crate::instruction::{
    local_variable::LocalVariables, traits::CreateInstruction, Exec, Instruction, Recreate,
};
use crate::{
    interpreter::Interpreter,
    variable::{GetReturnType, Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct BinNot {
    pub instruction: Instruction,
}

impl CreateInstruction for BinNot {
    fn create_instruction(
        pair: Pair<crate::parse::Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let pair = pair.into_inner().next().unwrap();
        let instruction = Instruction::new(pair, interpreter, local_variables)?;
        if instruction.get_return_type() == Type::Int {
            Ok(Self::create_from_instruction(instruction))
        } else {
            Err(Error::OperandMustBeInt("~"))
        }
    }
}

impl BinNot {
    fn create_from_instruction(instruction: Instruction) -> Instruction {
        match instruction {
            Instruction::Variable(Variable::Int(value)) => Instruction::Variable((!value).into()),
            instruction => Self { instruction }.into(),
        }
    }
}

impl Exec for BinNot {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let Variable::Int(result) = self.instruction.exec(interpreter)? else {
            panic!()
        };
        Ok((!result).into())
    }
}

impl Recreate for BinNot {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let instruction = self.instruction.recreate(local_variables, interpreter)?;
        Ok(Self::create_from_instruction(instruction))
    }
}

impl From<BinNot> for Instruction {
    fn from(value: BinNot) -> Self {
        Self::BinNot(value.into())
    }
}
