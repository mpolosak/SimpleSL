use crate::{
    instruction::{
        local_variable::LocalVariables,
        traits::{BaseInstruction, ExecResult, ExecStop},
        Exec, Instruction, Recreate,
    },
    interpreter::Interpreter,
    parse::Rule,
    variable::{FunctionType, ReturnType, Type, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Reduce {
    array: Instruction,
    initial_value: Instruction,
    function: Instruction,
}

impl Reduce {
    pub fn create_instruction(
        array: Instruction,
        initial_value: Pair<Rule>,
        function: Instruction,
        local_variables: &LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, Error> {
        let initial_value =
            Instruction::new_expression(initial_value, interpreter, local_variables)?;
        let Some(element_type) = array.return_type().element_type() else {
            return Err(Error::WrongType("array".into(), [Type::Any].into()));
        };
        if let Type::Never = element_type {
            return Ok(initial_value);
        }
        let Some(return_type) = function.return_type().return_type() else {
            return Err(Error::WrongType(
                "function".into(),
                FunctionType {
                    params: [Type::Any, element_type].into(),
                    return_type: Type::Any,
                }
                .into(),
            ));
        };
        let acc_type = initial_value.return_type() | element_type.clone() | return_type.clone();
        let expected_function = Type::from(FunctionType {
            params: [acc_type, element_type].into(),
            return_type,
        });
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
        interpreter: &Interpreter,
    ) -> Result<Instruction, ExecError> {
        let array = self.array.recreate(local_variables, interpreter)?;
        let initial_value = self.initial_value.recreate(local_variables, interpreter)?;
        let function = self.function.recreate(local_variables, interpreter)?;
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
                function.exec(&[acc, current.clone()])
            })
            .map_err(ExecStop::from)
    }
}

impl ReturnType for Reduce {
    fn return_type(&self) -> Type {
        self.function.return_type().return_type().unwrap() | self.initial_value.return_type()
    }
}

impl BaseInstruction for Reduce {}
