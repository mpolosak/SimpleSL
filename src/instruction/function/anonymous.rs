use crate::instruction::{
    local_variable::{LocalVariableMap, LocalVariables},
    recreate_instructions,
    traits::{Exec, Recreate},
    CreateInstruction, Instruction,
};
use crate::{
    function::{Body, Function, Param, Params},
    interpreter::Interpreter,
    parse::Rule,
    variable::{function_type::FunctionType, ReturnType, Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct AnonymousFunction {
    pub params: Params,
    body: Box<[Instruction]>,
    return_type: Type,
}

impl CreateInstruction for AnonymousFunction {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner().peekable();
        let params_pair = inner.next().unwrap();
        let params = Params(params_pair.into_inner().map(Param::from).collect());
        let return_type = if matches!(inner.peek(), Some(pair)
            if pair.as_rule() == Rule::return_type_decl)
        {
            Type::from(inner.next().unwrap().into_inner().next().unwrap())
        } else {
            Type::Void
        };
        let mut local_variables =
            local_variables.layer_from_map(LocalVariableMap::from(params.clone()));
        let body = inner
            .map(|instruction| Instruction::new(instruction, interpreter, &mut local_variables))
            .collect::<Result<Box<_>>>()?;

        let returned = body.last().map_or(Type::Void, ReturnType::return_type);
        if !returned.matches(&return_type) {
            return Err(Error::WrongReturn(return_type, returned));
        }

        Ok(Self::create(params, body, return_type))
    }
}

impl AnonymousFunction {
    fn create(params: Params, body: Box<[Instruction]>, return_type: Type) -> Instruction {
        if body
            .iter()
            .all(|instruction| matches!(instruction, Instruction::Variable(_)))
        {
            Instruction::Variable(
                Function {
                    ident: None,
                    params,
                    body: Body::Lang(body),
                    return_type,
                }
                .into(),
            )
        } else {
            Self {
                params,
                body,
                return_type,
            }
            .into()
        }
    }
}

impl Exec for AnonymousFunction {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
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
        let mut local_variables = local_variables.layer_from_map(self.params.clone().into());
        let body = recreate_instructions(&self.body, &mut local_variables, interpreter)?;
        Ok(Self::create(
            self.params.clone(),
            body,
            self.return_type.clone(),
        ))
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
