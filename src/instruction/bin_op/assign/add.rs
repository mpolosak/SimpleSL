use crate as simplesl;
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
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type =
        var_type!((mut int, int) | (mut float, float) | (mut string, string));
}

#[derive(Debug)]
pub struct AssignAdd {
    pub lhs: Instruction,
    pub rhs: Instruction,
}

impl BaseInstruction for AssignAdd {}

impl AssignAdd {
    pub fn create_instruction(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        let lhs_rt = lhs.return_type();
        let rhs_rt = rhs.return_type();
        if rhs_rt.matches(&var_type!([any])) {
            return AssignConcat::create_instruction(lhs, rhs);
        }
        if !Self::can_be_used(lhs_rt.clone(), rhs_rt.clone()) {
            return Err(Error::CannotDo2(lhs_rt, BinOperator::AssignAdd, rhs_rt));
        }
        Ok(Self { lhs, rhs }.into())
    }

    fn can_be_used(lhs: Type, rhs: Type) -> bool {
        var_type!((lhs, rhs)).matches(&ACCEPTED_TYPE)
    }
}

impl Exec for AssignAdd {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        let lhs = lhs.into_mut().unwrap();
        let mut lhs = lhs.variable.write().unwrap();
        *lhs = Add::calc(lhs.clone(), rhs);
        Ok(lhs.clone())
    }
}

impl ReturnType for AssignAdd {
    fn return_type(&self) -> Type {
        self.rhs.return_type()
    }
}
