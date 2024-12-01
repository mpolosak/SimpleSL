pub mod all;
pub mod any;
pub mod bitand;
pub mod bitor;
pub mod product;
pub mod sum;
use super::control_flow::if_else::return_type;
use super::recreate_instructions;
use crate as simplesl;
use crate::variable::Typed;
use crate::{
    instruction::{
        local_variable::LocalVariables, Exec, ExecResult, ExecStop, Instruction,
        InstructionWithStr, Recreate,
    },
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug)]
pub struct Reduce {
    array: InstructionWithStr,
    initial_value: Arc<[InstructionWithStr]>,
    function: InstructionWithStr,
}

impl Reduce {
    pub fn create_instruction(
        array: InstructionWithStr,
        initial_value: Pair<Rule>,
        function: InstructionWithStr,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut initial_value_vec = Vec::new();
        InstructionWithStr::create(initial_value, local_variables, &mut initial_value_vec)?;
        let initial_value: Arc<[InstructionWithStr]> = initial_value_vec.into();
        let Some(element_type) = array.return_type().index_result() else {
            return Err(Error::CannotReduce(array.str));
        };
        let Some(fn_return_type) = function.return_type().return_type() else {
            return Err(Error::WrongType(
                "function".into(),
                var_type!((any, element_type)->any),
            ));
        };
        let initial_value_type = local_variables.result.as_ref().unwrap().as_type();
        let acc_type = initial_value_type | element_type.clone() | fn_return_type.clone();
        let expected_function = var_type!((acc_type, element_type)->fn_return_type);
        if !function.return_type().matches(&expected_function) {
            return Err(Error::WrongType("function".into(), expected_function));
        }
        Ok(Self {
            array,
            initial_value,
            function,
        }
        .into())
    }
}

impl Recreate for Reduce {
    fn recreate(
        &self,
        local_variables: &mut crate::instruction::local_variable::LocalVariables,
    ) -> Result<Instruction, ExecError> {
        let array = self.array.recreate(local_variables)?;
        let initial_value = recreate_instructions(&self.initial_value, local_variables)?;
        let function = self.function.recreate(local_variables)?;
        Ok(Self {
            array,
            initial_value,
            function,
        }
        .into())
    }
}

impl Exec for Reduce {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let array = self.array.exec(interpreter)?;
        interpreter.exec_all(&self.initial_value)?;
        let initial_value = interpreter.result().unwrap().clone();
        let function = self.function.exec(interpreter)?;
        let (Variable::Array(array), Variable::Function(function)) = (&array, &function) else {
            unreachable!("Tried to do {array} ${initial_value} {function}")
        };
        array
            .iter()
            .try_fold(initial_value, |acc, current| {
                function.exec_with_args(&[acc, current.clone()])
            })
            .map_err(ExecStop::from)
    }
}

impl ReturnType for Reduce {
    fn return_type(&self) -> Type {
        self.function.return_type().return_type().unwrap() | return_type(&self.initial_value)
    }
}
