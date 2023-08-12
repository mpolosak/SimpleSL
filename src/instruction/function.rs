use super::{
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
pub struct FunctionDeclaration {
    pub params: Params,
    body: Box<[Instruction]>,
    return_type: Type,
}

impl CreateInstruction for FunctionDeclaration {
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

impl Exec for FunctionDeclaration {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let mut fn_local_variables = LocalVariables::from(self.params.clone());
        let body = recreate_instructions(&self.body, &mut fn_local_variables, interpreter)?;
        Ok(Function {
            params: self.params.clone(),
            body: Body::Lang(body),
            return_type: self.return_type.clone(),
        }
        .into())
    }
}

impl Recreate for FunctionDeclaration {
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

impl From<FunctionDeclaration> for Instruction {
    fn from(value: FunctionDeclaration) -> Self {
        Self::Function(value)
    }
}

impl GetReturnType for FunctionDeclaration {
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
