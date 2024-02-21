use crate::{
    binOp,
    instruction::{
        traits::{CanBeUsed, ExecResult},
        Exec, Instruction,
    },
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
    Result,
};

binOp!(Filter, "?");

impl CanBeUsed for Filter {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        let Type::Function(function_type) = rhs else {
            return false;
        };
        if lhs == &Type::EmptyArray {
            return function_type.params.len() == 1 && function_type.return_type == Type::Int;
        }
        let Type::Array(element_type) = lhs else {
            return false;
        };

        let params = &function_type.params;
        function_type.return_type == Type::Int
            && ((params.len() == 1 && element_type.matches(&params[0]))
                || (params.len() == 2
                    && Type::Int.matches(&params[0])
                    && element_type.matches(&params[1])))
    }
}

impl Filter {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        Ok(Self { lhs, rhs }.into())
    }
}

impl Exec for Filter {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let array = self.lhs.exec(interpreter)?;
        let function = self.rhs.exec(interpreter)?;
        let (Variable::Array(array), Variable::Function(function)) = (&array, &function) else {
            unreachable!("Tried to do {array} ? {function}")
        };
        let mut new_array: Vec<Variable> = Vec::new();
        if function.params.len() == 1 {
            for element in array.iter().cloned() {
                if function.exec(interpreter, &[element.clone()])? != Variable::Int(0) {
                    new_array.push(element);
                }
            }
            return Ok(new_array.into());
        }
        for (index, element) in array.iter().cloned().enumerate() {
            if function.exec(interpreter, &[index.into(), element.clone()])? != Variable::Int(0) {
                new_array.push(element);
            }
        }
        Ok(new_array.into())
    }
}

impl ReturnType for Filter {
    fn return_type(&self) -> Type {
        self.lhs.return_type()
    }
}
