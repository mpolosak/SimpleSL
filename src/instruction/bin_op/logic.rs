pub mod and {
    use crate::{
        instruction::{
            local_variable::LocalVariables, BinOperation, Exec, ExecResult, Instruction, Recreate,
        },
        variable::{ReturnType, Type, Variable},
        BinOperator, Error, ExecError, Interpreter,
    };

    pub fn create_op(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if lhs_type != Type::Bool || rhs_type != Type::Bool {
            return Err(Error::CannotDo2(lhs_type, BinOperator::And, rhs_type));
        }
        Ok(BinOperation {
            lhs,
            rhs,
            op: BinOperator::And,
        }
        .into())
    }

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
        instruction::{
            local_variable::LocalVariables, BinOperation, Exec, ExecResult, Instruction, Recreate,
        },
        variable::{ReturnType, Type, Variable},
        BinOperator, Error, ExecError, Interpreter,
    };

    pub fn create_op(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if lhs_type != Type::Bool || rhs_type != Type::Bool {
            return Err(Error::CannotDo2(lhs_type, BinOperator::Or, rhs_type));
        }
        Ok(BinOperation {
            lhs,
            rhs,
            op: BinOperator::Or,
        }
        .into())
    }

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
