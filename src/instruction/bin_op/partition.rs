use crate::{
    self as simplesl,
    BinOperator, Error, Interpreter,
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, Recreate,
        bin_op::filter, local_variable::LocalVariables,
    },
    variable::{Array, ReturnType, Type, Typed, Variable},
};
use simplesl_macros::{var, var_type};

#[derive(Debug)]
pub struct Partition {
    lhs: Instruction,
    rhs: Instruction,
}

impl Partition {
    pub fn create_instruction(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if !filter::can_be_used(&lhs_type, &rhs_type) {
            return Err(Error::CannotDo2(lhs_type, BinOperator::Partition, rhs_type));
        }
        Ok(Self { lhs, rhs }.into())
    }

    fn calc(iter: Variable, function: Variable) -> ExecResult {
        let (Variable::Function(iter), Variable::Function(function)) = (&iter, &function) else {
            unreachable!("Tried to do {iter} \\ {function}")
        };
        let mut left = Vec::new();
        let mut right = Vec::new();
        while let Variable::Tuple(tuple) = iter.exec(&[])? {
            if tuple[0] == Variable::Bool(false) {
                break;
            }
            let element = tuple[1].clone();
            if let Variable::Bool(true) = function.exec(std::slice::from_ref(&element))? {
                left.push(element)
            } else {
                right.push(element)
            };
        }

        let element_type = iter.as_type().iter_element().unwrap();
        let element_type2 = element_type.clone();
        let left = Array {
            element_type,
            elements: left.into(),
        };
        let right = Array {
            element_type: element_type2,
            elements: right.into(),
        };

        Ok(var!((left, right)))
    }

    pub fn return_type(var_type: Type) -> Type {
        let element_type = var_type.iter_element().unwrap();
        let element_type2 = element_type.clone();
        var_type!(([element_type], [element_type2]))
    }
}

impl BaseInstruction for Partition {}

impl Exec for Partition {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let iter = self.lhs.exec(interpreter)?;
        let function = self.rhs.exec(interpreter)?;
        Self::calc(iter, function)
    }
}

impl Recreate for Partition {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        Self {
            lhs: self.lhs.recreate(local_variables),
            rhs: self.rhs.recreate(local_variables),
        }
        .into()
    }
}

impl ReturnType for Partition {
    fn return_type(&self) -> Type {
        Self::return_type(self.lhs.return_type())
    }
}

#[cfg(test)]
mod tests {
    use crate::{self as simplesl, Code, Error, Interpreter, variable::Variable};
    use simplesl_macros::var;

    #[test]
    fn partitions_iterator() {
        assert_eq!(
            parse_and_exec(
                "is_even := (value: int) -> bool { return value % 2 == 0 };
                [1, 2, 3, 4]~ \\ is_even"
            ),
            Ok(var!(([2, 4], [1, 3])))
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
