use crate::{
    function::{Body, Function, Param, Params},
    instruction::{
        local_variable::{LocalVariable, LocalVariableMap, LocalVariables},
        recreate_instructions,
        set::Set,
        traits::{BaseInstruction, MutCreateInstruction},
        Exec, Instruction, Recreate,
    },
    interpreter::Interpreter,
    parse::Rule,
    variable::{function_type::FunctionType, GetReturnType, Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;
use std::rc::Rc;

#[derive(Debug)]
pub struct FunctionDeclaration {
    ident: Rc<str>,
    pub params: Params,
    body: Box<[Instruction]>,
    return_type: Type,
}

impl MutCreateInstruction for FunctionDeclaration {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let ident: Rc<str> = inner.next().unwrap().as_str().into();
        let mut inner = inner.next().unwrap().into_inner();
        let params_pair = inner.next().unwrap();
        let params = Params(params_pair.into_inner().map(Param::from).collect());
        let return_type = match inner.peek() {
            Some(pair) if pair.as_rule() == Rule::return_type_decl => {
                Type::from(inner.next().unwrap().into_inner().next().unwrap())
            }
            _ => Type::Void,
        };
        let body = {
            let mut local_variables = local_variables.create_layer();
            local_variables.insert(
                ident.clone(),
                LocalVariable::Function(params.clone(), return_type.clone()),
            );
            let mut local_variables =
                local_variables.layer_from_map(LocalVariableMap::from(params.clone()));
            inner
                .map(|instruction| Instruction::new(instruction, interpreter, &mut local_variables))
                .collect::<Result<Box<_>>>()
        }?;
        let returned = body
            .last()
            .map_or(Type::Void, GetReturnType::get_return_type);
        if !returned.matches(&return_type) {
            return Err(Error::WrongReturn(return_type, returned));
        }
        if body
            .iter()
            .all(|instruction| matches!(instruction, Instruction::Variable(_)))
        {
            let variable: Variable = Function {
                ident: Some(ident.clone()),
                params,
                body: Body::Lang(body),
                return_type,
            }
            .into();
            local_variables.insert(ident.clone(), LocalVariable::Variable(variable.clone()));
            Ok(Set {
                ident,
                instruction: Instruction::Variable(variable),
            }
            .into())
        } else {
            local_variables.insert(
                ident.clone(),
                LocalVariable::Function(params.clone(), return_type.clone()),
            );
            Ok(Self {
                ident,
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
        let mut local_variables = LocalVariables::from(self.params.clone());
        local_variables.insert(
            self.ident.clone(),
            LocalVariable::Function(self.params.clone(), self.return_type.clone()),
        );
        let body = recreate_instructions(&self.body, &mut local_variables, interpreter)?;
        let function: Rc<Function> = Function {
            ident: Some(self.ident.clone()),
            params: self.params.clone(),
            body: Body::Lang(body),
            return_type: self.return_type.clone(),
        }
        .into();
        interpreter.insert(self.ident.clone(), function.clone().into());
        Ok(function.into())
    }
}

impl Recreate for FunctionDeclaration {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let body = {
            let mut local_variables = local_variables.create_layer();
            local_variables.insert(
                self.ident.clone(),
                LocalVariable::Function(self.params.clone(), self.return_type.clone()),
            );
            let mut local_variables = local_variables.layer_from_map(self.params.clone().into());
            recreate_instructions(&self.body, &mut local_variables, interpreter)
        }?;
        let ident = self.ident.clone();
        let params = self.params.clone();
        let return_type = self.return_type.clone();
        if body
            .iter()
            .all(|instruction| matches!(instruction, Instruction::Variable(_)))
        {
            let variable: Variable = Function {
                ident: Some(ident.clone()),
                params,
                body: Body::Lang(body),
                return_type,
            }
            .into();
            local_variables.insert(ident.clone(), LocalVariable::Variable(variable.clone()));
            Ok(Set {
                ident,
                instruction: Instruction::Variable(variable),
            }
            .into())
        } else {
            local_variables.insert(
                ident.clone(),
                LocalVariable::Function(params.clone(), return_type.clone()),
            );
            Ok(Self {
                ident,
                params,
                body,
                return_type,
            }
            .into())
        }
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
            params,
            return_type,
        }
        .into()
    }
}

impl BaseInstruction for FunctionDeclaration {}
