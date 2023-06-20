use super::{Function, LangFunction, Line, Param, Params};
use crate::variable::Variable;
use crate::variable_type::{GetType, Type};
use crate::{
    error::Error,
    interpreter::{Interpreter, VariableMap},
    parse::Rule,
};
use pest::iterators::Pair;
use std::iter::zip;
use std::{collections::HashMap, fmt, rc::Rc};

#[derive(Clone)]
pub enum Instruction {
    FunctionCall {
        function: Rc<dyn Function>,
        args: Vec<Instruction>,
    },
    LocalFunctionCall {
        ident: String,
        args: Vec<Instruction>,
        return_type: Type,
    },
    Variable(Variable),
    LocalVariable(String, Type),
    Array(Vec<Instruction>),
    Function {
        params: Params,
        body: Vec<Line>,
    },
}

impl Instruction {
    pub fn new(
        variables: &VariableMap,
        pair: Pair<Rule>,
        local_variables: &HashMap<String, Type>,
    ) -> Result<Self, Error> {
        match pair.as_rule() {
            Rule::function_call => Self::create_function_call(&pair, variables, local_variables),
            Rule::int => {
                let value = pair.as_str().parse::<i64>().unwrap();
                Ok(Self::Variable(Variable::Int(value)))
            }
            Rule::float => {
                let value = pair.as_str().parse::<f64>().unwrap();
                Ok(Self::Variable(Variable::Float(value)))
            }
            Rule::ident => {
                let var_name = pair.as_str();
                if let Some(var_type) = local_variables.get(var_name) {
                    Ok(Self::LocalVariable(var_name.to_string(), var_type.clone()))
                } else {
                    let value = variables.get(var_name)?;
                    Ok(Self::Variable(value))
                }
            }
            Rule::string => {
                let value = pair.clone().into_inner().next().unwrap().as_str();
                let variable = Variable::String(value.into());
                Ok(Self::Variable(variable))
            }
            Rule::array => {
                let inner = pair.clone().into_inner();
                let array = inner
                    .map(|arg| Self::new(variables, arg, local_variables))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::Array(array))
            }
            Rule::function => Self::create_function(pair, local_variables, variables),
            Rule::null => Ok(Self::Variable(Variable::Null)),
            _ => panic!(),
        }
    }
    pub fn exec(
        &self,
        intepreter: &mut Interpreter,
        local_variables: &VariableMap,
    ) -> Result<Variable, Error> {
        match &self {
            Self::FunctionCall { function, args } => {
                let args = exec_instructions(args, intepreter, local_variables)?;
                function.exec("name", intepreter, args)
            }
            Self::LocalFunctionCall {
                ident,
                args: instructions,
                ..
            } => {
                let args = exec_instructions(instructions, intepreter, local_variables)?;
                let Variable::Function(function) = local_variables.get(ident).or(
                        intepreter.variables.get(ident)).unwrap() else {
                    return Err(error_wrong_type(instructions, ident));
                };
                function.exec(ident, intepreter, args)
            }
            Self::Variable(var) => Ok(var.clone()),
            Self::LocalVariable(name, _) => Ok(local_variables
                .get(name)
                .or(intepreter.variables.get(name))
                .unwrap()),
            Self::Array(instructions) => {
                let array = exec_instructions(instructions, intepreter, local_variables)?;
                Ok(Variable::Array(array.into()))
            }
            Self::Function { params, body } => {
                let mut fn_local_variables = HashMap::from(params);
                let body = recreate_lines(body, &mut fn_local_variables, local_variables)?;
                Ok(Variable::Function(Rc::new(LangFunction {
                    params: params.clone(),
                    body,
                })))
            }
        }
    }
    pub fn recreate(
        &self,
        local_variables: &HashMap<String, Type>,
        args: &VariableMap,
    ) -> Result<Self, Error> {
        Ok(match self {
            Self::LocalFunctionCall {
                ident,
                args: instructions,
                return_type,
            } => {
                let instructions = recreate_instructions(instructions, local_variables, args)?;
                if local_variables.contains_key(ident) {
                    Self::LocalFunctionCall {
                        ident: ident.clone(),
                        args: instructions,
                        return_type: return_type.clone(),
                    }
                } else {
                    let Variable::Function(function)
                        = args.get(ident).unwrap() else {
                        return Err(error_wrong_type(&instructions, ident));
                    };
                    Self::FunctionCall {
                        function,
                        args: instructions,
                    }
                }
            }
            Self::FunctionCall {
                function,
                args: instructions,
            } => {
                let instructions = recreate_instructions(instructions, local_variables, args)?;
                Self::FunctionCall {
                    function: function.clone(),
                    args: instructions,
                }
            }
            Self::LocalVariable(name, var_type) => {
                if local_variables.contains_key(name) {
                    Self::LocalVariable(name.clone(), var_type.clone())
                } else {
                    let variable = args.get(name).unwrap();
                    Self::Variable(variable)
                }
            }
            Self::Array(instructions) => {
                let instructions = recreate_instructions(instructions, local_variables, args)?;
                Self::Array(instructions)
            }
            Self::Function { params, body } => {
                let mut local_variables = local_variables.clone();
                local_variables.extend(HashMap::from(params));
                let body = recreate_lines(body, &mut local_variables, args)?;
                Self::Function {
                    params: params.clone(),
                    body,
                }
            }
            _ => self.clone(),
        })
    }
    fn create_function_call(
        pair: &Pair<'_, Rule>,
        variables: &VariableMap,
        local_variables: &HashMap<String, Type>,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.clone().into_inner();
        let var_name = inner.next().unwrap().as_str();
        let args = inner
            .next()
            .unwrap()
            .into_inner()
            .map(|pair| Self::new(variables, pair, local_variables))
            .collect::<Result<Vec<_>, _>>()?;
        match local_variables.get(var_name) {
            Some(Type::Function { return_type, .. }) => Ok(
                // todo: check if arguments match params
                Self::LocalFunctionCall {
                    ident: String::from(var_name),
                    args,
                    return_type: *return_type.clone(),
                },
            ),
            Some(_) => Err(error_wrong_type(&args, var_name)),
            None => {
                let Variable::Function(function) = variables.get(var_name)? else {
                    return Err(error_wrong_type(&args, var_name));
                };
                let params = function.get_params();
                check_args(var_name, params, &args)?;
                Ok(Self::FunctionCall { function, args })
            }
        }
    }
    fn create_function(
        pair: Pair<'_, Rule>,
        local_variables: &HashMap<String, Type>,
        variables: &VariableMap,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.clone().into_inner();
        let params_pair = inner.next().unwrap();
        let params: Vec<Param> = params_pair.into_inner().map(Param::from).collect();
        let params = Params {
            standard: params,
            catch_rest: None,
        };
        let mut local_variables = local_variables.clone();
        local_variables.extend(HashMap::from(&params));
        let body = inner
            .map(|arg| Line::new(variables, arg, &mut local_variables))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::Function { params, body })
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl GetType for Instruction {
    fn get_type(&self) -> Type {
        match self {
            Instruction::Variable(variable) => variable.get_type(),
            Instruction::Array(_) => Type::Array,
            Instruction::Function { params, body } => {
                let params_types: Vec<Type> = params
                    .standard
                    .iter()
                    .map(|Param { name: _, var_type }| var_type.clone())
                    .collect();
                let catch_rest = params.catch_rest.is_some();
                let return_type = match body.last() {
                    Some(Line {
                        result_var: _,
                        instruction,
                    }) => instruction.get_type(),
                    None => Type::Any,
                };
                Type::Function {
                    return_type: Box::new(return_type),
                    params: params_types,
                    catch_rest,
                }
            }
            Instruction::FunctionCall { function, .. } => function.get_return_type(),
            Instruction::LocalFunctionCall { return_type, .. } => return_type.clone(),
            Instruction::LocalVariable(_, var_type) => var_type.clone(),
        }
    }
}

