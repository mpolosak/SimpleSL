use crate::{
    instruction::{
        local_variable::LocalVariables,
        traits::{BinOp, CanBeUsed},
        CreateInstruction, Exec, Instruction, Recreate,
    },
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Filter {
    array: Instruction,
    function: Instruction,
}

impl BinOp for Filter {
    const SYMBOL: &'static str = "?";

    fn get_lhs(&self) -> &Instruction {
        &self.array
    }

    fn get_rhs(&self) -> &Instruction {
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

impl CreateInstruction for Filter {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let array = Instruction::new(inner.next().unwrap(), interpreter, local_variables)?;
        let function = Instruction::new(inner.next().unwrap(), interpreter, local_variables)?;
        let array_type = array.get_return_type();
        let function_type = function.get_return_type();
        if Self::can_be_used(&array_type, &function_type) {
            Ok(Self::construct(array, function).into())
        } else {
            Err(Error::CannotDo2(array_type, Self::SYMBOL, function_type))
        }
    }
}

impl Exec for Filter {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let array = self.array.exec(interpreter)?;
        let function = self.function.exec(interpreter)?;
        match (array, function) {
            (Variable::Array(array, _), Variable::Function(function))
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
            (Variable::Array(array, _), Variable::Function(function)) => {
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

impl Recreate for Filter {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let array = self.array.recreate(local_variables, interpreter)?;
        let function = self.function.recreate(local_variables, interpreter)?;
        Ok(Self::construct(array, function).into())
    }
}

impl GetReturnType for Filter {
    fn get_return_type(&self) -> Type {
        self.array.get_return_type()
    }
}

impl From<Filter> for Instruction {
    fn from(value: Filter) -> Self {
        Self::Filter(value.into())
    }
}
