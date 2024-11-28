use crate as simplesl;
use crate::{
    function::{Body, Function, Param, Params},
    instruction::{
        local_variable::{FunctionInfo, LocalVariable, LocalVariableMap, LocalVariables},
        recreate_instructions, Exec, ExecResult, Instruction, InstructionWithStr, Recreate,
    },
    interpreter::Interpreter,
    variable::{ReturnType, Type},
    Error, ExecError,
};
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug)]
pub struct FunctionDeclaration {
    ident: Arc<str>,
    pub params: Params,
    body: Arc<[InstructionWithStr]>,
    return_type: Type,
}

impl FunctionDeclaration {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let ident: Arc<str> = inner.next().unwrap().as_str().into();
        let mut inner = inner.next().unwrap().into_inner();
        let params_pair = inner.next().unwrap();
        let params = Params(params_pair.into_inner().map(Param::from).collect());
        let return_type = if matches!(inner.peek(), Some(pair)
            if pair.as_rule() == Rule::return_type_decl)
        {
            Type::from(inner.next().unwrap().into_inner().next().unwrap())
        } else {
            Type::Void
        };
        local_variables.insert(
            ident.clone(),
            LocalVariable::Function(params.clone(), return_type.clone()),
        );

        local_variables.push_layer(LocalVariableMap::from(params.clone()));
        local_variables.enter_function(FunctionInfo::new(Some(ident.clone()), return_type.clone()));
        let body = local_variables.create_instructions(inner)?;
        local_variables.exit_function();
        local_variables.drop_layer();
        if !Type::Void.matches(&return_type)
            && !body
                .iter()
                .map(ReturnType::return_type)
                .any(|var_type| var_type == Type::Never)
        {
            return Err(Error::MissingReturn {
                function_name: Some(ident),
                return_type,
            });
        }
        Ok(Self {
            ident,
            params,
            body,
            return_type,
        }
        .into())
    }
}

impl Exec for FunctionDeclaration {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let mut local_variables = LocalVariables::from_params(self.params.clone(), interpreter);
        local_variables.insert(
            self.ident.clone(),
            LocalVariable::Function(self.params.clone(), self.return_type.clone()),
        );
        let body = recreate_instructions(&self.body, &mut local_variables)?;
        let function: Arc<Function> = Function {
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
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        local_variables.insert(
            self.ident.clone(),
            LocalVariable::Function(self.params.clone(), self.return_type.clone()),
        );

        local_variables.push_layer(LocalVariableMap::from(self.params.clone()));
        local_variables.enter_function(FunctionInfo::new(
            Some(self.ident.clone()),
            self.return_type.clone(),
        ));
        let body = recreate_instructions(&self.body, local_variables)?;
        local_variables.exit_function();
        local_variables.drop_layer();
        Ok(Self {
            ident: self.ident.clone(),
            params: self.params.clone(),
            body,
            return_type: self.return_type.clone(),
        }
        .into())
    }
}

impl ReturnType for FunctionDeclaration {
    fn return_type(&self) -> Type {
        let params: Arc<[Type]> = self
            .params
            .iter()
            .map(|Param { name: _, var_type }| var_type.clone())
            .collect();
        let return_type = self.return_type.clone();
        var_type!(params -> return_type)
    }
}
