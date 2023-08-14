use std::rc::Rc;

use crate::instruction::{
    exec_instructions,
    function::AnonymousFunction,
    local_variable::{LocalVariable, LocalVariables},
    recreate_instructions,
    traits::{Exec, Recreate},
    CreateInstruction, Instruction,
};
use crate::{
    function::{check_args, Params},
    interpreter::Interpreter,
    variable::{function_type::FunctionType, GetReturnType, Type, Variable},
    Error, Result,
};

#[derive(Debug)]
pub struct FunctionCall {
    pub function: Instruction,
    pub args: Box<[Instruction]>,
}

impl CreateInstruction for FunctionCall {
    fn create_instruction(
        pair: pest::iterators::Pair<crate::parse::Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let pair_str = pair.as_str();
        let function = Instruction::new(pair, interpreter, local_variables)?;
        let args = inner
            .next()
            .unwrap()
            .into_inner()
            .map(|pair| Instruction::new(pair, interpreter, local_variables))
            .collect::<Result<Box<_>>>()?;
        match &function {
            Instruction::Variable(Variable::Function(function2)) => {
                Self::check_args_with_params(pair_str, &function2.params, &args)?;
                Ok(Self { function, args }.into())
            }
            Instruction::LocalVariable(_, LocalVariable::Function(params, _))
            | Instruction::AnonymousFunction(AnonymousFunction { params, .. }) => {
                Self::check_args_with_params(pair_str, params, &args)?;
                Ok(Self { function, args }.into())
            }
            instruction => {
                Self::check_args_with_type(pair_str, instruction.get_return_type(), &args)?;
                Ok(Self { function, args }.into())
            }
        }
    }
}

impl FunctionCall {
    fn check_args_with_params(pair_str: &str, params: &Params, args: &[Instruction]) -> Result<()> {
        check_args(
            pair_str,
            params,
            &args
                .iter()
                .map(Instruction::get_return_type)
                .collect::<Box<[Rc<Type>]>>(),
        )
    }
    fn check_args_with_type(
        pair_str: &str,
        var_type: Rc<Type>,
        args: &[Instruction],
    ) -> Result<()> {
        let params = args
            .iter()
            .map(Instruction::get_return_type)
            .collect::<Box<[Rc<Type>]>>();
        let expected = Type::Function(
            FunctionType {
                params,
                return_type: Type::Any.into(),
            }
            .into(),
        );
        if var_type.matches(&expected) {
            Ok(())
        } else {
            Err(Error::WrongType(pair_str.into(), expected.into()))
        }
    }
}

impl Exec for FunctionCall {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let args = exec_instructions(&self.args, interpreter)?;
        let Variable::Function(function) = self.function.exec(interpreter)? else {
            panic!("Tried to call not function")
        };
        function.exec(interpreter, &args)
    }
}

impl Recreate for FunctionCall {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let function = self.function.recreate(local_variables, interpreter)?;
        let args = recreate_instructions(&self.args, local_variables, interpreter)?;
        Ok(Self { function, args }.into())
    }
}

impl From<FunctionCall> for Instruction {
    fn from(value: FunctionCall) -> Self {
        Self::FunctionCall(value.into())
    }
}

impl GetReturnType for FunctionCall {
    fn get_return_type(&self) -> Rc<Type> {
        let instruction_return_type = &self.function.get_return_type();
        let Type::Function(function_type) = instruction_return_type.as_ref() else {
            panic!();
        };
        function_type.return_type.clone()
    }
}