use super::{
    local_variable::LocalVariableMap,
    recreate_instructions,
    traits::{Exec, Recreate},
    Instruction,
};
use crate::{
    error::Error,
    function::{LangFunction, Param, Params},
    interpreter::{Interpreter, VariableMap},
    parse::Rule,
    variable::Variable,
    variable_type::{GetReturnType, Type},
};
use pest::iterators::Pair;
use std::rc::Rc;

#[derive(Clone)]
pub struct Function {
    pub params: Params,
    body: Vec<Instruction>,
}

impl Function {
    pub fn new(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariableMap,
        variables: &VariableMap,
    ) -> Result<Self, Error> {
        let mut inner = pair.into_inner();
        let params_pair = inner.next().unwrap();
        let params: Vec<Param> = params_pair.into_inner().map(Param::from).collect();
        let params = Params {
            standard: params,
            catch_rest: None,
        };
        let mut local_variables = local_variables.clone();
        local_variables.extend(LocalVariableMap::from(params.clone()));
        let body = inner
            .map(|arg| Instruction::new(variables, arg, &mut local_variables))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { params, body })
    }
}

impl Exec for Function {
    fn exec(
        &self,
        _interpreter: &mut Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<Variable, Error> {
        let mut fn_local_variables = LocalVariableMap::from(self.params.clone());
        let body =
            recreate_instructions(self.body.clone(), &mut fn_local_variables, local_variables);
        Ok(Variable::Function(Rc::new(LangFunction {
            params: self.params.clone(),
            body,
        })))
    }
}

impl Recreate for Function {
    fn recreate(self, local_variables: &mut LocalVariableMap, args: &VariableMap) -> Instruction {
        let mut local_variables = local_variables.clone();
        local_variables.extend(LocalVariableMap::from(self.params.clone()));
        let body = recreate_instructions(self.body, &mut local_variables, args);
        Self { body, ..self }.into()
    }
}

impl From<Function> for Instruction {
    fn from(value: Function) -> Self {
        Self::Function(value)
    }
}

impl GetReturnType for Function {
    fn get_return_type(&self) -> Type {
        let params_types: Vec<Type> = self
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
