use std::rc::Rc;

use pest::iterators::Pair;

use crate::{
    function::{check_args, Params},
    instruction::traits::{BaseInstruction, ExecResult, ExecStop},
    interpreter::Interpreter,
    variable::{FunctionType, ReturnType, Type, Variable},
    Error, Result,
};
use crate::{
    instruction::{
        function::AnonymousFunction,
        local_variable::{LocalVariable, LocalVariables},
        recreate_instructions,
        traits::{Exec, Recreate},
        Instruction,
    },
    parse::Rule,
};

#[derive(Debug)]
pub struct FunctionCall {
    pub function: Instruction,
    pub args: Rc<[Instruction]>,
}

impl FunctionCall {
    pub fn create_instruction(
        function: Instruction,
        args: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &LocalVariables,
    ) -> Result<Instruction> {
        let args = args
            .into_inner()
            .map(|pair| Instruction::new_expression(pair, interpreter, local_variables))
            .collect::<Result<Rc<_>>>()?;
        match &function {
            Instruction::Variable(Variable::Function(function2)) => {
                Self::check_args_with_params("function", &function2.params, &args)?;
                Ok(Self { function, args }.into())
            }
            Instruction::LocalVariable(_, LocalVariable::Function(params, _))
            | Instruction::AnonymousFunction(AnonymousFunction { params, .. }) => {
                Self::check_args_with_params("function", params, &args)?;
                Ok(Self { function, args }.into())
            }
            instruction => {
                Self::check_args_with_type("function", &instruction.return_type(), &args)?;
                Ok(Self { function, args }.into())
            }
        }
    }
    fn check_args_with_params(pair_str: &str, params: &Params, args: &[Instruction]) -> Result<()> {
        check_args(
            pair_str,
            params,
            &args
                .iter()
                .map(Instruction::return_type)
                .collect::<Box<[Type]>>(),
        )
    }
    fn check_args_with_type(pair_str: &str, var_type: &Type, args: &[Instruction]) -> Result<()> {
        let params = args
            .iter()
            .map(Instruction::return_type)
            .collect::<Box<[Type]>>();
        let expected = Type::Function(
            FunctionType {
                params,
                return_type: Type::Any,
            }
            .into(),
        );
        if !var_type.matches(&expected) {
            return Err(Error::WrongType(pair_str.into(), expected));
        }
        Ok(())
    }
}

impl Exec for FunctionCall {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let args = interpreter.exec(&self.args)?;
        let Variable::Function(function) = self.function.exec(interpreter)? else {
            panic!("Tried to call not function")
        };
        function.exec(interpreter, &args).map_err(ExecStop::from)
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

impl ReturnType for FunctionCall {
    fn return_type(&self) -> Type {
        let Type::Function(function_type) = self.function.return_type() else {
            unreachable!();
        };
        function_type.return_type.clone()
    }
}

impl BaseInstruction for FunctionCall {}
