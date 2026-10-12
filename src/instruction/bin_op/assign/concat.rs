use crate::{
    BinOperator, Error,
    instruction::{BaseInstruction, Exec, Instruction, bin_op::add::Add},
    variable::{ReturnType, Type},
};

#[derive(Debug)]
pub struct AssignConcat {
    pub lhs: Instruction,
    pub rhs: Instruction,
}

impl BaseInstruction for AssignConcat {}

impl AssignConcat {
    pub fn create_instruction(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        let lhs_rt = lhs.return_type();
        let rhs_rt = rhs.return_type();
        if !Self::can_be_used(&lhs_rt, &rhs_rt) {
            return Err(Error::CannotDo2(lhs_rt, BinOperator::AssignAdd, rhs_rt));
        }
        Ok(Self { lhs, rhs }.into())
    }

    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        let Some(element_type) = lhs.mut_element_type() else {
            return false;
        };
        rhs.matches(&element_type)
    }
}

impl Exec for AssignConcat {
    fn exec(&self, interpreter: &mut crate::Interpreter) -> crate::instruction::ExecResult {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        let lhs = lhs.into_mut().unwrap();
        let mut lhs = lhs.variable.write().unwrap();
        *lhs = Add::calc(lhs.clone(), rhs);
        Ok(lhs.clone())
    }
}

impl ReturnType for AssignConcat {
    fn return_type(&self) -> Type {
        self.lhs.return_type().mut_element_type().unwrap()
    }
}
