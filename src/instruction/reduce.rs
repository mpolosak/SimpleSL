pub mod all;
pub mod any;
pub mod bitand;
pub mod bitor;
pub mod product;
pub mod sum;
use crate as simplesl;
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

#[derive(Debug)]
pub struct Reduce {
    array: InstructionWithStr,
    initial_value: InstructionWithStr,
    function: InstructionWithStr,
}

impl Reduce {
    pub fn create_instruction(
        array: InstructionWithStr,
        initial_value: Pair<Rule>,
        function: InstructionWithStr,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error> {
        let initial_value = InstructionWithStr::new_expression(initial_value, local_variables)?;
        let Some(element_type) = array.return_type().index_result() else {
            return Err(Error::CannotReduce(array.str));
        };
        let Some(return_type) = function.return_type().return_type() else {
            return Err(Error::WrongType(
                "function".into(),
                var_type!((any, element_type)->any),
            ));
        };
        let acc_type = initial_value.return_type() | element_type.clone() | return_type.clone();
        let expected_function = var_type!((acc_type, element_type)->return_type);
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
        let initial_value = self.initial_value.recreate(local_variables)?;
        if let Some(Type::Never) = array.return_type().index_result() {
            return Ok(initial_value.instruction);
        }
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
        let initial_value = self.initial_value.exec(interpreter)?;
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
        self.function.return_type().return_type().unwrap() | self.initial_value.return_type()
    }
}
