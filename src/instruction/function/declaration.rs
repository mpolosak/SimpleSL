use crate::{
    function::{Body, Function, Param, Params},
    instruction::{
        local_variable::{FunctionInfo, LocalVariable, LocalVariableMap, LocalVariables},
        recreate_instructions,
        traits::{BaseInstruction, ExecResult, MutCreateInstruction},
        Exec, Instruction, Recreate,
    },
    interpreter::Interpreter,
    parse::Rule,
    variable::{FunctionType, ReturnType, Type},
    Error, ExecError,
};
use pest::iterators::Pair;
use std::rc::Rc;

#[derive(Debug)]
pub struct FunctionDeclaration {
    ident: Rc<str>,
    pub params: Params,
    body: Rc<[Instruction]>,
    return_type: Type,
}

impl MutCreateInstruction for FunctionDeclaration {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let ident: Rc<str> = inner.next().unwrap().as_str().into();
        let mut inner = inner.next().unwrap().into_inner();
        let params_pair = inner.next().unwrap();
        let params = Params(params_pair.into_inner().map(Param::from_pair).collect());
        let return_type = if matches!(inner.peek(), Some(pair)
            if pair.as_rule() == Rule::return_type_decl)
        {
            Type::from_pair(inner.next().unwrap().into_inner().next().unwrap())
        } else {
            Type::Void
        };
        local_variables.insert(
            ident.clone(),
            LocalVariable::Function(params.clone(), return_type.clone()),
        );

        let mut local_variables = local_variables.function_layer(
            LocalVariableMap::from(params.clone()),
            FunctionInfo::new(Some(ident.clone()), return_type.clone()),
        );
        let body = interpreter.create_instructions(inner, &mut local_variables)?;
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
    ) -> Result<Instruction, ExecError> {
        local_variables.insert(
            self.ident.clone(),
            LocalVariable::Function(self.params.clone(), self.return_type.clone()),
        );
        let mut local_variables = local_variables.function_layer(
            self.params.clone().into(),
            FunctionInfo::new(Some(self.ident.clone()), self.return_type.clone()),
        );
        let body = recreate_instructions(&self.body, &mut local_variables, interpreter)?;
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
