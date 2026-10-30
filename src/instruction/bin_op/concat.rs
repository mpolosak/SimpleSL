use crate as simplesl;
use crate::{
    Interpreter,
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, Recreate, local_variable::LocalVariables,
    },
    variable::{Array, ReturnType, Type, Variable},
};
use simplesl_macros::var_type;

#[derive(Debug)]
pub struct Concat {
    pub lhs: Instruction,
    pub rhs: Instruction,
}

impl Concat {
    pub fn calc(lhs: Variable, rhs: Variable) -> Variable {
        let lhs = lhs.into_array().unwrap();
        let rhs = rhs.into_array().unwrap();
        Array::concat(lhs, rhs).into()
    }

    pub fn return_type(lhs: Type, rhs: Type) -> Type {
        let lhs_element = lhs.element_type().unwrap_or(Type::Never);
        let rhs_element = rhs.element_type().unwrap_or(Type::Never);
        var_type!([lhs_element | rhs_element])
    }
}

impl BaseInstruction for Concat {}

impl Exec for Concat {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok(Self::calc(lhs, rhs))
    }
}

impl Recreate for Concat {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let lhs = self.lhs.recreate(local_variables);
        let rhs = self.rhs.recreate(local_variables);
        let (Some(lhs), Some(rhs)) = (
            lhs.downcast_ref::<Variable>(),
            rhs.downcast_ref::<Variable>(),
        ) else {
            return Self { lhs, rhs }.into();
        };
        Self::calc(lhs.clone(), rhs.clone()).into()
    }
}

impl ReturnType for Concat {
    fn return_type(&self) -> Type {
        let lhs = self.lhs.return_type();
        let rhs = self.rhs.return_type();
        Self::return_type(lhs, rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate as simplesl;
    use crate::instruction::bin_op::concat::Concat;
    use simplesl_macros::var_type;

    #[test]
    fn return_type() {
        assert_eq!(
            Concat::return_type(var_type!([int]), var_type!([int])),
            var_type!([int])
        );
        assert_eq!(
            Concat::return_type(var_type!([int]), var_type!([float])),
            var_type!([float | int])
        );
        assert_eq!(
            Concat::return_type(var_type!([int]), var_type!([any])),
            var_type!([any])
        );
        assert_eq!(
            Concat::return_type(var_type!([]), var_type!([string])),
            var_type!([string])
        );
        assert_eq!(
            Concat::return_type(var_type!([]), var_type!([])),
            var_type!([])
        );
    }
}
