use crate::{
    BinOperator, Interpreter,
    instruction::{
        BaseInstruction, Exec, ExecResult,
        bin_op::{add::Add, assign::concat::AssignConcat},
    },
};
use crate::{
    Error,
    instruction::Instruction,
    variable::{ReturnType, Type},
};

#[derive(Debug)]
pub struct AssignAdd {
    pub lhs: Instruction,
    pub rhs: Instruction,
}

impl BaseInstruction for AssignAdd {}

impl AssignAdd {
    pub fn create_instruction(lhs: Instruction, rhs: Instruction, errors: &mut Vec<Error>) -> Instruction {
        let lhs_rt = lhs.return_type();
        let rhs_rt = rhs.return_type();
        if rhs_rt.is_array() {
            return AssignConcat::create_instruction(lhs, rhs, errors);
        }
        if !Self::can_be_used(&lhs_rt, &rhs_rt) {
            errors.push(Error::CannotDo2(lhs_rt, BinOperator::AssignAdd, rhs_rt));
        }
        Self { lhs, rhs }.into()
    }

    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        let Some(element) = lhs.mut_element_type() else {
            return false;
        };
        Add::can_be_used(element.clone(), rhs.clone())
    }
}

impl Exec for AssignAdd {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        let lhs = lhs.into_mut().unwrap();
        let mut lhs = lhs.variable.write().unwrap();
        *lhs = lhs.clone() + rhs;
        Ok(lhs.clone())
    }
}

impl ReturnType for AssignAdd {
    fn return_type(&self) -> Type {
        self.rhs.return_type()
    }
}
