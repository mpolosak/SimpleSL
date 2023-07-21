use super::{
    local_variable::LocalVariableMap,
    recreate_instructions,
    traits::{Exec, Recreate},
    CreateInstruction, Instruction,
};
use crate::{
    error::Error,
    function::{LangFunction, Param, Params},
    interpreter::{Interpreter, VariableMap},
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
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
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let params_pair = inner.next().unwrap();
        let params = params_pair.into_inner().map(Param::from).collect();
        let params = Params {
            standard: params,
            catch_rest: None,
        };
        let mut local_variables = local_variables.clone();
        local_variables.extend(LocalVariableMap::from(params.clone()));
        let body = inner
            .map(|arg| Instruction::new(arg, variables, &mut local_variables))
            .collect::<Result<Box<_>, _>>()?;
        if body
            .iter()
            .all(|instruction| matches!(instruction, Instruction::Variable(_)))
        {
            Ok(Instruction::Variable(Variable::Function(Rc::new(
                LangFunction { params, body },
            ))))
        } else {
            Ok(Self { params, body }.into())
        }
    }
}

impl Exec for Function {
    fn exec(
        &self,
        _interpreter: &mut Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<Variable, Error> {
        let mut fn_local_variables = LocalVariableMap::from(self.params.clone());
        let body = recreate_instructions(&self.body, &mut fn_local_variables, local_variables)?;
        Ok(Variable::Function(Rc::new(LangFunction {
            params: self.params.clone(),
            body,
        })))
    }
}

impl Recreate for Function {
    fn recreate(
        &self,
        local_variables: &mut LocalVariableMap,
        args: &VariableMap,
    ) -> Result<Instruction, Error> {
        let mut local_variables = local_variables.clone();
        local_variables.extend(LocalVariableMap::from(self.params.clone()));
        let body = recreate_instructions(&self.body, &mut local_variables, args)?;
        if body
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
        }
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
        Type::Function {
            return_type: Box::new(return_type),
            params: params_types,
            catch_rest,
        }
    }
}
