pub mod add;
mod concat;
use crate::{
    BinOperator, Error, ExecError, Interpreter,
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, Recreate,
        bin_op::assign::{add::AssignAdd, concat::AssignConcat},
        local_variable::LocalVariables,
    },
    variable::{ReturnType, Type, Variable},
};
use duplicate::duplicate_item;

#[derive(Debug)]
pub struct Assign {
    lhs: Instruction,
    rhs: Instruction,
}

impl Assign {
    pub fn create_instruction(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        let lhs_rt = lhs.return_type();
        let rhs_rt = rhs.return_type();
        if let Some(element_type) = lhs_rt.mut_element_type()
            && rhs_rt.matches(&element_type)
        {
            Ok(Assign { lhs, rhs }.into())
        } else {
            Err(Error::CannotDo2(lhs_rt, BinOperator::Assign, rhs_rt))
        }
    }
}

impl BaseInstruction for Assign {}

impl Exec for Assign {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        let lhs = lhs.into_mut().unwrap();
        let mut lhs = lhs.variable.write().unwrap();
        *lhs = rhs;
        Ok(lhs.clone())
    }
}

#[duplicate_item(T; [Assign]; [AssignAdd]; [AssignConcat];)]
impl Recreate for T {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let lhs = self.lhs.recreate(local_variables);
        let rhs = self.rhs.recreate(local_variables);
        Self { lhs, rhs }.into()
    }
}

impl ReturnType for Assign {
    fn return_type(&self) -> Type {
        self.rhs.return_type()
    }
}

pub fn can_be_used<T, S>(lhs: &Type, rhs: &Type, can_be_used: T, return_type: S) -> bool
where
    T: FnOnce(&Type, &Type) -> bool,
    S: FnOnce(&Type, &Type) -> Type,
{
    let Some(var_type) = lhs.mut_element_type() else {
        return false;
    };
    let can_be_used = can_be_used(&var_type, rhs);
    let return_type = return_type(&var_type, rhs);
    can_be_used && return_type.matches(&var_type)
}

pub fn exec<T: FnOnce(Variable, Variable) -> Variable>(
    lhs: Variable,
    rhs: Variable,
    function: T,
) -> Variable {
    let lhs = lhs.into_mut().unwrap();
    let mut lhs = lhs.variable.write().unwrap();
    *lhs = function(lhs.clone(), rhs);
    lhs.clone()
}

pub fn try_exec<T: FnOnce(Variable, Variable) -> Result<Variable, ExecError>>(
    lhs: Variable,
    rhs: Variable,
    function: T,
) -> Result<Variable, ExecError> {
    let lhs = lhs.into_mut().unwrap();
    let mut lhs = lhs.variable.write().unwrap();
    *lhs = function(lhs.clone(), rhs)?;
    Ok(lhs.clone())
}
