use crate::{
    Error, Interpreter,
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, InstructionWithStr, Recreate,
        local_variable::LocalVariables,
    },
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type},
};

#[derive(Debug)]
pub struct Deref(Instruction);

impl Deref {
    pub fn create_instruction(instruction: InstructionWithStr) -> Result<Instruction, Error> {
        let return_type = instruction.return_type();
        if return_type.mut_element_type().is_none() {
            return Err(Error::IncorectUnaryOperatorOperand {
                ins: instruction.str,
                op: UnaryOperator::Indirection,
                expected: Type::Mut(Type::Any.into()),
                given: return_type,
            });
        }
        Ok(Self(instruction.instruction).into())
    }
}

impl BaseInstruction for Deref {}

impl Exec for Deref {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let value = self.0.exec(interpreter)?;
        Ok(value.into_mut().unwrap().variable.read().unwrap().clone())
    }
}

impl Recreate for Deref {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let ins = self.0.recreate(local_variables);
        Self(ins).into()
    }
}

impl ReturnType for Deref {
    fn return_type(&self) -> crate::variable::Type {
        self.0.return_type().mut_element_type().unwrap()
    }
}
