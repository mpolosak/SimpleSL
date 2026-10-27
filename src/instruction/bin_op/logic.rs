use crate::{
    Interpreter,
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, Recreate, local_variable::LocalVariables,
    },
    variable::{ReturnType, Type, Variable},
};

#[derive(Debug)]
pub struct And {
    pub lhs: Instruction,
    pub rhs: Instruction,
}

impl BaseInstruction for And {}

impl Exec for And {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        if let Variable::Bool(false) = self.lhs.exec(interpreter)? {
            return Ok(Variable::Bool(false));
        }
        self.rhs.exec(interpreter)
    }
}

impl Recreate for And {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let lhs = self.lhs.recreate(local_variables);
        let Some(lhs) = lhs.downcast_ref::<Variable>() else {
            let rhs = self.rhs.recreate(local_variables);
            if let Some(Variable::Bool(true)) = rhs.downcast_ref::<Variable>() {
                return lhs;
            }
            return Self { lhs, rhs }.into();
        };
        if let Variable::Bool(true) = lhs {
            return self.rhs.recreate(local_variables);
        }
        Variable::Bool(false).into()
    }
}

impl ReturnType for And {
    fn return_type(&self) -> Type {
        Type::Bool
    }
}

#[derive(Debug)]
pub struct Or {
    pub lhs: Instruction,
    pub rhs: Instruction,
}

impl BaseInstruction for Or {}

impl Exec for Or {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        if let Variable::Bool(true) = self.lhs.exec(interpreter)? {
            return Ok(Variable::Bool(true));
        }
        self.rhs.exec(interpreter)
    }
}

impl Recreate for Or {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let lhs = self.lhs.recreate(local_variables);
        let Some(lhs) = lhs.downcast_ref::<Variable>() else {
            let rhs = self.rhs.recreate(local_variables);
            if let Some(Variable::Bool(false)) = rhs.downcast_ref::<Variable>() {
                return lhs;
            }
            return Self { lhs, rhs }.into();
        };
        if let Variable::Bool(false) = lhs {
            return self.rhs.recreate(local_variables);
        }
        Variable::Bool(true).into()
    }
}

impl ReturnType for Or {
    fn return_type(&self) -> Type {
        Type::Bool
    }
}