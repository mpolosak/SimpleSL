pub mod and {
    use crate::{
        BinOperator, Interpreter,
        instruction::{
            BinOperation, Exec, ExecResult, Instruction, Recreate, local_variable::LocalVariables,
        },
        variable::Variable,
    };

    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        let Some(variable) = lhs.downcast_ref::<Variable>() else {
            return BinOperation {
                lhs,
                rhs,
                op: BinOperator::Or,
            }
            .into();
        };
        if let Variable::Bool(true) = variable {
            return rhs
        }
        Variable::Bool(false).into()
    }

    pub fn recreate(
        lhs: Instruction,
        rhs: &Instruction,
        local_variables: &mut LocalVariables,
    ) -> Instruction {
        let Some(variable) = lhs.downcast_ref::<Variable>() else {
            return BinOperation {
                lhs,
                rhs: rhs.recreate(local_variables),
                op: BinOperator::Or,
            }
            .into();
        };
        if let Variable::Bool(true) = variable {
            return rhs.recreate(local_variables)
        }
        Variable::Bool(false).into()
    }

    pub fn exec(lhs: Variable, rhs: &Instruction, interpreter: &mut Interpreter) -> ExecResult {
        let lhs = lhs.into_bool().unwrap();
        if !lhs {
            return Ok(Variable::Bool(false));
        }
        rhs.exec(interpreter)
    }
}

pub mod or {
    use crate::{
        instruction::{
            local_variable::LocalVariables, BinOperation, Exec, ExecResult, Instruction, Recreate
        }, variable::Variable, BinOperator, Interpreter
    };

    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        let Some(variable) = lhs.downcast_ref::<Variable>() else {
            return BinOperation {
                lhs,
                rhs,
                op: BinOperator::Or,
            }
            .into();
        };
        if let Variable::Bool(true) = variable {
            return Variable::Bool(true).into()
        }
        rhs
    }

    pub fn recreate(
        lhs: Instruction,
        rhs: &Instruction,
        local_variables: &mut LocalVariables,
    ) -> Instruction {
        let Some(variable) = lhs.downcast_ref::<Variable>() else {
            return BinOperation {
                lhs,
                rhs: rhs.recreate(local_variables),
                op: BinOperator::Or,
            }
            .into();
        };
        if let Variable::Bool(true) = variable {
            return Variable::Bool(true).into()
        }
        rhs.recreate(local_variables)
    }

    pub fn exec(lhs: Variable, rhs: &Instruction, interpreter: &mut Interpreter) -> ExecResult {
        let lhs = lhs.into_bool().unwrap();
        if lhs {
            return Ok(Variable::Bool(true));
        }
        rhs.exec(interpreter)
    }
}