pub fn check_args(var_name: &str, params: &Params, args: &Vec<Instruction>) -> Result<(), Error> {
    match params.catch_rest {
        Some(_) if args.len() < params.standard.len() => {
            return Err(Error::WrongNumberOfArguments(
                String::from(var_name),
                params.standard.len(),
            ));
        }
        None if args.len() != params.standard.len() => {
            return Err(Error::WrongNumberOfArguments(
                String::from(var_name),
                params.standard.len(),
            ));
        }
        _ => (),
    }

    for (arg, Param { name, var_type }) in zip(args, &params.standard) {
        if !arg.get_type().matches(var_type) {
            return Err(Error::WrongType(name.clone(), var_type.clone()));
        }
    }
    Ok(())
}

pub fn recreate_instructions(
    instructions: &[Instruction],
    local_variables: &HashMap<String, Type>,
    args: &VariableMap,
) -> Result<Vec<Instruction>, Error> {
    instructions
        .iter()
        .map(|instruction| instruction.recreate(local_variables, args))
        .collect()
}

pub fn recreate_lines(
    lines: &[Line],
    local_variables: &mut HashMap<String, Type>,
    args: &VariableMap,
) -> Result<Vec<Line>, Error> {
    lines
        .iter()
        .map(|line| line.recreate(local_variables, args))
        .collect::<Result<Vec<Line>, Error>>()
}

pub fn exec_instructions(
    instructions: &[Instruction],
    intepreter: &mut Interpreter,
    local_variables: &VariableMap,
) -> Result<Vec<Variable>, Error> {
    instructions
        .iter()
        .map(|instruction| instruction.exec(intepreter, local_variables))
        .collect::<Result<Vec<_>, _>>()
}

fn error_wrong_type(args: &[Instruction], var_name: &str) -> Error {
    let params = args.iter().map(Instruction::get_type).collect();
    Error::WrongType(
        var_name.to_owned(),
        Type::Function {
            return_type: Type::Any.into(),
            params,
            catch_rest: false,
        },
    )
}
