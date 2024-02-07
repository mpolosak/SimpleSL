use crate::{
    function::{Body, Function, Param, Params},
    interpreter::Interpreter,
    parse::Rule,
    variable::{FunctionType, ReturnType, Type},
    Result,
};
use crate::{
    instruction::{
        local_variable::{FunctionInfo, LocalVariableMap, LocalVariables},
        recreate_instructions,
        traits::{Exec, ExecResult, Recreate},
        CreateInstruction, Instruction,
    },
    Error,
};
use pest::iterators::Pair;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct AnonymousFunction {
    pub params: Params,
    body: Rc<[Instruction]>,
    return_type: Type,
}

impl CreateInstruction for AnonymousFunction {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let params_pair = inner.next().unwrap();
        let params = Params(params_pair.into_inner().map(Param::from).collect());
        let return_type = if matches!(inner.peek(), Some(pair)
            if pair.as_rule() == Rule::return_type_decl)
        {
            Type::from(inner.next().unwrap().into_inner().next().unwrap())
        } else {
            Type::Void
        };
        let mut local_variables = local_variables.function_layer(
            LocalVariableMap::from(params.clone()),
            FunctionInfo::new(None, return_type.clone()),
        );
        let body = interpreter.create_instructions(inner, &mut local_variables)?;
        if !Type::Void.matches(&return_type)
            && !body
                .iter()
                .map(ReturnType::return_type)
                .any(|var_type| var_type == Type::Never)
        {
            return Err(Error::MissingReturn {
                function_name: None,
                return_type,
            });
        }
        Ok(Self {
            params,
            body,
            return_type,
        }
        .into())
    }
}

impl Exec for AnonymousFunction {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let mut fn_local_variables = LocalVariables::from(self.params.clone());
        let body = recreate_instructions(&self.body, &mut fn_local_variables, interpreter)?;
        Ok(Function {
            ident: None,
            params: self.params.clone(),
            body: Body::Lang(body),
            return_type: self.return_type.clone(),
        }
        .into())
    }
}

impl Recreate for AnonymousFunction {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let mut local_variables = local_variables.function_layer(
            self.params.clone().into(),
            FunctionInfo::new(None, self.return_type.clone()),
        );
        let body = recreate_instructions(&self.body, &mut local_variables, interpreter)?;
        Ok(Self {
            params: self.params.clone(),
            body,
            return_type: self.return_type.clone(),
        }
        .into())
    }
}

impl From<AnonymousFunction> for Instruction {
    fn from(value: AnonymousFunction) -> Self {
        Self::AnonymousFunction(value)
    }
}

impl ReturnType for AnonymousFunction {
    fn return_type(&self) -> Type {
        let params: Box<[Type]> = self
            .params
            .iter()
            .map(|param| param.var_type.clone())
            .collect();
        let return_type = self.return_type.clone();
        FunctionType {
            params,
            return_type,
        }
        .into()
    }
}
