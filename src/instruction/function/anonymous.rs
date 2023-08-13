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
    variable::{function_type::FunctionType, GetReturnType, Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct AnonymousFunction {
    pub params: Params,
    body: Box<[Instruction]>,
    return_type: Type,
}

impl CreateInstruction for AnonymousFunction {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner().peekable();
        let params_pair = inner.next().unwrap();
        let params = Params(params_pair.into_inner().map(Param::from).collect());
        let return_type = match inner.peek() {
            Some(pair) if pair.as_rule() == Rule::return_type_decl => {
                Type::from(inner.next().unwrap().into_inner().next().unwrap())
            }
            _ => Type::Void,
        };
        let mut local_variables =
            local_variables.layer_from_map(LocalVariableMap::from(params.clone()));
        let body = inner
            .map(|instruction| Instruction::new(instruction, interpreter, &mut local_variables))
            .collect::<Result<Box<_>>>()?;
        if return_type != Type::Void {
            let returned = match body.last() {
                Some(instruction) => instruction.get_return_type(),
                None => Type::Void,
            };
            if !returned.matches(&return_type) {
                return Err(Error::WrongReturn(return_type, returned));
            }
        }
        if body
            .iter()
            .all(|instruction| matches!(instruction, Instruction::Variable(_)))
        {
            Ok(Instruction::Variable(
                Function {
                    ident: None,
                    params,
                    body: Body::Lang(body),
                    return_type,
                }
                .into(),
            ))
        } else {
            Ok(Self {
                params,
                body,
                return_type,
            }
            .into())
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
        if body
            .iter()
            .all(|instruction| matches!(instruction, Instruction::Variable(_)))
        {
            Ok(Instruction::Variable(
                Function {
                    ident: None,
                    params: self.params.clone(),
                    body: Body::Lang(body),
                    return_type: self.return_type.clone(),
                }
                .into(),
            ))
        } else {
            Ok(Self {
                params: self.params.clone(),
                body,
                return_type: self.return_type.clone(),
            }
            .into())
        }
    }
}

impl From<AnonymousFunction> for Instruction {
    fn from(value: AnonymousFunction) -> Self {
        Self::AnonymousFunction(value)
    }
}

impl GetReturnType for AnonymousFunction {
    fn get_return_type(&self) -> Type {
        let params: Box<[Type]> = self
            .params
            .iter()
            .map(|Param { name: _, var_type }| var_type.clone())
            .collect();
        let return_type = self.return_type.clone();
        FunctionType {
            return_type,
            params,
        }
        .into()
    }
}
