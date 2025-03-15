use super::{Instruction, InstructionWithStr, function::AnonymousFunction};
use crate::{
    self as simplesl, Error, Interpreter,
    function::{Param, Params},
    variable::{ReturnType, Type, Typed, Variable},
};
use pest::{Parser, iterators::Pairs};
use simplesl_macros::var_type;
use simplesl_parser::{Rule, SimpleSLParser};
use std::{collections::HashMap, fs, sync::Arc};

pub type LocalVariableMap = HashMap<Arc<str>, LocalVariable>;
pub struct LocalVariables<'a> {
    variables: LocalVariableMap,
    lower_layer: Option<&'a Self>,
    function: Option<FunctionInfo>,
    pub in_loop: bool,
    pub interpreter: &'a Interpreter<'a>,
}

impl<'a> LocalVariables<'a> {
    #[must_use]
    pub fn new(interpreter: &'a Interpreter) -> Self {
        Self {
            variables: LocalVariableMap::new(),
            lower_layer: None,
            function: None,
            interpreter,
            in_loop: false,
        }
    }

    pub fn from_params(params: Params, interpreter: &'a Interpreter) -> Self {
        Self {
            variables: LocalVariableMap::from(params),
            lower_layer: None,
            function: None,
            interpreter,
            in_loop: false,
        }
    }
    pub fn insert(&mut self, name: Arc<str>, variable: LocalVariable) {
        self.variables.insert(name, variable);
    }
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&LocalVariable> {
        self.variables
            .get(name)
            .or_else(|| self.lower_layer?.get(name))
    }
    #[must_use]
    pub fn contains_key(&self, name: &Arc<str>) -> bool {
        self.variables.contains_key(name)
            || self
                .lower_layer
                .is_some_and(|layer| layer.contains_key(name))
    }
    #[must_use]
    pub fn create_layer(&'a self) -> Self {
        Self {
            variables: LocalVariableMap::new(),
            lower_layer: Some(self),
            function: None,
            interpreter: self.interpreter,
            in_loop: self.in_loop,
        }
    }

    #[must_use]
    pub fn function_layer(&'a self, layer: LocalVariableMap, function: FunctionInfo) -> Self {
        Self {
            variables: layer,
            lower_layer: Some(self),
            function: Some(function),
            interpreter: self.interpreter,
            in_loop: false,
        }
    }

    pub fn function(&'a self) -> Option<&'a FunctionInfo> {
        self.function
            .as_ref()
            .or_else(|| self.lower_layer.and_then(LocalVariables::function))
    }

    pub(crate) fn load(&mut self, path: &str) -> Result<Arc<[InstructionWithStr]>, Error> {
        let contents = fs::read_to_string(path)?;
        self.parse_input(&contents)
    }

    pub(crate) fn parse_input(&mut self, input: &str) -> Result<Arc<[InstructionWithStr]>, Error> {
        let pairs = SimpleSLParser::parse(Rule::input, input)?;
        self.create_instructions(pairs)
    }

    pub(crate) fn create_instructions(
        &mut self,
        pairs: Pairs<'_, Rule>,
    ) -> Result<Arc<[InstructionWithStr]>, Error> {
        let mut instructions = pairs
            .map(|pair| InstructionWithStr::new(pair, self))
            .collect::<Result<Vec<InstructionWithStr>, Error>>()?;
        let Some(last) = instructions.pop() else {
            return Ok(Arc::from([]));
        };
        instructions.retain(|instruction| {
            !matches!(
                instruction,
                InstructionWithStr {
                    instruction: Instruction::Variable(..),
                    ..
                }
            )
        });
        instructions.push(last);
        Ok(instructions.into())
    }
}

impl<V> Extend<(Arc<str>, V)> for LocalVariables<'_>
where
    V: Into<LocalVariable>,
{
    fn extend<T: IntoIterator<Item = (Arc<str>, V)>>(&mut self, iter: T) {
        self.variables
            .extend(iter.into_iter().map(|(ident, var)| (ident, var.into())));
    }
}

#[derive(Clone, Debug)]
pub enum LocalVariable {
    Function(Params, Type),
    Variable(Variable),
    Other(Type),
}

impl From<Type> for LocalVariable {
    fn from(value: Type) -> Self {
        Self::Other(value)
    }
}

impl From<&Instruction> for LocalVariable {
    fn from(value: &Instruction) -> Self {
        match value {
            Instruction::AnonymousFunction(function) => function.into(),
            Instruction::LocalVariable(_, var) => var.clone(),
            Instruction::Variable(var) => var.clone().into(),
            ins => ins.return_type().into(),
        }
    }
}

impl From<Variable> for LocalVariable {
    fn from(value: Variable) -> Self {
        Self::Variable(value)
    }
}

impl From<&AnonymousFunction> for LocalVariable {
    fn from(value: &AnonymousFunction) -> Self {
        Self::Function(value.params.clone(), value.return_type())
    }
}

impl Typed for LocalVariable {
    fn as_type(&self) -> Type {
        match self {
            LocalVariable::Function(params, return_type) => {
                let params: Arc<[Type]> = params
                    .iter()
                    .map(|Param { var_type, name: _ }| var_type.clone())
                    .collect();
                let return_type = return_type.clone();
                var_type!(params -> return_type)
            }
            LocalVariable::Variable(variable) => variable.as_type(),
            LocalVariable::Other(var_type) => var_type.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FunctionInfo {
    name: Option<Arc<str>>,
    return_type: Type,
}

impl FunctionInfo {
    pub fn new(name: Option<Arc<str>>, return_type: Type) -> Self {
        Self { name, return_type }
    }

    pub fn name(&self) -> Option<Arc<str>> {
        self.name.clone()
    }

    pub fn return_type(&self) -> &Type {
        &self.return_type
    }
}
