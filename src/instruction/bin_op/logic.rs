pub mod and {
    use crate::{
        BinOperator, ExecError, Interpreter,
        instruction::{
            BinOperation, Exec, ExecResult, Instruction, Recreate, local_variable::LocalVariables,
        },
        variable::Variable,
    };

    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(Variable::Bool(true)), rhs) => rhs,
            (Instruction::Variable(_), _) => Instruction::Variable(false.into()),
            (lhs, rhs) => BinOperation {
                lhs,
                rhs,
                op: BinOperator::And,
            }
            .into(),
        }
    }

    pub fn recreate(
        lhs: Instruction,
        rhs: &Instruction,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, ExecError> {
        match (lhs, rhs) {
            (Instruction::Variable(Variable::Bool(true)), rhs) => rhs.recreate(local_variables),
            (Instruction::Variable(_), _) => Ok(Instruction::Variable(false.into())),
            (lhs, rhs) => Ok(BinOperation {
                lhs,
                rhs: rhs.recreate(local_variables)?,
                op: BinOperator::And,
            }
            .into()),
        }
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
        BinOperator, ExecError, Interpreter,
        instruction::{
            BinOperation, Exec, ExecResult, Instruction, Recreate, local_variable::LocalVariables,
        },
        variable::Variable,
    };

    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(Variable::Bool(true)), _) => Instruction::Variable(true.into()),
            (Instruction::Variable(_), rhs) => rhs,
            (lhs, rhs) => BinOperation {
                lhs,
                rhs,
                op: BinOperator::Or,
            }
            .into(),
        }
    }

    pub fn recreate(
        lhs: Instruction,
        rhs: &Instruction,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, ExecError> {
        match (lhs, rhs) {
            (Instruction::Variable(Variable::Bool(true)), _) => {
                Ok(Instruction::Variable(true.into()))
            }
            (Instruction::Variable(_), rhs) => rhs.recreate(local_variables),
            (lhs, rhs) => Ok(BinOperation {
                lhs,
                rhs: rhs.recreate(local_variables)?,
                op: BinOperator::Or,
            }
            .into()),
        }
    }

    pub fn exec(lhs: Variable, rhs: &Instruction, interpreter: &mut Interpreter) -> ExecResult {
        let lhs = lhs.into_bool().unwrap();
        if lhs {
            return Ok(Variable::Bool(true));
        }
        rhs.exec(interpreter)
    }
}
