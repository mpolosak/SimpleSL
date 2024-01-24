use crate::{
    binOp,
    instruction::{
        traits::{CanBeUsed, CreateFromInstructions},
        Exec, Instruction,
    },
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
    Result,
};

binOp!(Filter, "?");

impl CanBeUsed for Filter {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        match (lhs, rhs) {
            (Type::Array(element_type), Type::Function(function_type)) => {
                let params = &function_type.params;
                function_type.return_type == Type::Int
                    && ((params.len() == 1 && element_type.matches(&params[0]))
                        || (params.len() == 2
                            && Type::Int.matches(&params[0])
                            && element_type.matches(&params[1])))
            }
            (Type::EmptyArray, Type::Function(function_type)) => {
                function_type.params.len() == 1 && function_type.return_type == Type::Int
            }
            _ => false,
        }
    }
}

impl CreateFromInstructions for Filter {
    fn create_from_instructions(array: Instruction, function: Instruction) -> Result<Instruction> {
        Ok(Self::construct(array, function).into())
    }
}

impl Exec for Filter {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let array = self.lhs.exec(interpreter)?;
        let function = self.rhs.exec(interpreter)?;
        let (Variable::Array(array), Variable::Function(function)) = (&array, &function) else {
            unreachable!("Tried to do {array} {} {function}", Self::SYMBOL)
        };
        let mut new_array: Vec<Variable> = Vec::new();
        if function.params.len() == 1 {
            for element in array.iter().cloned() {
                if function.exec(interpreter, &[element.clone()])? != Variable::Int(0) {
                    new_array.push(element);
                }
            }
        } else {
            for (index, element) in array.iter().cloned().enumerate() {
                if function.exec(interpreter, &[index.into(), element.clone()])? != Variable::Int(0)
                {
                    new_array.push(element);
                }
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
