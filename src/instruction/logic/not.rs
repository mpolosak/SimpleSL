use crate::instruction::{
    local_variable::LocalVariables, traits::CreateInstruction, Exec, Instruction, Recreate,
};
use crate::{
    interpreter::Interpreter,
    variable::{GetReturnType, Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Not {
    pub instruction: Instruction,
}

impl CreateInstruction for Not {
    fn create_instruction(
        pair: Pair<crate::parse::Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let pair = pair.into_inner().next().unwrap();
        let instruction = Instruction::new(pair, interpreter, local_variables)?;
        let return_type = instruction.get_return_type();
        if return_type == Type::Int || return_type == Type::Array(Type::Int.into()) {
            Ok(Self::create_from_instruction(instruction))
        } else {
            Err(Error::CannotDo("!", instruction.get_return_type()))
        }
    }
}

impl Not {
    fn not(operand: Variable) -> Variable {
        match operand {
            Variable::Int(operand) => (operand == 0).into(),
            Variable::Array(array, _) => array.iter().cloned().map(Self::not).collect(),
            operand => panic!("Tried to negate {operand}"),
        }
    }
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
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let result = self.instruction.exec(interpreter)?;
        Ok(Self::not(result))
    }
}

impl Recreate for Not {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let instruction = self.instruction.recreate(local_variables, interpreter)?;
        Ok(Self::create_from_instruction(instruction))
    }
}

impl GetReturnType for Not {
    fn get_return_type(&self) -> Type {
        self.instruction.get_return_type()
    }
}

impl From<Not> for Instruction {
    fn from(value: Not) -> Self {
        Self::Not(value.into())
    }
}
