use crate::{
    instruction::{
        local_variable::LocalVariables, traits::BaseInstruction, Exec, Instruction, Recreate,
    },
    interpreter::Interpreter,
    parse::Rule,
    variable::{FunctionType, ReturnType, Type, Variable},
    Error, Result,
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
    ) -> Result<Instruction> {
        let initial_value =
            Instruction::new_expression(initial_value, interpreter, local_variables)?;
        let element_type = match array.return_type() {
            Type::Array(array) => array.as_ref().clone(),
            Type::EmptyArray => return Ok(initial_value),
            _ => {
                return Err(Error::WrongType(
                    "array".into(),
                    Type::Array(Type::Any.into()),
                ))
            }
        };
        let Type::Function(function_type) = function.return_type() else {
            return Err(Error::WrongType(
                "function".into(),
                Type::Function(
                    FunctionType {
                        params: [Type::Any, element_type].into(),
                        return_type: Type::Any,
                    }
                    .into(),
                ),
            ));
        };
        if function_type.params.len() != 2 {
            return Err(Error::WrongType(
                "function".into(),
                Type::Function(
                    FunctionType {
                        params: [Type::Any, element_type].into(),
                        return_type: Type::Any,
                    }
                    .into(),
                ),
            ));
        };
        let initial_type = initial_value.return_type();
        let acc_type = &function_type.params[0];
        let current_type = &function_type.params[1];
        let return_type = &function_type.return_type;
        let acc_expected = initial_type | return_type.clone();
        if acc_expected.matches(acc_type) && current_type.matches(&element_type) {
            Ok(Self {
                array,
                initial_value,
                function,
            }
            .into())
        } else {
            Err(Error::WrongType(
                "function".into(),
                Type::Function(
                    FunctionType {
                        params: [acc_expected, element_type].into(),
                        return_type: return_type.clone(),
                    }
                    .into(),
                ),
            ))
        }
    }
}

impl Recreate for Reduce {
    fn recreate(
        &self,
        local_variables: &mut crate::instruction::local_variable::LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let array = self.array.recreate(local_variables, interpreter)?;
        let initial_value = self.array.recreate(local_variables, interpreter)?;
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
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let array = self.array.exec(interpreter)?;
        let initial_value = self.initial_value.exec(interpreter)?;
        let function = self.function.exec(interpreter)?;
        match (array, function) {
            (Variable::Array(array, _), Variable::Function(function)) => {
                array.iter().try_fold(initial_value, |acc, current| {
                    function.exec(interpreter, &[acc, current.clone()])
                })
            }
            (array, function) => panic!("Tried to do {array} ${initial_value} {function}"),
        }
    }
}

impl ReturnType for Reduce {
    fn return_type(&self) -> Type {
        let Type::Function(function) = self.function.return_type() else {
            unreachable!()
        };
        function.return_type() | self.initial_value.return_type()
    }
}

impl BaseInstruction for Reduce {}
