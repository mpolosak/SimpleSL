use super::{
    local_variable::{LocalVariableMap, LocalVariables},
    recreate_instructions,
    traits::{Exec, Recreate},
    CreateInstruction, Instruction,
};
use crate::{
    error::Error,
    function::{LangFunction, Param, Params},
    interpreter::Interpreter,
    parse::Rule,
    variable::{function_type::FunctionType, GetReturnType, Type, Variable},
};
use pest::iterators::Pair;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Function {
    pub params: Params,
    body: Box<[Instruction]>,
}

impl CreateInstruction for Function {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let params_pair = inner.next().unwrap();
        let params = params_pair.into_inner().map(Param::from).collect();
        let params = Params {
            standard: params,
            catch_rest: None,
        };
        local_variables.push_layer(LocalVariableMap::from(params.clone()));
        let body = inner
            .map(|arg| Instruction::new(arg, interpreter, local_variables))
            .collect::<Result<Box<_>, _>>()?;
        let result = if body
            .iter()
            .all(|instruction| matches!(instruction, Instruction::Variable(_)))
        {
            Ok(Instruction::Variable(Variable::Function(Rc::new(
                LangFunction { params, body },
            ))))
        } else {
            Ok(Self { params, body }.into())
        };
        local_variables.remove_layer();
        result
    }
}

impl Exec for Function {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable, Error> {
        let mut fn_local_variables = LocalVariables::from(self.params.clone());
        let body = recreate_instructions(&self.body, &mut fn_local_variables, interpreter)?;
        Ok(Variable::Function(Rc::new(LangFunction {
            params: self.params.clone(),
            body,
        })))
    }
}

impl Recreate for Function {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, Error> {
        local_variables.push_layer(self.params.clone().into());
        let body = recreate_instructions(&self.body, local_variables, interpreter)?;
        let result = if body
            .iter()
            .all(|instruction| matches!(instruction, Instruction::Variable(_)))
        {
            Ok(Instruction::Variable(Variable::Function(Rc::new(
                LangFunction {
                    params: self.params.clone(),
                    body,
                },
            ))))
        } else {
            Ok(Self {
                params: self.params.clone(),
                body,
            }
            .into())
        };
        local_variables.remove_layer();
        result
    }
}

impl From<Function> for Instruction {
    fn from(value: Function) -> Self {
        Self::Function(value)
    }
}

impl GetReturnType for Function {
    fn get_return_type(&self) -> Type {
        let params_types: Box<[Type]> = self
            .params
            .standard
            .iter()
            .map(|Param { name: _, var_type }| var_type.clone())
            .collect();
        let catch_rest = self.params.catch_rest.is_some();
        let return_type = match self.body.last() {
            Some(instruction) => instruction.get_return_type(),
            None => Type::Any,
        };
        FunctionType {
            return_type,
            params: params_types,
            catch_rest,
        }
        .into()
    }
}
