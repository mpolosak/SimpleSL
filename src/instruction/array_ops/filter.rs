use crate::{
    instruction::{
        traits::{BaseInstruction, BinOp, CanBeUsed, CreateFromInstructions},
        Exec, Instruction,
    },
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
    Result,
};

#[derive(Debug)]
pub struct Filter {
    array: Instruction,
    function: Instruction,
}

impl BinOp for Filter {
    const SYMBOL: &'static str = "?";

    fn lhs(&self) -> &Instruction {
        &self.array
    }

    fn rhs(&self) -> &Instruction {
        &self.function
    }

    fn construct(array: Instruction, function: Instruction) -> Self {
        Self { array, function }
    }
}

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
        let array = self.array.exec(interpreter)?;
        let function = self.function.exec(interpreter)?;
        match (array, function) {
            (Variable::Array(array), Variable::Function(function))
                if function.params.len() == 1 =>
            {
                let mut new_array: Box<Vec<Variable>> = Box::default();
                for element in array.iter().cloned() {
                    if function.exec(interpreter, &[element.clone()])? != Variable::Int(0) {
                        new_array.push(element);
                    }
                }
                Ok((*new_array).into())
            }
            (Variable::Array(array), Variable::Function(function)) => {
                let mut new_array: Box<Vec<Variable>> = Box::default();
                for (index, element) in array.iter().cloned().enumerate() {
                    if function.exec(interpreter, &[index.into(), element.clone()])?
                        != Variable::Int(0)
                    {
                        new_array.push(element);
                    }
                }
                Ok((*new_array).into())
            }
            (array, function) => panic!("Tried to do {array} {} {function}", Self::SYMBOL),
        }
    }
}

impl ReturnType for Filter {
    fn return_type(&self) -> Type {
        self.array.return_type()
    }
}

impl BaseInstruction for Filter {}
